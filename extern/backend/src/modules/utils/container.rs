use super::prelude::*;

// -------------------- CONTAINER -------------------- //
#[derive(Clone, PartialEq)]
pub struct Container<T> {
    pub tree: Vec<StrongContext<T>>,
}

impl<T> Container<T> {
    pub fn new() -> Self {
        Self { tree: Vec::new() }
    }
    pub fn push(&mut self, item: T) {
        self.push_context(StrongContext::from(item))
    }
    pub fn push_context(&mut self, ctx: StrongContext<T>) {
        self.tree.push(ctx);
    }
}

impl<T> Default for Container<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ContainerIter<'a, T> {
    iter: std::slice::Iter<'a, StrongContext<T>>,
}

impl<T> Container<T> {
    pub fn iter(&self) -> ContainerIter<T> {
        ContainerIter {
            iter: self.tree.iter(),
        }
    }
}

impl<'a, T> Iterator for ContainerIter<'a, T> {
    type Item = &'a StrongContext<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<T> FromIterator<StrongContext<T>> for Container<T> {
    fn from_iter<I: IntoIterator<Item = StrongContext<T>>>(iter: I) -> Self {
        let mut container = Container::new();
        for item in iter {
            container.push_context(item);
        }
        container
    }
}

// -------------------- NODE CONTAINER -------------------- //
pub type NodeContainer = Container<Node>;

// -------------------- PARAM CONTAINER -------------------- //
pub type ParamContainer = Container<Param>;
