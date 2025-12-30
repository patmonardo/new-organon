//! Depth-aware visitor for task hierarchies.

use super::TaskVisitor;

/// Abstract visitor that tracks traversal depth in task hierarchy.
///
/// Useful for indented rendering, depth-limited operations, etc.
pub trait DepthAwareTaskVisitor: TaskVisitor {
    /// Set current depth in the hierarchy.
    fn set_depth(&mut self, depth: usize);

    /// Get current traversal depth.
    fn depth(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::utils::progress::tasks::Task;
    use std::cell::RefCell;

    struct TestDepthVisitor {
        depth: RefCell<usize>,
        max_depth_seen: RefCell<usize>,
    }

    impl TaskVisitor for TestDepthVisitor {
        fn visit(&self, _task: &Task) {
            let current_depth = *self.depth.borrow();
            let mut max_depth = self.max_depth_seen.borrow_mut();
            if current_depth > *max_depth {
                *max_depth = current_depth;
            }
        }
    }

    impl DepthAwareTaskVisitor for TestDepthVisitor {
        fn set_depth(&mut self, depth: usize) {
            *self.depth.borrow_mut() = depth;
        }

        fn depth(&self) -> usize {
            *self.depth.borrow()
        }
    }

    #[test]
    fn test_depth_tracking() {
        let mut visitor = TestDepthVisitor {
            depth: RefCell::new(0),
            max_depth_seen: RefCell::new(0),
        };

        let task = Task::new("test".to_string(), vec![]);

        visitor.set_depth(0);
        visitor.visit(&task);
        assert_eq!(*visitor.max_depth_seen.borrow(), 0);

        visitor.set_depth(2);
        visitor.visit(&task);
        assert_eq!(*visitor.max_depth_seen.borrow(), 2);
        assert_eq!(visitor.depth(), 2);
    }
}
