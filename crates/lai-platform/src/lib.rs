//! Injected operating-system boundaries. Importing this crate performs no I/O.
use std::path::{Path, PathBuf};
use lai_contracts::Result;
#[derive(Debug,Clone,PartialEq,Eq)]
pub enum EntryKind { Missing, File, Directory, Symlink, Other }
#[derive(Debug,Clone)]
pub struct DirectoryListing { pub entries: Vec<PathBuf>, pub truncated: bool }
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct ReadChunk { pub bytes: Vec<u8>, pub next_offset: u64, pub total_size: u64, pub truncated: bool }
pub trait LocalFilesystem: Send + Sync {
 fn probe(&self, path: &Path) -> Result<EntryKind>;
 fn children(&self, path: &Path, max_entries: usize) -> Result<DirectoryListing>;
 fn read_chunk(&self, path: &Path, offset: u64, max_bytes: usize) -> Result<ReadChunk>;
}
pub struct NativeFilesystem;
impl LocalFilesystem for NativeFilesystem {
 fn probe(&self, _: &Path) -> Result<EntryKind> { unimplemented!("test-first filesystem probe") }
 fn children(&self, _: &Path, _: usize) -> Result<DirectoryListing> { unimplemented!("test-first directory listing") }
 fn read_chunk(&self, _: &Path, _: u64, _: usize) -> Result<ReadChunk> { unimplemented!("test-first bounded reader") }
}
pub fn child_path(_: &Path, _: &str) -> Result<PathBuf> { unimplemented!("test-first safe child") }
