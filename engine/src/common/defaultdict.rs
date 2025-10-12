use std::{
    collections::HashMap,
    hash::{BuildHasher, Hash},
};

pub struct DefaultHashMap<K, V, S, F>
where
    K: Eq + Hash,
    F: Fn() -> V,
{
    map: HashMap<K, V, S>,
    default: F,
}

impl<K, V, S, F> DefaultHashMap<K, V, S, F>
where
    S: Default + BuildHasher,
    K: Eq + Hash,
    F: Fn() -> V,
{
    pub fn new(default: F) -> Self {
        Self {
            map: HashMap::default(),
            default,
        }
    }

    pub fn has(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    pub fn get_mut_or_default(&mut self, key: K) -> &mut V {
        self.map.entry(key).or_insert_with(&self.default)
    }

    pub fn zeroed(&mut self, key: K) {
        self.map.entry(key).or_insert_with(&self.default);
    }

    pub fn get(&self, key: &K) -> &V {
        self.map.get(key).unwrap()
    }

    pub fn inner(&self) -> &HashMap<K, V, S> {
        &self.map
    }

    pub fn into_inner(self) -> HashMap<K, V, S> {
        self.map
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.map.insert(key, value)
    }
}

pub type DefaultFxHashMap<K, V, F> = DefaultHashMap<K, V, rustc_hash::FxBuildHasher, F>;
