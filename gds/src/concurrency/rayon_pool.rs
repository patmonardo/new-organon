use crate::concurrency::Concurrency;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::collections::HashMap;
use std::sync::Arc;

static POOLS_BY_SIZE: Lazy<RwLock<HashMap<usize, Arc<ThreadPool>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub(crate) fn install_with_concurrency<R>(
    concurrency: Concurrency,
    f: impl FnOnce() -> R + Send,
) -> R
where
    R: Send,
{
    let pool_size = concurrency.value();

    // Fast path: avoid taking an exclusive lock on every call.
    let cached_pool = { POOLS_BY_SIZE.read().get(&pool_size).cloned() };
    if let Some(pool) = cached_pool {
        return pool.install(f);
    }

    let pool = {
        let mut pools = POOLS_BY_SIZE.write();
        pools
            .entry(pool_size)
            .or_insert_with(|| {
                Arc::new(
                    ThreadPoolBuilder::new()
                        .num_threads(pool_size)
                        .thread_name(move |idx| format!("gds-rayon-{}-{}", pool_size, idx))
                        .build()
                        .expect("failed to build Rayon thread pool"),
                )
            })
            .clone()
    };

    pool.install(f)
}
