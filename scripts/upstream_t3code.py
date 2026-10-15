#!/usr/bin/env python3
"""Compare T3 Code's provider surface with the pinned Local AI Kit revision.

Released builds do not run this script. It diffs the paths recorded in
upstream/t3code/snapshot.json: local model providers, cloud CLI providers,
installed CLI detection, and the ChatGPT OAuth sign-in flow. A difference
can open a review pull request. It never changes adapter behavior.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SNAPSHOT_PATH = ROOT / "upstream" / "t3code" / "snapshot.json"
STATE_PATH = ROOT / "UPSTREAM_T3CODE.md"
SOURCES_PATH = ROOT / "upstream" / "sources.json"
REPOSITORY = "pingdotgg/t3code"
GIT_URL = f"https://github.com/{REPOSITORY}.git"
API = f"https://api.github.com/repos/{REPOSITORY}"
NIGHTLY_TAG = re.compile(r"^v(\d+)\.(\d+)\.(\d+)-nightly\.(\d{8})\.(\d+)$")

# Every daily pass must still be able to see these areas. The snapshot may
# list more paths; it may not drop one of these.
REQUIRED_PATHS = {
    "local-and-cloud-catalog": "apps/server/src/provider/builtInDrivers.ts",
    "cloud-relay": "apps/server/src/cloud",
    "cli-codex": "apps/server/src/provider/CodexInstallation.ts",
    "cli-claude-home": "apps/server/src/provider/Drivers/ClaudeHome.ts",
    "cli-claude-executable": "apps/server/src/provider/Drivers/ClaudeExecutable.ts",
    "cli-opencode": "packages/provider-opencode/src/server/driver.ts",
    "cli-pi": "packages/provider-pi/src/server/usage.ts",
    "cli-grok": "packages/provider-grok/src/server",
    "cli-cursor": "packages/provider-cursor/src/settings.ts",
    "cli-acp": "packages/provider-acp-registry/src/server",
    "chatgpt-oauth": "apps/server/src/provider/CodexChatGptAuth.ts",
    "chatgpt-handoff": "apps/server/src/provider/CodexChatGptHandoff.ts",
}
REQUIRED_AREAS = {
    "local-model-runtimes",
    "cloud-llm-apis",
    "chatgpt-multi-account",
    "acp-registry-install",
    "codex-managed-download",
    "gemini-cli",
    "claude-windows-shim",
}
STATE_MARKERS = (
    "Local model providers",
    "Cloud model providers",
    "Installed CLI and tool detection",
    "ChatGPT OAuth",
)


def load_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def snapshot_commit(snapshot: dict) -> str:
    return snapshot["source"]["commit"]


def watched_paths(snapshot: dict) -> list[str]:
    paths = snapshot.get("watchedPaths") or []
    if not isinstance(paths, list) or not all(isinstance(item, str) and item for item in paths):
        raise SystemExit("snapshot watchedPaths must be a list of relative paths")
    return paths


def validate_offline() -> int:
    snapshot = load_json(SNAPSHOT_PATH)
    sources = load_json(SOURCES_PATH)
    state = STATE_PATH.read_text(encoding="utf-8")
    errors: list[str] = []
    source = snapshot.get("source") or {}
    commit = source.get("commit", "")
    if source.get("repository") != REPOSITORY:
        errors.append("snapshot repository is not pingdotgg/t3code")
    if not re.fullmatch(r"[0-9a-f]{40}", commit):
        errors.append("snapshot commit is not a full SHA")
    if commit not in state:
        errors.append("UPSTREAM_T3CODE.md does not record the snapshot commit")
    nightly = source.get("nightlyCommit", "")
    if nightly and nightly not in state:
        errors.append("UPSTREAM_T3CODE.md does not record the nightly commit")
    for marker in STATE_MARKERS:
        if marker not in state:
            errors.append(f"UPSTREAM_T3CODE.md is missing the {marker} section")
    paths = set(watched_paths(snapshot))
    for area, path in REQUIRED_PATHS.items():
        if path not in paths:
            errors.append(f"watched paths dropped {area}: {path}")
    areas = {item.get("area") for item in snapshot.get("reviewedNotAdopted", [])}
    missing = REQUIRED_AREAS - areas
    if missing:
        errors.append("reviewedNotAdopted is missing " + ", ".join(sorted(missing)))
    source_ids = [item.get("id") for item in sources.get("sources", [])]
    if "t3code" not in source_ids:
        errors.append("sources.json does not list t3code")
    if errors:
        print("\n".join(errors))
        return 1
    print(
        f"Pinned T3 Code {commit} records local model providers, cloud CLI "
        "providers, installed CLI detection, and ChatGPT OAuth. No upstream "
        "fetch was performed."
    )
    return 0


def nightly_sort_key(tag: str) -> tuple[int, ...] | None:
    match = NIGHTLY_TAG.fullmatch(tag)
    if not match:
        return None
    major, minor, patch, day, sequence = (int(part) for part in match.groups())
    return (day, sequence, major, minor, patch)


def newest_nightly(tags: list[str]) -> str | None:
    ranked = [(nightly_sort_key(tag), tag) for tag in tags]
    ranked = [(key, tag) for key, tag in ranked if key is not None]
    if not ranked:
        return None
    return max(ranked)[1]


def diff_is_relevant(text: str) -> bool:
    return bool(text.strip())


def format_report(
    pinned: str,
    main_commit: str,
    main_diff: str,
    nightly_tag: str,
    nightly_base: str,
    nightly_commit: str,
    nightly_diff: str,
) -> str:
    def section(title: str, body: str) -> str:
        if not diff_is_relevant(body):
            return f"## {title}\n\nNo watched-path changes.\n"
        clipped = body if len(body) <= 12000 else body[:12000] + "\n… diff truncated …\n"
        return f"## {title}\n\n```diff\n{clipped.rstrip()}\n```\n"

    return (
        f"# T3 Code provider review\n\n"
        f"Pinned commit: `{pinned}`\n\n"
        f"Compared `main` `{main_commit}` and nightly `{nightly_tag}` "
        f"(`{nightly_commit}`) against nightly pin `{nightly_base}`.\n\n"
        "Watched areas: local model providers, cloud CLI providers, installed "
        "CLI detection, and ChatGPT OAuth. This report does not change Local "
        "AI Kit behavior.\n\n"
        + section(f"main ({pinned[:12]}..{main_commit[:12]})", main_diff)
        + "\n"
        + section(f"nightly ({nightly_tag})", nightly_diff)
        + "\nDo not move the pin until a maintainer ports a fact and updates "
        "the snapshot in the same change.\n"
    )


def github_json(path: str):
    import urllib.request

    request = urllib.request.Request(
        API + path,
        headers={
            "Accept": "application/vnd.github+json",
            "User-Agent": "local-ai-kit-upstream-t3code",
            **(
                {"Authorization": f"Bearer {token}"}
                if (token := os.environ.get("GITHUB_TOKEN") or os.environ.get("GH_TOKEN"))
                else {}
            ),
        },
    )
    with urllib.request.urlopen(request, timeout=60) as response:
        return json.loads(response.read().decode("utf-8"))


def tip_commit() -> str:
    return github_json("/commits/main")["sha"]


def list_nightly_tags() -> list[str]:
    # Tags come from git so a paged releases response cannot hide a nightly.
    completed = run(["git", "ls-remote", "--tags", GIT_URL])
    tags = []
    for line in completed.stdout.splitlines():
        _sha, ref = line.split("\t", 1)
        name = ref.removeprefix("refs/tags/")
        if name.endswith("^{}"):
            continue
        if NIGHTLY_TAG.fullmatch(name):
            tags.append(name)
    return tags


def commit_for_tag(tag: str) -> str:
    completed = run(["git", "ls-remote", GIT_URL, f"refs/tags/{tag}"])
    lines = [line for line in completed.stdout.splitlines() if line.strip()]
    if not lines:
        raise SystemExit(f"nightly tag {tag} was not found")
    # An annotated tag lists the tag object, then the peeled commit as tag^{}.
    peeled = [line for line in lines if line.endswith("^{}")]
    chosen = peeled[-1] if peeled else lines[-1]
    return chosen.split("\t", 1)[0]


def run(command: list[str], cwd: Path | None = None) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        command,
        cwd=cwd,
        check=True,
        text=True,
        capture_output=True,
    )


def materialize(commit: str, dest: Path, paths: list[str]) -> None:
    if dest.exists():
        shutil.rmtree(dest)
    dest.mkdir(parents=True)
    token = os.environ.get("GITHUB_TOKEN") or os.environ.get("GH_TOKEN")
    url = f"https://x-access-token:{token}@github.com/{REPOSITORY}.git" if token else GIT_URL
    run(["git", "init", str(dest)])
    run(["git", "remote", "add", "origin", url], dest)
    run(["git", "sparse-checkout", "init", "--cone"], dest)
    run(["git", "sparse-checkout", "set", *dict.fromkeys(paths)], dest)
    run(["git", "fetch", "--filter=blob:none", "--depth", "1", "origin", commit], dest)
    run(["git", "checkout", "--detach", "FETCH_HEAD"], dest)


def diff_commits(repo: Path, base: str, tip: str, paths: list[str]) -> str:
    run(["git", "fetch", "--filter=blob:none", "origin", base, tip], repo)
    completed = subprocess.run(
        ["git", "diff", "--stat", "--patch", f"{base}..{tip}", "--", *paths],
        cwd=repo,
        check=True,
        text=True,
        capture_output=True,
    )
    return completed.stdout


def set_output(key: str, value: str) -> None:
    path = os.environ.get("GITHUB_OUTPUT")
    if not path:
        return
    with open(path, "a", encoding="utf-8") as handle:
        handle.write(f"{key}={value}\n")


def append_summary(text: str) -> None:
    path = os.environ.get("GITHUB_STEP_SUMMARY")
    if not path:
        return
    with open(path, "a", encoding="utf-8") as handle:
        handle.write(text)
        if not text.endswith("\n"):
            handle.write("\n")


def redact(text: str) -> str:
    secret = os.environ.get("GITHUB_TOKEN") or os.environ.get("GH_TOKEN") or ""
    if secret and text:
        return text.replace(secret, "***")
    return text


def open_review_pr(report: str, main_commit: str) -> str:
    branch = "upstream/t3code-provider-review"
    run(["git", "fetch", "origin", "main"])
    existing = subprocess.run(
        ["git", "rev-parse", "--verify", f"origin/{branch}"],
        cwd=ROOT,
        text=True,
        capture_output=True,
    )
    if existing.returncode == 0:
        run(["git", "checkout", "-B", branch, f"origin/{branch}"])
        merge = subprocess.run(
            ["git", "merge", "origin/main", "--no-edit"],
            cwd=ROOT,
            text=True,
            capture_output=True,
        )
        if merge.returncode != 0:
            raise SystemExit(redact(merge.stderr or "could not update the review branch with main"))
    else:
        run(["git", "checkout", "-B", branch, "origin/main"])
    review = ROOT / "upstream" / "t3code" / "reviews" / "latest"
    review.mkdir(parents=True, exist_ok=True)
    (review / "report.md").write_text(report, encoding="utf-8")
    run(["git", "add", "--", "upstream/t3code/reviews/latest"])
    status = subprocess.run(["git", "diff", "--cached", "--quiet"], cwd=ROOT, check=False)
    if status.returncode != 0:
        message = (
            "chore(upstream): report T3 Code provider changes\n\n"
            f"Review {main_commit} against the pinned provider surface. "
            "This commit does not change runtime discovery or ChatGPT auth."
        )
        subprocess.run(["git", "commit", "-m", message], cwd=ROOT, check=True)
    run(["git", "push", "-u", "origin", branch])
    title = "Review T3 Code provider changes"
    body = (
        "## Summary\n\n"
        "- T3 Code `main` or a newer nightly changed a watched provider path.\n"
        "- The watched surface is local model providers, cloud CLI providers, "
        "installed CLI detection, and ChatGPT OAuth.\n"
        "- Production behavior is unchanged by this pull request.\n\n"
        "## Upstream diff\n\n"
        "```text\n"
        + report[:12000]
        + "\n```\n\n"
        "## Test plan\n\n"
        "- [ ] Confirm the diff is in local models, cloud CLIs, installed CLI detection, or ChatGPT OAuth\n"
        "- [ ] Decide which facts to adopt without copying T3 Code source\n"
        "- [ ] Update `upstream/t3code/snapshot.json` and `UPSTREAM_T3CODE.md` only with the adopted pin\n"
        "- [ ] Re-run `python scripts/upstream_t3code.py --offline`\n"
    )
    listed = subprocess.run(
        ["gh", "pr", "list", "--head", branch, "--json", "number,url", "--jq", ".[0].url // \"\""],
        cwd=ROOT,
        text=True,
        capture_output=True,
        check=True,
    )
    url = listed.stdout.strip()
    if url:
        subprocess.run(["gh", "pr", "edit", url, "--title", title, "--body", body], cwd=ROOT, check=True)
        return url
    created = subprocess.run(
        ["gh", "pr", "create", "--base", "main", "--head", branch, "--title", title, "--body", body],
        cwd=ROOT,
        text=True,
        capture_output=True,
        check=True,
    )
    return created.stdout.strip()


def compare(open_pr: bool) -> int:
    snapshot = load_json(SNAPSHOT_PATH)
    pinned = snapshot_commit(snapshot)
    nightly_base = snapshot["source"]["nightlyCommit"]
    paths = watched_paths(snapshot)
    main_commit = tip_commit()
    tag = newest_nightly(list_nightly_tags())
    if tag is None:
        raise SystemExit("T3 Code has no nightly tags")
    nightly_commit = commit_for_tag(tag)
    set_output("upstream_sha", main_commit)
    set_output("nightly_tag", tag)
    needs_main = main_commit != pinned
    needs_nightly = nightly_commit != nightly_base
    if not needs_main and not needs_nightly:
        message = f"T3 Code main and nightly {tag} match the pin."
        print(message)
        append_summary(message + "\n")
        set_output("relevant", "false")
        return 0
    with tempfile.TemporaryDirectory() as temporary:
        dest = Path(temporary) / "t3code"
        materialize(main_commit, dest, paths)
        main_diff = diff_commits(dest, pinned, main_commit, paths) if needs_main else ""
        nightly_diff = (
            diff_commits(dest, nightly_base, nightly_commit, paths) if needs_nightly else ""
        )
    relevant = diff_is_relevant(main_diff) or diff_is_relevant(nightly_diff)
    if not relevant:
        message = (
            f"T3 Code {pinned[:12]}..{main_commit[:12]} and nightly {tag} "
            "have no watched provider changes."
        )
        print(message)
        append_summary(message + "\n")
        set_output("relevant", "false")
        return 0
    report = format_report(
        pinned, main_commit, main_diff, tag, nightly_base, nightly_commit, nightly_diff
    )
    print(report)
    append_summary("```text\n" + report[:8000] + "\n```\n")
    set_output("relevant", "true")
    if open_pr:
        url = open_review_pr(report, main_commit)
        print(f"Review pull request: {url}")
        set_output("pull_request", url)
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--offline",
        action="store_true",
        help="validate the pin, watched areas, and sync state without contacting GitHub",
    )
    parser.add_argument(
        "--open-pr",
        action="store_true",
        help="when watched paths differ, push a review branch and open or update a pull request",
    )
    args = parser.parse_args(argv)
    if args.offline:
        return validate_offline()
    return compare(args.open_pr)


if __name__ == "__main__":
    sys.exit(main())
