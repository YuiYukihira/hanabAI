use std::{collections::HashMap, hash::Hash};

pub struct PriorityQueue<K, V> {
    inner: HashMap<K, Vec<V>>,
}

impl<K, V> PriorityQueue<K, V> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
}

impl<K: Eq + Ord + Hash + Copy, V> PriorityQueue<K, V> {
    pub fn push(&mut self, item: V, priority: K) {
        self.inner.entry(priority).or_default().push(item);
    }

    pub fn push_front(&mut self, item: V, priority: K) {
        self.inner.entry(priority).or_default().insert(0, item);
    }

    pub fn pop(&mut self) -> Option<V> {
        let lowest_priority = *self.inner.keys().min()?;
        let v = self.inner.get_mut(&lowest_priority)?;
        v.pop()
    }

    pub fn peek(&self) -> Option<&V> {
        let lowest_priority = *self.inner.keys().min()?;
        let v = self.inner.get(&lowest_priority)?;
        v.first()
    }

    pub fn peek_mut(&mut self) -> Option<&mut V> {
        let lowest_priority = *self.inner.keys().min()?;
        let v = self.inner.get_mut(&lowest_priority)?;
        v.get_mut(0)
    }
}
