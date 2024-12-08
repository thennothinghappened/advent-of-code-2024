use std::hash::Hash;

use rustc_hash::FxHashMap;

/// Further extensions to the tools provided by `itertools`. Some of these are modified functions
/// taken from its library.
pub trait MoreIterTools: Iterator {
    /// Return a `FxHashMap` of keys mapped to `Vec`s of values. Keys and values
    /// are taken from `(Key, Value)` tuple pairs yielded by the input iterator.
    ///
    /// Essentially a shorthand for `.into_grouping_map().collect::<Vec<_>>()`.
    ///
    /// ```
    /// use itertools::Itertools;
    ///
    /// let data = vec![(0, 10), (2, 12), (3, 13), (0, 20), (3, 33), (2, 42)];
    /// let lookup = data.into_iter().into_group_map();
    ///
    /// assert_eq!(lookup[&0], vec![10, 20]);
    /// assert_eq!(lookup.get(&1), None);
    /// assert_eq!(lookup[&2], vec![12, 42]);
    /// assert_eq!(lookup[&3], vec![13, 33]);
    /// ```
    fn into_group_map<K, V>(self) -> FxHashMap<K, Vec<V>>
    where
        Self: Iterator<Item = (K, V)> + Sized,
        K: Hash + Eq,
    {
        let mut lookup = FxHashMap::default();

        self.for_each(|(key, val)| {
            lookup.entry(key).or_insert_with(Vec::new).push(val);
        });

        lookup
    }
}
