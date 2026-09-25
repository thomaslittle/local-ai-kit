//! Injected operating-system boundaries. Importing this crate performs no I/O.
mod native;
mod paths;
use lai_contracts::Result;
pub use paths::child_path;
use std::path::{Path, PathBuf};
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryKind {
    Missing,
    File,
    Directory,
    Symlink,
    Other,
}
#[derive(Debug, Clone)]
pub struct DirectoryListing {
    pub entries: Vec<PathBuf>,
    pub truncated: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileIdentity {
    pub volume: u128,
    pub file: u128,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadChunk {
    pub bytes: Vec<u8>,
    pub next_offset: u64,
    pub total_size: u64,
    pub truncated: bool,
    pub identity: Option<FileIdentity>,
}
pub trait LocalFilesystem: Send + Sync {
    fn probe(&self, path: &Path) -> Result<EntryKind>;
    fn children(&self, path: &Path, max_entries: usize) -> Result<DirectoryListing>;
    fn read_chunk(&self, path: &Path, offset: u64, max_bytes: usize) -> Result<ReadChunk>;
}
pub struct NativeFilesystem;

pub mod watcher;
