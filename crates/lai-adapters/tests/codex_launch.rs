use lai_adapters::codex::resolve_executable;
#[test]
fn direct_native_executable_is_preserved() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp
        .path()
        .join(if cfg!(windows) { "codex.exe" } else { "codex" });
    std::fs::write(&path, []).unwrap();
    assert_eq!(resolve_executable(&path).unwrap(), path);
}
#[test]
fn known_windows_npm_wrapper_resolves_to_packaged_native_binary() {
    let temp = tempfile::tempdir().unwrap();
    let wrapper = temp.path().join("codex.cmd");
    std::fs::write(&wrapper, "must not execute this shell wrapper").unwrap();
    let arch = if cfg!(target_arch = "aarch64") {
        ("codex-win32-arm64", "aarch64-pc-windows-msvc")
    } else {
        ("codex-win32-x64", "x86_64-pc-windows-msvc")
    };
    let bin = temp.path().join(format!(
        "node_modules/@openai/codex/node_modules/@openai/{}/vendor/{}/bin/codex.exe",
        arch.0, arch.1
    ));
    std::fs::create_dir_all(bin.parent().unwrap()).unwrap();
    std::fs::write(&bin, []).unwrap();
    assert_eq!(resolve_executable(&wrapper).unwrap(), bin);
}
#[test]
fn arbitrary_wrapper_and_absent_binary_are_not_executed() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("arbitrary.cmd");
    std::fs::write(&path, "malicious").unwrap();
    assert!(resolve_executable(&path).is_err());
    assert!(resolve_executable(&temp.path().join("codex.cmd")).is_err());
    assert!(resolve_executable(std::path::Path::new("codex")).is_err());
}

#[cfg(windows)]
#[test]
fn extensionless_npm_shim_uses_the_native_package_not_a_shell() {
    let temp = tempfile::tempdir().unwrap();
    let shim = temp.path().join("codex");
    std::fs::write(&shim, "#!/bin/sh\nDO-NOT-EXECUTE").unwrap();
    let (package, target) = if cfg!(target_arch = "aarch64") {
        ("codex-win32-arm64", "aarch64-pc-windows-msvc")
    } else {
        ("codex-win32-x64", "x86_64-pc-windows-msvc")
    };
    let bin = temp.path().join(format!(
        "node_modules/@openai/codex/node_modules/@openai/{package}/vendor/{target}/bin/codex.exe"
    ));
    std::fs::create_dir_all(bin.parent().unwrap()).unwrap();
    std::fs::write(&bin, []).unwrap();
    assert_eq!(resolve_executable(&shim).unwrap(), bin);
}
