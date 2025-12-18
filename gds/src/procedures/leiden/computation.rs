use super::spec::LeidenConfig;
use super::storage::LeidenStorage;
use std::collections::HashMap;

pub fn leiden<F>(node_count: usize, get_neighbors: F, config: &LeidenConfig) -> LeidenStorage
where
    F: Fn(usize) -> Vec<(usize, f64)>,
{
    let mut storage = LeidenStorage::new(node_count);
    let mut communities: Vec<u64> = match &config.seed_communities {
        Some(seed) => seed.clone(),
        None => (0..node_count).map(|i| i as u64).collect(),
    };
    let node_volumes: Vec<f64> = (0..node_count)
        .map(|n| get_neighbors(n).iter().map(|(_, w)| w).sum())
        .collect();
    let total_volume: f64 = node_volumes.iter().sum();
    let mut prev_mod = f64::NEG_INFINITY;

    for level in 0..config.max_iterations {
        let mut changed = true;
        let mut iter = 0;
        let mut cvol: HashMap<u64, f64> = HashMap::new();
        for (i, &c) in communities.iter().enumerate() {
            *cvol.entry(c).or_insert(0.0) += node_volumes[i];
        }

        while changed && iter < 100 {
            changed = false;
            iter += 1;
            for node in 0..node_count {
                let cc = communities[node];
                let nv = node_volumes[node];
                let mut cw: HashMap<u64, f64> = HashMap::new();
                let mut sw = 0.0;
                for (nb, w) in get_neighbors(node) {
                    if nb == node {
                        sw += w;
                    }
                    *cw.entry(communities[nb]).or_insert(0.0) += w;
                }
                let wc = cw.get(&cc).copied().unwrap_or(0.0) - sw;
                let mut bc = cc;
                let mut bg = 0.0;
                let cvwo = cvol[&cc] - nv;
                for (&cd, &ew) in &cw {
                    if cd == cc {
                        continue;
                    }
                    let cdv = cvol.get(&cd).copied().unwrap_or(0.0);
                    let g = (ew - wc) / total_volume
                        - config.gamma * nv * (cdv - cvwo) / (total_volume * total_volume);
                    if g > bg {
                        bg = g;
                        bc = cd;
                    }
                }
                if bc != cc && bg > 1e-10 {
                    communities[node] = bc;
                    *cvol.get_mut(&cc).unwrap() -= nv;
                    *cvol.entry(bc).or_insert(0.0) += nv;
                    changed = true;
                }
            }
        }

        let mut ie = 0.0;
        for n in 0..node_count {
            for (nb, w) in get_neighbors(n) {
                if communities[nb] == communities[n] {
                    ie += w;
                }
            }
        }
        let mut m = ie / total_volume;
        for v in cvol.values() {
            m -= config.gamma * (v * v) / (total_volume * total_volume);
        }
        storage.add_modularity(m);
        if (m - prev_mod).abs() < config.tolerance {
            storage.set_communities(remap(&communities));
            storage.set_modularity(m);
            storage.set_levels(level + 1);
            storage.set_converged(true);
            return storage;
        }
        prev_mod = m;
    }
    storage.set_communities(remap(&communities));
    storage.set_modularity(prev_mod);
    storage.set_levels(config.max_iterations);
    storage.set_converged(false);
    storage
}

fn remap(c: &[u64]) -> Vec<u64> {
    let u: std::collections::HashSet<u64> = c.iter().copied().collect();
    let mut s: Vec<u64> = u.into_iter().collect();
    s.sort_unstable();
    let m: HashMap<u64, u64> = s.iter().enumerate().map(|(i, &x)| (x, i as u64)).collect();
    c.iter().map(|x| m[x]).collect()
}
