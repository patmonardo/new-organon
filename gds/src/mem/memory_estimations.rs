//! Java-parity helpers for composing `MemoryEstimation` instances.
//!
//! This is the Rust equivalent of Java's `MemoryEstimations` utility class:
//! it provides small combinators and a builder for constructing hierarchical
//! `MemoryEstimation` implementations without hand-writing many structs.

use std::sync::Arc;

use crate::core::graph_dimensions::GraphDimensions;

use super::{MemoryEstimation, MemoryRange, MemoryTree};

pub struct MemoryEstimations;

impl MemoryEstimations {
    pub const RESIDENT_MEMORY: &'static str = "residentMemory";
    pub const TEMPORARY_MEMORY: &'static str = "temporaryMemory";

    pub fn empty() -> Box<dyn MemoryEstimation> {
        Box::new(NullEstimation)
    }

    pub fn of_range(
        description: impl Into<String>,
        range: MemoryRange,
    ) -> Box<dyn MemoryEstimation> {
        let description = description.into();
        Box::new(LeafEstimation::new(
            description,
            move |_dim, _concurrency| range,
        ))
    }

    pub fn of_resident<F>(description: impl Into<String>, resident: F) -> Box<dyn MemoryEstimation>
    where
        F: Fn(&dyn GraphDimensions, usize) -> MemoryRange + Send + Sync + 'static,
    {
        Box::new(LeafEstimation::new(description.into(), resident))
    }

    pub fn delegate_estimation(
        delegate: Box<dyn MemoryEstimation>,
        description: impl Into<String>,
    ) -> Box<dyn MemoryEstimation> {
        Box::new(DelegateEstimation {
            description: description.into(),
            delegate,
        })
    }

    pub fn setup<F>(description: impl Into<String>, setup: F) -> Box<dyn MemoryEstimation>
    where
        F: Fn(&dyn GraphDimensions, usize) -> Box<dyn MemoryEstimation> + Send + Sync + 'static,
    {
        Box::new(SetupEstimation {
            description: description.into(),
            setup: Arc::new(setup),
        })
    }

    pub fn and_then<F>(
        description: impl Into<String>,
        delegate: Box<dyn MemoryEstimation>,
        and_then: F,
    ) -> Box<dyn MemoryEstimation>
    where
        F: Fn(MemoryRange, &dyn GraphDimensions, usize) -> MemoryRange + Send + Sync + 'static,
    {
        Box::new(AndThenEstimation {
            description: description.into(),
            delegate,
            and_then: Arc::new(and_then),
        })
    }

    pub fn per_node(
        description: impl Into<String>,
        delegate: Box<dyn MemoryEstimation>,
    ) -> Box<dyn MemoryEstimation> {
        Self::and_then(description, delegate, |range, dim, _conc| {
            range.times(dim.node_count())
        })
    }

    pub fn per_thread(
        description: impl Into<String>,
        delegate: Box<dyn MemoryEstimation>,
    ) -> Box<dyn MemoryEstimation> {
        Self::and_then(description, delegate, |range, _dim, conc| range.times(conc))
    }

    pub fn max_estimation(components: Vec<Box<dyn MemoryEstimation>>) -> Box<dyn MemoryEstimation> {
        let description = if components.len() == 2 {
            format!(
                "max of {} and {}",
                components[0].description(),
                components[1].description()
            )
        } else {
            "max".to_string()
        };

        Self::max_estimation_named(description, components)
    }

    pub fn max_estimation_named(
        description: impl Into<String>,
        components: Vec<Box<dyn MemoryEstimation>>,
    ) -> Box<dyn MemoryEstimation> {
        Box::new(MaxEstimation {
            description: description.into(),
            components,
        })
    }

    pub fn builder(description: impl Into<String>) -> Builder {
        Builder::new(description.into())
    }
}

#[derive(Default)]
pub struct Builder {
    stack: Vec<Frame>,
}

#[derive(Default)]
struct Frame {
    description: String,
    components: Vec<Box<dyn MemoryEstimation>>,
}

impl Builder {
    pub fn new(description: String) -> Self {
        Self {
            stack: vec![Frame {
                description,
                components: Vec::new(),
            }],
        }
    }

    fn current_mut(&mut self) -> &mut Frame {
        self.stack
            .last_mut()
            .expect("MemoryEstimations::Builder stack must not be empty")
    }

    pub fn start_field(mut self, description: impl Into<String>) -> Self {
        self.stack.push(Frame {
            description: description.into(),
            components: Vec::new(),
        });
        self
    }

    pub fn end_field(mut self) -> Self {
        if self.stack.len() <= 1 {
            panic!("Cannot end field from root builder");
        }

        let frame = self.stack.pop().expect("stack len checked");
        let estimation = Box::new(CompositeEstimation {
            description: frame.description,
            components: frame.components,
        });

        self.current_mut().components.push(estimation);
        self
    }

    pub fn add(mut self, estimation: Box<dyn MemoryEstimation>) -> Self {
        self.current_mut().components.push(estimation);
        self
    }

    pub fn add_as(
        mut self,
        description: impl Into<String>,
        estimation: Box<dyn MemoryEstimation>,
    ) -> Self {
        self.current_mut()
            .components
            .push(MemoryEstimations::delegate_estimation(
                estimation,
                description,
            ));
        self
    }

    pub fn fixed(self, description: impl Into<String>, bytes: usize) -> Self {
        self.add(MemoryEstimations::of_range(
            description,
            MemoryRange::of(bytes),
        ))
    }

    pub fn fixed_range(self, description: impl Into<String>, range: MemoryRange) -> Self {
        self.add(MemoryEstimations::of_range(description, range))
    }

    pub fn range_per_graph_dimension<F>(self, description: impl Into<String>, f: F) -> Self
    where
        F: Fn(&dyn GraphDimensions, usize) -> MemoryRange + Send + Sync + 'static,
    {
        self.add(MemoryEstimations::of_resident(description, f))
    }

    pub fn per_node(
        self,
        description: impl Into<String>,
        estimation: Box<dyn MemoryEstimation>,
    ) -> Self {
        self.add(MemoryEstimations::per_node(description, estimation))
    }

    pub fn per_thread(
        self,
        description: impl Into<String>,
        estimation: Box<dyn MemoryEstimation>,
    ) -> Self {
        self.add(MemoryEstimations::per_thread(description, estimation))
    }

    pub fn build(mut self) -> Box<dyn MemoryEstimation> {
        while self.stack.len() > 1 {
            self = self.end_field();
        }

        let frame = self.stack.pop().expect("builder always has one frame");
        Box::new(CompositeEstimation {
            description: frame.description,
            components: frame.components,
        })
    }
}

struct NullEstimation;

impl MemoryEstimation for NullEstimation {
    fn description(&self) -> String {
        "".to_string()
    }

    fn estimate(&self, _dimensions: &dyn GraphDimensions, _concurrency: usize) -> MemoryTree {
        MemoryTree::empty()
    }
}

struct LeafEstimation {
    description: String,
    resident: Arc<dyn Fn(&dyn GraphDimensions, usize) -> MemoryRange + Send + Sync>,
}

impl LeafEstimation {
    fn new<F>(description: String, resident: F) -> Self
    where
        F: Fn(&dyn GraphDimensions, usize) -> MemoryRange + Send + Sync + 'static,
    {
        Self {
            description,
            resident: Arc::new(resident),
        }
    }
}

impl MemoryEstimation for LeafEstimation {
    fn description(&self) -> String {
        self.description.clone()
    }

    fn estimate(&self, dimensions: &dyn GraphDimensions, concurrency: usize) -> MemoryTree {
        let range = (self.resident)(dimensions, concurrency);
        MemoryTree::leaf(self.description.clone(), range)
    }
}

struct SetupEstimation {
    description: String,
    setup: Arc<dyn Fn(&dyn GraphDimensions, usize) -> Box<dyn MemoryEstimation> + Send + Sync>,
}

impl MemoryEstimation for SetupEstimation {
    fn description(&self) -> String {
        self.description.clone()
    }

    fn estimate(&self, dimensions: &dyn GraphDimensions, concurrency: usize) -> MemoryTree {
        let estimation = (self.setup)(dimensions, concurrency);
        estimation.estimate(dimensions, concurrency)
    }
}

struct AndThenEstimation {
    description: String,
    delegate: Box<dyn MemoryEstimation>,
    and_then: Arc<dyn Fn(MemoryRange, &dyn GraphDimensions, usize) -> MemoryRange + Send + Sync>,
}

impl MemoryEstimation for AndThenEstimation {
    fn description(&self) -> String {
        self.description.clone()
    }

    fn estimate(&self, dimensions: &dyn GraphDimensions, concurrency: usize) -> MemoryTree {
        let tree = self.delegate.estimate(dimensions, concurrency);
        let new_range = (self.and_then)(*tree.memory_usage(), dimensions, concurrency);
        MemoryTree::new(
            self.description.clone(),
            new_range,
            tree.components().to_vec(),
        )
    }
}

struct CompositeEstimation {
    description: String,
    components: Vec<Box<dyn MemoryEstimation>>,
}

impl MemoryEstimation for CompositeEstimation {
    fn description(&self) -> String {
        self.description.clone()
    }

    fn estimate(&self, dimensions: &dyn GraphDimensions, concurrency: usize) -> MemoryTree {
        let trees: Vec<MemoryTree> = self
            .components
            .iter()
            .map(|e| e.estimate(dimensions, concurrency))
            .collect();

        let total = trees
            .iter()
            .fold(MemoryRange::empty(), |acc, t| acc.add(t.memory_usage()));

        MemoryTree::new(self.description.clone(), total, trees)
    }
}

struct DelegateEstimation {
    description: String,
    delegate: Box<dyn MemoryEstimation>,
}

impl MemoryEstimation for DelegateEstimation {
    fn description(&self) -> String {
        self.description.clone()
    }

    fn estimate(&self, dimensions: &dyn GraphDimensions, concurrency: usize) -> MemoryTree {
        let tree = self.delegate.estimate(dimensions, concurrency);
        MemoryTree::new(
            self.description.clone(),
            *tree.memory_usage(),
            tree.components().to_vec(),
        )
    }
}

struct MaxEstimation {
    description: String,
    components: Vec<Box<dyn MemoryEstimation>>,
}

impl MemoryEstimation for MaxEstimation {
    fn description(&self) -> String {
        self.description.clone()
    }

    fn estimate(&self, dimensions: &dyn GraphDimensions, concurrency: usize) -> MemoryTree {
        let trees: Vec<MemoryTree> = self
            .components
            .iter()
            .map(|e| e.estimate(dimensions, concurrency))
            .collect();

        let total = trees.iter().fold(MemoryRange::empty(), |acc, t| {
            MemoryRange::maximum(&acc, t.memory_usage())
        });

        MemoryTree::new(self.description.clone(), total, trees)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::graph_dimensions::ConcreteGraphDimensions;

    #[test]
    fn builder_builds_composite_tree_with_sum() {
        let dims = ConcreteGraphDimensions::of(10, 0);

        let estimation = MemoryEstimations::builder("root")
            .fixed("a", 10)
            .fixed("b", 20)
            .build();

        let tree = estimation.estimate(&dims, 4);
        assert_eq!(tree.description(), "root");
        assert_eq!(tree.memory_usage().min(), 30);
        assert_eq!(tree.components().len(), 2);
    }

    #[test]
    fn builder_nested_fields() {
        let dims = ConcreteGraphDimensions::of(10, 0);

        let estimation = MemoryEstimations::builder("root")
            .start_field(MemoryEstimations::RESIDENT_MEMORY)
            .fixed("residentPart", 10)
            .end_field()
            .start_field(MemoryEstimations::TEMPORARY_MEMORY)
            .fixed("tempPart", 20)
            .end_field()
            .build();
        let tree = estimation.estimate(&dims, 1);

        assert_eq!(tree.components().len(), 2);
        assert_eq!(tree.memory_usage().min(), 30);
        assert!(tree.render().contains("residentMemory"));
        assert!(tree.render().contains("temporaryMemory"));
    }

    #[test]
    fn per_node_modifier() {
        let dims = ConcreteGraphDimensions::of(3, 0);
        let per = MemoryEstimations::of_range("per", MemoryRange::of(10));
        let estimation = MemoryEstimations::per_node("total", per);

        let tree = estimation.estimate(&dims, 1);
        assert_eq!(tree.memory_usage().min(), 30);
    }

    #[test]
    fn max_estimation_uses_element_wise_max() {
        let dims = ConcreteGraphDimensions::of(1, 0);

        let a = MemoryEstimations::of_range("a", MemoryRange::of_range(10, 100));
        let b = MemoryEstimations::of_range("b", MemoryRange::of_range(20, 50));

        let estimation = MemoryEstimations::max_estimation(vec![a, b]);
        let tree = estimation.estimate(&dims, 1);

        assert_eq!(tree.memory_usage().min(), 20);
        assert_eq!(tree.memory_usage().max(), 100);
    }
}
