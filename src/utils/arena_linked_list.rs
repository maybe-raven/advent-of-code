use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy)]
struct LinkedNode<T> {
    value: T,
    next_index: Option<usize>,
}

#[derive(Debug)]
pub struct LinkedNodeEntry<'a, T> {
    source: &'a ArenaLinkedList<T>,
    node: &'a LinkedNode<T>,
}

impl<T> Copy for LinkedNodeEntry<'_, T> {}
impl<T> Clone for LinkedNodeEntry<'_, T> {
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}

impl<T> LinkedNodeEntry<'_, T> {
    pub fn next(self) -> Option<Self> {
        let next_index = self.node.next_index?;
        Some(Self {
            node: &self.source.nodes[next_index],
            ..self
        })
    }

    pub fn iter(&self) -> LinkedListIter<T> {
        self.into_iter()
    }
}

impl<'a, T> LinkedNodeEntry<'a, T> {
    pub fn get(&self) -> &'a T {
        &self.node.value
    }
}

impl<'a, T> IntoIterator for LinkedNodeEntry<'a, T> {
    type Item = &'a T;

    type IntoIter = LinkedListIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        LinkedListIter(Some(self))
    }
}

#[derive(Debug)]
pub struct LinkedNodeEntryMut<'a, T> {
    source: &'a mut ArenaLinkedList<T>,
    node_index: usize,
}

impl<T> LinkedNodeEntryMut<'_, T> {
    pub fn get(&self) -> &T {
        &self.node().value
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.node_mut().value
    }

    pub fn insert_next(&mut self, value: T) {
        let i = self.source.nodes.len();
        let old_next = self.node_mut().next_index.replace(i);
        self.source.nodes.push(LinkedNode {
            value,
            next_index: old_next,
        });
        self.node_index = i;
    }

    pub fn next(mut self) -> Option<Self> {
        let next_index = self.node_mut().next_index?;
        self.node_index = next_index;
        Some(self)
    }

    fn node(&self) -> &LinkedNode<T> {
        self.source.nodes.index(self.node_index)
    }

    fn node_mut(&mut self) -> &mut LinkedNode<T> {
        self.source.nodes.index_mut(self.node_index)
    }
}

#[derive(Debug)]
pub struct LinkedListEntry<'a, T> {
    source: &'a mut ArenaLinkedList<T>,
    head_index: usize,
}

impl<T> LinkedListEntry<'_, T> {
    pub fn prepend(&mut self, value: T) {
        let new_index = self.source.nodes.len();
        let old_head = self
            .source
            .heads
            .index_mut(self.head_index)
            .replace(new_index);
        let node = LinkedNode {
            value,
            next_index: old_head,
        };
        self.source.nodes.push(node);
    }
}

impl<'a, T> LinkedListEntry<'a, T> {
    pub fn into_head_entry(self) -> Option<LinkedNodeEntry<'a, T>> {
        let LinkedListEntry {
            source,
            head_index: index,
        } = self;
        Some(LinkedNodeEntry {
            source,
            node: source.nodes.index(source.heads[index]?),
        })
    }

    pub fn into_head_entry_mut(self) -> Option<LinkedNodeEntryMut<'a, T>> {
        let LinkedListEntry {
            source,
            head_index: index,
        } = self;
        let node_index = source.heads[index]?;
        Some(LinkedNodeEntryMut { source, node_index })
    }

    pub fn into_prepended_head_entry(self, value: T) -> LinkedNodeEntry<'a, T> {
        let LinkedListEntry {
            source,
            head_index: index,
        } = self;

        let old_head = source.heads.index_mut(index).replace(source.nodes.len());
        source.nodes.push(LinkedNode {
            value,
            next_index: old_head,
        });

        LinkedNodeEntry {
            source,
            node: source.nodes.last().unwrap(),
        }
    }

    pub fn into_prepended_head_entry_mut(self, value: T) -> LinkedNodeEntryMut<'a, T> {
        let LinkedListEntry {
            source,
            head_index: index,
        } = self;

        let i = source.nodes.len();
        let old_head = source.heads.index_mut(index).replace(i);
        source.nodes.push(LinkedNode {
            value,
            next_index: old_head,
        });

        LinkedNodeEntryMut {
            source,
            node_index: i,
        }
    }
}

impl<'a, T> IntoIterator for LinkedListEntry<'a, T> {
    type Item = &'a T;

    type IntoIter = LinkedListIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        LinkedListIter(self.into_head_entry())
    }
}

#[derive(Debug)]
pub struct LinkedListIter<'a, T>(Option<LinkedNodeEntry<'a, T>>);

impl<T> Copy for LinkedListIter<'_, T> {}
impl<'a, T> Clone for LinkedListIter<'a, T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.0.take()?;
        let item = entry.get();
        self.0 = entry.next();
        Some(item)
    }
}

#[derive(Debug, Clone)]
pub struct ArenaLinkedList<T> {
    heads: Vec<Option<usize>>,
    nodes: Vec<LinkedNode<T>>,
}

impl<T> Default for ArenaLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ArenaLinkedList<T> {
    pub fn new() -> Self {
        ArenaLinkedList {
            heads: Vec::new(),
            nodes: Vec::new(),
        }
    }

    pub fn add_list(&mut self) -> LinkedListEntry<T> {
        let i = self.heads.len();
        self.heads.push(None);
        LinkedListEntry {
            source: self,
            head_index: i,
        }
    }

    pub fn insert_list(&mut self, index: usize) -> LinkedListEntry<T> {
        self.heads.push(None);
        LinkedListEntry {
            source: self,
            head_index: index,
        }
    }

    pub fn get_list(&self, index: usize) -> LinkedListIter<T> {
        let node_entry = self
            .heads
            .get(index)
            .copied()
            .flatten()
            .and_then(|i| self.nodes.get(i))
            .map(|node| LinkedNodeEntry { source: self, node });

        LinkedListIter(node_entry)
    }

    pub fn get_list_mut(&mut self, index: usize) -> LinkedListEntry<T> {
        LinkedListEntry {
            source: self,
            head_index: index,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        let mut all = ArenaLinkedList::new();

        let mut linked_list = all.add_list();
        linked_list.prepend(4);
        linked_list.prepend(3);
        linked_list.prepend(1);
        linked_list.into_head_entry_mut().unwrap().insert_next(2);

        linked_list = all.add_list();
        let mut entry = linked_list.into_prepended_head_entry_mut(10);
        entry.insert_next(20);
        entry.insert_next(30);

        let mut iter = all.get_list_mut(0).into_iter();
        assert_eq!(Some(1), iter.next().copied());
        assert_eq!(Some(2), iter.next().copied());
        assert_eq!(Some(3), iter.next().copied());
        assert_eq!(Some(4), iter.next().copied());
        assert_eq!(None, iter.next());

        let mut iter = all.get_list_mut(1).into_iter();
        assert_eq!(Some(10), iter.next().copied());
        assert_eq!(Some(20), iter.next().copied());
        assert_eq!(Some(30), iter.next().copied());
        assert_eq!(None, iter.next());
    }
}
