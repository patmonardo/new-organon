//! Collections Caching Extensions
//!
//! Provides a lightweight in-memory cache for repeated element access.
//! This is intended as a Polars-first, dataset-adjacent building block.

use crate::collections::{CachingError, CachingSupport, Collections};
use crate::config::{CollectionsBackend, Extension};
use crate::types::ValueType;
use std::collections::{HashMap, VecDeque};
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

/// Cache eviction policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CachePolicy {
    /// Evict in insertion order.
    Fifo,
    /// Disable eviction (cache grows until max_entries is reached, then inserts are ignored).
    None,
}

/// Caching configuration.
#[derive(Debug, Clone)]
pub struct CachingConfig {
    pub max_entries: usize,
    pub policy: CachePolicy,
    pub track_stats: bool,
}

impl Default for CachingConfig {
    fn default() -> Self {
        Self {
            max_entries: 1024,
            policy: CachePolicy::Fifo,
            track_stats: true,
        }
    }
}

/// Cache statistics.
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    pub hits: usize,
    pub misses: usize,
    pub evictions: usize,
    pub inserts: usize,
}

#[derive(Debug)]
struct CacheState<T> {
    entries: HashMap<usize, T>,
    order: VecDeque<usize>,
    stats: CacheStats,
}

impl<T> Default for CacheState<T> {
    fn default() -> Self {
        Self {
            entries: HashMap::new(),
            order: VecDeque::new(),
            stats: CacheStats::default(),
        }
    }
}

/// Cached collection wrapper.
pub struct CachedCollection<T, C>
where
    C: Collections<T>,
{
    inner: C,
    config: CachingConfig,
    enabled: AtomicBool,
    state: Mutex<CacheState<T>>,
    _phantom: PhantomData<T>,
}

impl<T, C> CachedCollection<T, C>
where
    C: Collections<T>,
    T: Clone + Send + Sync,
{
    pub fn new(inner: C) -> Self {
        Self {
            inner,
            config: CachingConfig::default(),
            enabled: AtomicBool::new(false),
            state: Mutex::new(CacheState::default()),
            _phantom: PhantomData,
        }
    }

    pub fn with_config(inner: C, config: CachingConfig) -> Self {
        Self {
            inner,
            config,
            enabled: AtomicBool::new(false),
            state: Mutex::new(CacheState::default()),
            _phantom: PhantomData,
        }
    }

    pub fn cache_stats(&self) -> Option<CacheStats> {
        if !self.config.track_stats {
            return None;
        }
        let state = self.state.lock().ok()?;
        Some(state.stats.clone())
    }

    pub fn cache_len(&self) -> usize {
        let state = match self.state.lock() {
            Ok(state) => state,
            Err(poisoned) => poisoned.into_inner(),
        };
        state.entries.len()
    }

    fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    fn record_hit(state: &mut CacheState<T>, track_stats: bool) {
        if track_stats {
            state.stats.hits += 1;
        }
    }

    fn record_miss(state: &mut CacheState<T>, track_stats: bool) {
        if track_stats {
            state.stats.misses += 1;
        }
    }

    fn record_insert(state: &mut CacheState<T>, track_stats: bool) {
        if track_stats {
            state.stats.inserts += 1;
        }
    }

    fn record_eviction(state: &mut CacheState<T>, track_stats: bool) {
        if track_stats {
            state.stats.evictions += 1;
        }
    }

    fn insert_cached(&self, index: usize, value: T) {
        let mut state = match self.state.lock() {
            Ok(state) => state,
            Err(poisoned) => poisoned.into_inner(),
        };

        if state.entries.contains_key(&index) {
            state.entries.insert(index, value);
            return;
        }

        if self.config.max_entries == 0 {
            return;
        }

        if state.entries.len() >= self.config.max_entries {
            match self.config.policy {
                CachePolicy::Fifo => {
                    if let Some(evict_key) = state.order.pop_front() {
                        state.entries.remove(&evict_key);
                        Self::record_eviction(&mut state, self.config.track_stats);
                    }
                }
                CachePolicy::None => {
                    return;
                }
            }
        }

        state.entries.insert(index, value);
        state.order.push_back(index);
        Self::record_insert(&mut state, self.config.track_stats);
    }
}

impl<T, C> Collections<T> for CachedCollection<T, C>
where
    C: Collections<T>,
    T: Clone + Send + Sync,
{
    fn get(&self, index: usize) -> Option<T> {
        if !self.is_enabled() {
            return self.inner.get(index);
        }

        if let Ok(mut state) = self.state.lock() {
            if let Some(value) = state.entries.get(&index).cloned() {
                Self::record_hit(&mut state, self.config.track_stats);
                return Some(value);
            }
            Self::record_miss(&mut state, self.config.track_stats);
        }

        let value = self.inner.get(index);
        if let Some(ref v) = value {
            self.insert_cached(index, v.clone());
        }
        value
    }

    fn set(&mut self, index: usize, value: T) {
        self.inner.set(index, value.clone());
        if self.is_enabled() {
            self.insert_cached(index, value);
        }
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn sum(&self) -> Option<T>
    where
        T: std::iter::Sum,
    {
        self.inner.sum()
    }

    fn min(&self) -> Option<T>
    where
        T: Ord,
    {
        self.inner.min()
    }

    fn max(&self) -> Option<T>
    where
        T: Ord,
    {
        self.inner.max()
    }

    fn mean(&self) -> Option<f64> {
        self.inner.mean()
    }

    fn std_dev(&self) -> Option<f64> {
        self.inner.std_dev()
    }

    fn variance(&self) -> Option<f64> {
        self.inner.variance()
    }

    fn median(&self) -> Option<T>
    where
        T: Ord,
    {
        self.inner.median()
    }

    fn percentile(&self, p: f64) -> Option<T>
    where
        T: Ord,
    {
        self.inner.percentile(p)
    }

    fn binary_search(&self, key: &T) -> Result<usize, usize>
    where
        T: Ord,
    {
        self.inner.binary_search(key)
    }

    fn sort(&mut self)
    where
        T: Ord,
    {
        self.inner.sort()
    }

    fn to_vec(self) -> Vec<T> {
        self.inner.to_vec()
    }

    fn as_slice(&self) -> &[T] {
        self.inner.as_slice()
    }

    fn is_null(&self, index: usize) -> bool {
        self.inner.is_null(index)
    }

    fn null_count(&self) -> usize {
        self.inner.null_count()
    }

    fn default_value(&self) -> T {
        self.inner.default_value()
    }

    fn backend(&self) -> CollectionsBackend {
        self.inner.backend()
    }

    fn features(&self) -> &[Extension] {
        &[Extension::Caching]
    }

    fn extensions(&self) -> &[Extension] {
        &[Extension::Caching]
    }

    fn value_type(&self) -> ValueType {
        self.inner.value_type()
    }

    fn with_capacity(_capacity: usize) -> Self
    where
        Self: Sized,
    {
        Self::new(C::with_capacity(_capacity))
    }

    fn with_defaults(_count: usize, _default_value: T) -> Self
    where
        Self: Sized,
    {
        Self::new(C::with_defaults(_count, _default_value))
    }
}

impl<T, C> CachingSupport<T> for CachedCollection<T, C>
where
    C: Collections<T>,
    T: Clone + Send + Sync,
{
    fn enable_caching(&mut self) -> Result<(), CachingError> {
        if self.config.max_entries == 0 {
            return Err(CachingError::CacheInitFailed(
                "max_entries must be > 0".to_string(),
            ));
        }
        self.enabled.store(true, Ordering::Relaxed);
        Ok(())
    }

    fn disable_caching(&mut self) {
        self.enabled.store(false, Ordering::Relaxed);
        self.clear_cache();
    }

    fn is_caching_enabled(&self) -> bool {
        self.is_enabled()
    }

    fn clear_cache(&mut self) {
        let mut state = match self.state.lock() {
            Ok(state) => state,
            Err(poisoned) => poisoned.into_inner(),
        };
        state.entries.clear();
        state.order.clear();
        state.stats = CacheStats::default();
    }
}
