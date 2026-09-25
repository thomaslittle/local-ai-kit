use lai_contracts::Result;
use lai_discovery::log_tail::{LogTailState, read_lines};
use lai_platform::{DirectoryListing, EntryKind, LocalFilesystem, NativeFilesystem, ReadChunk};
use std::{
    fs,
    io::{Seek, SeekFrom, Write},
    path::Path,
    sync::atomic::{AtomicUsize, Ordering},
};

#[test]
fn append_and_partial_records_are_incremental() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("events.jsonl");
    fs::write(&path, b"one\npar").unwrap();
    let mut state = LogTailState::default();
    let first = read_lines(&NativeFilesystem, &path, &mut state, 1024, 20).unwrap();
    assert_eq!(first.lines, vec![b"one".to_vec()]);
    assert_eq!(state.pending_bytes(), 3);
    assert!(!first.rotated && !first.truncated);
    fs::OpenOptions::new()
        .append(true)
        .open(&path)
        .unwrap()
        .write_all(b"tial\ntwo\r\n")
        .unwrap();
    let second = read_lines(&NativeFilesystem, &path, &mut state, 1024, 20).unwrap();
    assert_eq!(second.lines, vec![b"partial".to_vec(), b"two".to_vec()]);
    assert_eq!(state.pending_bytes(), 0);
}
#[test]
fn truncation_restarts_without_replaying_old_records() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("events");
    fs::write(&path, b"a\nb\n").unwrap();
    let mut state = LogTailState::default();
    assert_eq!(
        read_lines(&NativeFilesystem, &path, &mut state, 1024, 20)
            .unwrap()
            .lines
            .len(),
        2
    );
    fs::write(&path, b"z\n").unwrap();
    let next = read_lines(&NativeFilesystem, &path, &mut state, 1024, 20).unwrap();
    assert!(next.truncated);
    assert!(!next.rotated);
    assert_eq!(next.lines, vec![b"z".to_vec()]);
}
#[test]
fn replacement_rotation_is_detected_even_when_new_file_is_larger() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("events");
    fs::write(&path, b"old\n").unwrap();
    let mut state = LogTailState::default();
    read_lines(&NativeFilesystem, &path, &mut state, 1024, 20).unwrap();
    fs::rename(&path, tmp.path().join("events.1")).unwrap();
    fs::write(&path, b"new-long\n").unwrap();
    let next = read_lines(&NativeFilesystem, &path, &mut state, 1024, 20).unwrap();
    assert!(next.rotated);
    assert!(!next.truncated);
    assert_eq!(next.lines, vec![b"new-long".to_vec()]);
}
#[test]
fn replacement_with_same_anchor_bytes_is_detected_by_file_identity() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("events");
    let suffix = b"k\n".repeat(32);
    let original = [b"a\n".repeat(32), suffix.clone()].concat();
    fs::write(&path, original).unwrap();
    let mut state = LogTailState::default();
    read_lines(&NativeFilesystem, &path, &mut state, 1024, 100).unwrap();
    fs::remove_file(&path).unwrap();
    let replacement = [b"b\n".repeat(32), suffix, b"new\n".to_vec()].concat();
    fs::write(&path, replacement).unwrap();
    let batch = read_lines(&NativeFilesystem, &path, &mut state, 1024, 100).unwrap();
    assert!(batch.rotated);
    assert_eq!(batch.lines.first().unwrap(), b"b");
    assert_eq!(batch.lines.last().unwrap(), b"new");
}
#[test]
fn line_limit_buffers_complete_records_without_duplicate_file_reads() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("events");
    fs::write(&path, b"a\nb\nc\n").unwrap();
    let mut state = LogTailState::default();
    let a = read_lines(&NativeFilesystem, &path, &mut state, 1024, 1).unwrap();
    assert_eq!(a.lines, vec![b"a".to_vec()]);
    assert!(a.has_more);
    let b = read_lines(&NativeFilesystem, &path, &mut state, 1024, 1).unwrap();
    assert_eq!(b.lines, vec![b"b".to_vec()]);
    assert!(b.has_more);
    let c = read_lines(&NativeFilesystem, &path, &mut state, 1024, 1).unwrap();
    assert_eq!(c.lines, vec![b"c".to_vec()]);
    assert!(!c.has_more);
}
struct CountingFs {
    bytes: AtomicUsize,
}
impl LocalFilesystem for CountingFs {
    fn probe(&self, p: &Path) -> Result<EntryKind> {
        NativeFilesystem.probe(p)
    }
    fn children(&self, p: &Path, n: usize) -> Result<DirectoryListing> {
        NativeFilesystem.children(p, n)
    }
    fn read_chunk(&self, p: &Path, o: u64, n: usize) -> Result<ReadChunk> {
        let c = NativeFilesystem.read_chunk(p, o, n)?;
        self.bytes.fetch_add(c.bytes.len(), Ordering::SeqCst);
        Ok(c)
    }
}
#[test]
fn sparse_gigabyte_log_reads_only_the_bounded_window() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("huge");
    let mut f = fs::File::create(&path).unwrap();
    f.set_len(1_073_741_824).unwrap();
    f.seek(SeekFrom::Start(1_073_741_820)).unwrap();
    f.write_all(b"x\ny\n").unwrap();
    drop(f);
    let fs = CountingFs {
        bytes: AtomicUsize::new(0),
    };
    let mut state = LogTailState::default();
    let batch = read_lines(&fs, &path, &mut state, 4096, 32).unwrap();
    assert!(batch.lines.is_empty());
    assert!(batch.has_more);
    assert!(fs.bytes.load(Ordering::SeqCst) < 5000);
}
#[test]
fn cancellation_is_checked_before_any_file_read() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("events");
    fs::write(&path, b"one\n").unwrap();
    let fs = CountingFs {
        bytes: AtomicUsize::new(0),
    };
    let mut state = LogTailState::default();
    let error =
        lai_discovery::log_tail::read_lines_with_cancel(&fs, &path, &mut state, 1024, 20, || true)
            .unwrap_err();
    assert_eq!(error.code, lai_contracts::ErrorCode::Cancelled);
    assert_eq!(fs.bytes.load(Ordering::SeqCst), 0);
    assert_eq!(state.offset, 0);
}
#[test]
fn cancellation_during_a_poll_preserves_the_callers_saved_offset_and_buffer() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("events");
    fs::write(&path, b"one\npartial").unwrap();
    let fs = CountingFs {
        bytes: AtomicUsize::new(0),
    };
    let mut state = LogTailState::default();
    let mut checks = 0;
    let error =
        lai_discovery::log_tail::read_lines_with_cancel(&fs, &path, &mut state, 1024, 20, || {
            checks += 1;
            checks == 3
        })
        .unwrap_err();
    assert_eq!(error.code, lai_contracts::ErrorCode::Cancelled);
    assert_eq!(state.offset, 0);
    assert_eq!(state.pending_bytes(), 0);
}
#[cfg(unix)]
#[test]
fn tailer_rejects_symlink_paths() {
    let tmp = tempfile::tempdir().unwrap();
    let target = tmp.path().join("target");
    let link = tmp.path().join("events");
    fs::write(&target, b"private\n").unwrap();
    std::os::unix::fs::symlink(&target, &link).unwrap();
    let mut state = LogTailState::default();
    assert!(read_lines(&NativeFilesystem, &link, &mut state, 1024, 20).is_err());
    assert_eq!(state.offset, 0);
}
#[cfg(unix)]
#[test]
fn tailer_rejects_paths_that_traverse_a_symlinked_directory() {
    let tmp = tempfile::tempdir().unwrap();
    let target_dir = tmp.path().join("real");
    let linked_dir = tmp.path().join("linked");
    fs::create_dir(&target_dir).unwrap();
    fs::write(target_dir.join("events"), b"private\n").unwrap();
    std::os::unix::fs::symlink(&target_dir, &linked_dir).unwrap();
    let mut state = LogTailState::default();
    assert!(
        read_lines(
            &NativeFilesystem,
            &linked_dir.join("events"),
            &mut state,
            1024,
            20
        )
        .is_err()
    );
    assert_eq!(state.offset, 0);
}
#[test]
fn invalid_budgets_and_oversized_single_records_fail_closed() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("events");
    fs::write(&path, vec![b'x'; 1024 * 1024 + 1]).unwrap();
    let mut state = LogTailState::default();
    assert!(read_lines(&NativeFilesystem, &path, &mut state, 0, 1).is_err());
    assert!(read_lines(&NativeFilesystem, &path, &mut state, 1024, 0).is_err());
    let first = read_lines(&NativeFilesystem, &path, &mut state, 1024 * 1024, 1).unwrap();
    assert!(first.lines.is_empty());
    assert!(read_lines(&NativeFilesystem, &path, &mut state, 1, 1).is_err());
}
