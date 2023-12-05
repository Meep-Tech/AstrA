use std::{
    collections::{HashMap, HashSet},
    sync::LazyLock,
};

pub(crate) static _EMPTY_KEYS: LazyLock<HashMap<String, usize>> = LazyLock::new(|| HashMap::new());
pub(crate) static _EMPTY_TAGS: LazyLock<HashSet<String>> = LazyLock::new(|| HashSet::new());

pub trait Data<TNode> {
    fn name(&self) -> &str;

    fn tags(&self) -> &HashSet<String>;
    fn tag(&self, tag: &str) -> bool {
        return self.tags().contains(tag);
    }

    fn start(&self) -> usize;
    fn end(&self) -> usize;
    fn range(&self) -> std::ops::Range<usize> {
        return self.start()..self.end();
    }

    //fn parent(&self) -> Option<&TNode>;

    fn children(&self) -> Vec<&TNode>;
    fn child(&self, index: usize) -> Option<&TNode> {
        return match self.children().get(index) {
            Some(child) => Some(child),
            None => None,
        };
    }

    fn keys(&self) -> &HashMap<String, usize>;
    fn key(&self, index: usize) -> Option<&String> {
        return match self.keys().iter().find(|(_, i)| **i == index) {
            Some((key, _)) => Some(key),
            None => None,
        };
    }
    fn index(&self, key: &str) -> Option<usize> {
        return match self.keys().get(key) {
            Some(index) => Some(*index),
            None => None,
        };
    }

    fn fields(&self) -> HashMap<String, &TNode> {
        return self
            .keys()
            .iter()
            .map(|(key, index)| (key.clone(), self.child(*index).unwrap()))
            .collect();
    }
    fn field(&self, key: &str) -> Option<&TNode> {
        return match self.index(key) {
            Some(index) => self.child(index),
            None => None,
        };
    }
}
