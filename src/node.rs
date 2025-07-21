use std::cell::RefCell;

pub struct Node<T, M> {
    pub elt: Box<RefCell<T>>,
    pub meta: Box<RefCell<M>>,
}

impl<T, M> Node<T, M> {
    pub fn new(elt: T, meta: M) -> Self {
        Node {
            elt: Box::new(RefCell::new(elt)),
            meta: Box::new(RefCell::new(meta)),
        }
    }

    pub fn meta<U>(self, meta: U) -> Node<T, U> {
        Node {
            elt: self.elt,
            meta: Box::new(RefCell::new(meta)),
        }
    }

    pub fn emap<F, U>(self, f: F) -> Node<U, M>
    where
        F: FnOnce(&T) -> U,
    {
        Node {
            elt: Box::new(RefCell::new(f(&self.elt.borrow()))),
            meta: self.meta,
        }
    }

    pub fn mmap<F, U>(self, f: F) -> Node<T, U>
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

pub trait NodeExt: Sized {
    fn elt(self) -> Node<Self, ()> {
        Node::new(self, ())
    }
    fn node<M>(self, meta: M) -> Node<Self, M> {
        Node::new(self, meta)
    }
}
