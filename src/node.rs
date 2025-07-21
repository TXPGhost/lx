use std::cell::RefCell;
use std::fmt::Debug;

pub trait NodeElt: Clone + Debug + PartialEq + Eq {}
impl<T: Clone + Debug + PartialEq + Eq> NodeElt for T {}

pub trait NodeMeta: NodeElt + Default {}
impl<T: NodeElt + Default> NodeMeta for T {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node<T: NodeElt, M: NodeMeta> {
    pub elt: Box<RefCell<T>>,
    pub meta: Box<RefCell<M>>,
}

impl<T: NodeElt, M: NodeMeta> Node<T, M> {
    pub fn new(elt: T, meta: M) -> Self {
        Node {
            elt: Box::new(RefCell::new(elt)),
            meta: Box::new(RefCell::new(meta)),
        }
    }

    pub fn meta<U: NodeMeta>(self, meta: U) -> Node<T, U> {
        Node {
            elt: self.elt,
            meta: Box::new(RefCell::new(meta)),
        }
    }

    pub fn emap<F, U: NodeElt>(self, f: F) -> Node<U, M>
    where
        F: FnOnce(&T) -> U,
    {
        Node {
            elt: Box::new(RefCell::new(f(&self.elt.borrow()))),
            meta: self.meta,
        }
    }

    pub fn mmap<F, U: NodeMeta>(self, f: F) -> Node<T, U>
    where
        F: FnOnce(&M) -> U,
    {
        Node {
            elt: self.elt,
            meta: Box::new(RefCell::new(f(&self.meta.borrow()))),
        }
    }

    pub fn emap_mut<F>(self, f: F)
    where
        F: FnOnce(&mut T),
    {
        f(&mut self.elt.borrow_mut())
    }

    pub fn mmap_mut<F>(self, f: F)
    where
        F: FnOnce(&mut M),
    {
        f(&mut self.meta.borrow_mut())
    }
}

pub trait NodeExt: NodeElt + Sized {
    fn elt<M: NodeMeta>(self) -> Node<Self, M> {
        Node::new(self, M::default())
    }

    fn node<M: NodeMeta>(self, meta: M) -> Node<Self, M> {
        Node::new(self, meta)
    }
}
