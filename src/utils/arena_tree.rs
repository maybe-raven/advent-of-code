#![allow(dead_code)]
use std::ops::{Index, IndexMut};

use super::arena_linked_list::{ArenaLinkedList, LinkedListIter};

const ROOT_INDEX: usize = 0;

#[derive(Debug, Clone, Copy)]
struct TreeNode<T> {
    value: T,
    parent_index: usize,
    children_index: usize,
}

impl<T> TreeNode<T> {
    pub fn is_root(&self) -> bool {
        self.parent_index == ROOT_INDEX
    }
}

#[derive(Debug)]
pub struct TreeNodeEntry<'a, T> {
    source: &'a ArenaTree<T>,
    node: &'a TreeNode<T>,
}

impl<T> Copy for TreeNodeEntry<'_, T> {}
impl<T> Clone for TreeNodeEntry<'_, T> {
    fn clone(&self) -> Self {
        Self {
            source: self.source,
            node: self.node,
        }
    }
}

impl<T> TreeNodeEntry<'_, T> {
    pub fn get_parent(&self) -> Self {
        if self.node.is_root() {
            *self
        } else {
            Self {
                source: self.source,
                node: self.source.nodes.index(self.node.parent_index),
            }
        }
    }

    pub fn goto_parent(&mut self) {
        if self.node.is_root() {
            return;
        }

        self.node = self.source.nodes.index(self.node.parent_index)
    }
}

impl<'a, T> TreeNodeEntry<'a, T> {
    pub fn get(&self) -> &'a T {
        &self.node.value
    }

    pub fn get_children(&self) -> NodeChildren<'a, T> {
        NodeChildren {
            source: self.source,
            index_iter: self
                .source
                .children_lists
                .get_list(self.node.children_index),
        }
    }
}

#[derive(Debug)]
pub struct NodeChildren<'a, T> {
    source: &'a ArenaTree<T>,
    index_iter: LinkedListIter<'a, usize>,
}

impl<T> Copy for NodeChildren<'_, T> {}
impl<'a, T> Clone for NodeChildren<'a, T> {
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}

impl<'a, T> Iterator for NodeChildren<'a, T> {
    type Item = TreeNodeEntry<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let &i = self.index_iter.next()?;
        Some(TreeNodeEntry {
            source: self.source,
            node: self.source.nodes.index(i),
        })
    }
}

#[derive(Debug)]
pub struct TreeNodeEntryMut<'a, T> {
    source: &'a mut ArenaTree<T>,
    index: usize,
}

impl<T> TreeNodeEntryMut<'_, T> {
    pub fn get(&self) -> &T {
        &self.source.nodes[self.index].value
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.source.nodes[self.index].value
    }

    pub fn goto_parent(&mut self) {
        self.index = self.source.nodes[self.index].parent_index;
    }

    pub fn get_children(&self) -> NodeChildren<T> {
        NodeChildren {
            source: self.source,
            index_iter: self
                .source
                .children_lists
                .get_list(self.source.nodes[self.index].children_index),
        }
    }

    pub fn goto_child(&mut self, predicate: impl Fn(&TreeNodeEntry<T>) -> bool) -> bool {
        if let Some(&child_index) = self
            .source
            .children_lists
            .get_list(self.source.nodes[self.index].children_index)
            .find(|&&i| {
                predicate(&TreeNodeEntry {
                    source: self.source,
                    node: self.source.nodes.index(i),
                })
            })
        {
            self.index = child_index;
            true
        } else {
            false
        }
    }

    // pub fn get_children_mut(&mut self) -> NodeChildrenMut<T> {
    //     let iter = self
    //         .source
    //         .children_lists
    //         .get_list(self.source.nodes[self.index].children_index);
    //     NodeChildrenMut {
    //         source: self.source,
    //         index_iter: iter,
    //     }
    // }

    fn node(&self) -> &TreeNode<T> {
        self.source.nodes.index(self.index)
    }

    fn node_mut(&mut self) -> &mut TreeNode<T> {
        self.source.nodes.index_mut(self.index)
    }
}

pub struct NodeChildrenMut<'a, T> {
    source: &'a mut ArenaTree<T>,
    index_iter: Option<LinkedListIter<'a, usize>>,
}

#[derive(Debug)]
pub struct ArenaTree<T> {
    nodes: Vec<TreeNode<T>>,
    children_lists: ArenaLinkedList<usize>,
}

impl<T> Default for ArenaTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ArenaTree<T> {
    pub fn new() -> Self {
        ArenaTree {
            nodes: Vec::new(),
            children_lists: ArenaLinkedList::new(),
        }
    }
}
