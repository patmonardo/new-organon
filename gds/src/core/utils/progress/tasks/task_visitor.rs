//! Visitor pattern for traversing task hierarchies.

use super::{IterativeTask, LeafTask, Task};

/// Visitor for traversing task hierarchies.
///
/// Provides specialized visit methods for different task types.
/// Default implementations delegate to the generic `visit()` method.
pub trait TaskVisitor {
    /// Visit a leaf task (terminal node).
    fn visit_leaf_task(&self, task: &LeafTask) {
        self.visit(task.base());
    }

    /// Visit an intermediate task (has children).
    fn visit_intermediate_task(&self, task: &Task) {
        self.visit(task);
    }

    /// Visit an iterative task (repeating operation).
    fn visit_iterative_task(&self, task: &IterativeTask) {
        self.visit(task.base());
    }

    /// Generic visit method - fallback for all task types.
    fn visit(&self, _task: &Task) {
        // Default implementation does nothing
    }
}

/// Marker trait for types that can be visited (kept for backwards compatibility).
pub trait TaskLike {
    fn description(&self) -> &str;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct CountingVisitor {
        visit_count: RefCell<usize>,
        leaf_count: RefCell<usize>,
    }

    impl TaskVisitor for CountingVisitor {
        fn visit_leaf_task(&self, _task: &LeafTask) {
            *self.leaf_count.borrow_mut() += 1;
        }

        fn visit(&self, _task: &Task) {
            *self.visit_count.borrow_mut() += 1;
        }
    }

    #[test]
    fn test_visitor_delegation() {
        let visitor = CountingVisitor {
            visit_count: RefCell::new(0),
            leaf_count: RefCell::new(0),
        };

        let task = LeafTask::new("test".to_string(), 100);

        visitor.visit_leaf_task(&task);
        assert_eq!(*visitor.leaf_count.borrow(), 1);
    }
}
