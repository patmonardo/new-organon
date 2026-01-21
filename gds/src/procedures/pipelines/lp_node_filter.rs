use crate::types::graph::id_map::IdMap;
use crate::types::graph::Graph;

pub trait LPNodeFilter: Send + Sync {
    fn valid_node_count(&self) -> i64;
    fn test(&self, node_id: i64) -> bool;
}

struct AllNodesFilter {
    node_count: i64,
}

impl LPNodeFilter for AllNodesFilter {
    fn valid_node_count(&self) -> i64 {
        self.node_count
    }

    fn test(&self, _node_id: i64) -> bool {
        true
    }
}

struct IdMapNodeFilter<'a> {
    graph: &'a dyn Graph,
    id_map: &'a dyn IdMap,
}

impl<'a> LPNodeFilter for IdMapNodeFilter<'a> {
    fn valid_node_count(&self) -> i64 {
        self.id_map.node_count() as i64
    }

    fn test(&self, node_id: i64) -> bool {
        let Some(original) = self.graph.to_original_node_id(node_id) else {
            return false;
        };
        self.id_map.contains_original_id(original)
    }
}

pub fn lp_node_filter_of<'a>(
    graph: &'a dyn Graph,
    id_map: &'a dyn IdMap,
) -> Box<dyn LPNodeFilter + 'a> {
    if graph.node_count() == id_map.node_count() {
        Box::new(AllNodesFilter {
            node_count: id_map.node_count() as i64,
        })
    } else {
        Box::new(IdMapNodeFilter { graph, id_map })
    }
}
