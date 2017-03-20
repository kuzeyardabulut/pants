
use std::path::PathBuf;

use graph::Graph;
use tasks::Tasks;
use types::Types;
use fs::{PosixFS, Snapshots};


/**
 * The core context shared (via Arc) between the Scheduler and the Context objects of
 * all running Nodes.
 *
 * TODO: Move `nodes.Context` to this module and rename both of these.
 */
pub struct Core {
  pub graph: Graph,
  pub tasks: Tasks,
  pub types: Types,
  pub snapshots: Snapshots,
  pub vfs: PosixFS,
}

impl Core {
  pub fn new(
    tasks: Tasks,
    types: Types,
    build_root: PathBuf,
    ignore_patterns: Vec<String>,
  ) -> Core {
    Core {
      graph: Graph::new(),
      tasks: tasks,
      types: types,
      snapshots: Snapshots::new()
        .unwrap_or_else(|e| {
          panic!("Could not initialize Snapshot directory: {:?}", e);
        }),
      // FIXME: Errors in initialization should definitely be exposed as python
      // exceptions, rather than as panics.
      vfs:
        PosixFS::new(build_root, ignore_patterns)
        .unwrap_or_else(|e| {
          panic!("Could not initialize VFS: {:?}", e);
        }),
    }
  }

  /**
   * Reinitializes a Core in a new process (basically, recreates its CpuPool).
   */
  pub fn post_fork(&self) {
    // Reinitialize the VFS pool.
    self.vfs.post_fork();
  }
}