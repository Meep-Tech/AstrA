use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dex<TVal, TTag = String>
where
    TTag: Eq + std::hash::Hash,
    TVal: Eq + std::hash::Hash,
{
    __: HashMap<TTag, HashSet<TVal>>,
}

impl<TVal, TTag> Dex<TVal, TTag>
where
    TTag: Eq + std::hash::Hash,
    TVal: Eq + std::hash::Hash,
{
    pub type Entry<'e> = DexEntry<'e, TVal> where TVal: 'e;

    /// Create a new empty Dex.
    pub fn new() -> Self {
        Self { __: HashMap::new() }
    }

    /// Create a new Dex with a set of tags and values.
    pub fn from(links: &dyn Iterator<Item = (TTag, TVal)>) -> Self {
        let mut dex = Self::new();
        for (tag, val) in links.into_iter() {
            dex.add(tag, val);
        }

        dex
    }

    /// Get all tags linked to any value in the Dex.
    pub fn tags(&self) -> HashSet<&TTag> {
        self.__.keys().collect()
    }

    /// Get all values linked to any tag in the Dex.
    pub fn vals(&self) -> HashSet<&TVal> {
        let mut vals = HashSet::new();
        for set in self.__.values() {
            for val in set {
                vals.insert(val);
            }
        }

        vals
    }

    /// Check if a tag is linked to a value.
    pub fn has(&self, tag: TTag, val: TVal) -> bool {
        if let Some(set) = self.__.get(&tag) {
            return set.contains(&val);
        }

        false
    }

    /// Get all values linked to a specific tag.
    pub fn with(&self, tag: TTag) -> Option<&HashSet<TVal>> {
        self.__.get(&tag)
    }

    /// Get all values linked to a specific tag, filtered by a predicate.
    pub fn when<F>(&self, f: F) -> Option<HashSet<TVal>>
    where
        F: Fn(&DexEntry<TVal>) -> bool,
    {
        let mut results = HashSet::new();
        for val in self.vals() {
            let entry = DexEntry { source: self, val };
            if f(&entry) {
                results.insert(val);
            }
        }

        if results.is_empty() {
            None
        } else {
            Some(results)
        }
    }

    /// Check if a tag is in the Dex.
    pub fn has_tag(&self, tag: TTag) -> bool {
        self.__.contains_key(&tag)
    }

    /// Check if a value is in the Dex.
    pub fn has_val(&self, val: TVal) -> bool {
        for set in self.__.values() {
            if set.contains(&val) {
                return true;
            }
        }

        false
    }

    /// Set or override the full set of linked values for a tag.
    pub fn set(&mut self, tag: TTag, vals: &dyn Iterator<Item = TVal>) {
        let mut set = HashSet::new();
        for val in vals.into_iter() {
            set.insert(val);
        }
        self.__.insert(tag, set);
    }

    /// Add a link between a value and a tag
    /// (Returns whether the link between this tag and value is new).
    pub fn add(&mut self, tag: TTag, val: TVal) -> bool {
        if let Some(tag_vals) = self.__.get_mut(&tag) {
            tag_vals.insert(val)
        } else {
            let mut tag_vals = HashSet::new();
            tag_vals.insert(val);
            self.__.insert(tag, tag_vals);

            true
        }
    }

    /// Cut a link between a tag and a value
    /// (Removes the value from the tag's set).
    pub fn cut(&mut self, tag: TTag, val: TVal) -> bool {
        if let Some(set) = self.__.get_mut(&tag) {
            return set.remove(&val);
        }

        false
    }

    /// Delete a tag
    /// (Removes the tag and its set of values from the Dex, leaving any other links between the effected values and other tags intact).
    pub fn del(&mut self, tag: TTag) -> Option<HashSet<TVal>> {
        self.__.remove(&tag)
    }

    /// Remove a value from the dex entirely
    /// (Removes the value from all tags' sets).
    pub fn rem(&mut self, val: &TVal) -> bool {
        let mut removed = false;
        for set in self.__.values_mut() {
            removed = removed || set.remove(val);
        }

        removed
    }
}

pub struct DexEntry<'e, V>
where
    V: Eq + std::hash::Hash,
{
    source: &'e Dex<V>,
    pub val: &'e V,
}

impl<'e, T> DexEntry<'e, T>
where
    T: Eq + std::hash::Hash,
{
    pub fn has(&self, tag: &str) -> bool {
        match self.source.__.get(tag) {
            Some(set) => set.contains(self.val),
            None => false,
        }
    }
}
