#![cfg_attr(feature="clippy", allow(redundant_closure))]

use std;

error_chain! {
  foreign_links {
    std::io::Error, IO;
  }

  errors {
    CreateDirFailed(path: String)
    ReadDirFailed(path: String)
    ReadDirItemFailed(path: String)
    RemoveDirAllFailed(path: String)
    RemoveFileFailed(path: String)
    MoveOneFileFailed { from: String, to: String }
    MoveFilesFailed { from: String, to: String }
    CopyRecursivelyFailed { from: String, to: String }
    CopyFileFailed { from: String, to: String }
    ReadFileFailed(path: String)
    RenameFileFailed { from: String, to: String }
  }
}