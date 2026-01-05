use super::spec::RelationshipIntersectConfig;

pub trait IntersectionConsumer {
    fn accept(&mut self, c: usize, b: usize, a: usize);
}

impl<F> IntersectionConsumer for F
where
    F: FnMut(usize, usize, usize),
{
    fn accept(&mut self, c: usize, b: usize, a: usize) {
        self(c, b, a)
    }
}

pub trait RelationshipIntersect {
    fn intersect_all(&mut self, a: usize, consumer: &mut dyn IntersectionConsumer);
}

pub trait AdjacencyProvider {
    /// Degree for `node`.
    fn degree(&self, node: usize) -> usize;

    /// Sorted (non-decreasing) adjacency list for `node`.
    fn neighbors(&self, node: usize) -> &[usize];
}

/// Enumerates triangles by intersecting adjacency lists.
///
/// Matches the Java contract: emits triangles in canonical order `c < b < a`.
pub struct GraphIntersect<'a, P: AdjacencyProvider> {
    provider: &'a P,
    max_degree: usize,
}

impl<'a, P: AdjacencyProvider> GraphIntersect<'a, P> {
    pub fn new(provider: &'a P, config: RelationshipIntersectConfig) -> Self {
        let max_degree = if config.max_degree == u64::MAX {
            usize::MAX
        } else {
            config.max_degree as usize
        };
        Self {
            provider,
            max_degree,
        }
    }

    fn degree_ok(&self, node: usize) -> bool {
        self.provider.degree(node) <= self.max_degree
    }
}

impl<'a, P: AdjacencyProvider> RelationshipIntersect for GraphIntersect<'a, P> {
    fn intersect_all(&mut self, a: usize, consumer: &mut dyn IntersectionConsumer) {
        if !self.degree_ok(a) {
            return;
        }

        let neighbors_of_a = self.provider.neighbors(a);

        // Iterate b where b < a.
        for &b in neighbors_of_a.iter() {
            if b >= a {
                break;
            }
            if !self.degree_ok(b) {
                continue;
            }

            let neighbors_of_b = self.provider.neighbors(b);

            // Intersect neighbors_of_a and neighbors_of_b for c < b.
            let mut ia = 0usize;
            let mut ib = 0usize;

            while ia < neighbors_of_a.len() && ib < neighbors_of_b.len() {
                let ca = neighbors_of_a[ia];
                let cb = neighbors_of_b[ib];

                if ca >= b || cb >= b {
                    break;
                }

                if ca == cb {
                    let c = ca;
                    if self.degree_ok(c) {
                        consumer.accept(c, b, a);
                    }
                    ia += 1;
                    ib += 1;
                } else if ca < cb {
                    ia += 1;
                } else {
                    ib += 1;
                }
            }
        }
    }
}
