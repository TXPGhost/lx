use std::cell::{Ref, RefCell, RefMut};
use std::fmt::Debug;
use std::rc::{Rc, Weak};

pub trait NodeElt: Clone + Debug + PartialEq + Eq {}
impl<T: Clone + Debug + PartialEq + Eq> NodeElt for T {}

pub trait NodeMeta: NodeElt + Default {}
impl<T: NodeElt + Default> NodeMeta for T {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node<T: NodeElt, M: NodeMeta> {
    pub elt: Rc<RefCell<T>>,
    pub meta: Rc<RefCell<M>>,
}

impl<T: NodeElt, M: NodeMeta> Node<T, M> {
    pub fn new(elt: T, meta: M) -> Self {
        Node {
            elt: Rc::new(RefCell::new(elt)),
            meta: Rc::new(RefCell::new(meta)),
        }
    }

    pub fn meta<U: NodeMeta>(self, meta: U) -> Node<T, U> {
        Node {
            elt: self.elt,
            meta: Rc::new(RefCell::new(meta)),
        }
    }

    pub fn emap<F, U: NodeElt>(self, f: F) -> Node<U, M>
    where
        F: FnOnce(&T) -> U,
    {
        Node {
            elt: Rc::new(RefCell::new(f(&self.elt.borrow()))),
            meta: self.meta,
        }
    }

    pub fn mmap<F, U: NodeMeta>(self, f: F) -> Node<T, U>
    where
        F: FnOnce(&M) -> U,
    {
        Node {
            elt: self.elt,
            meta: Rc::new(RefCell::new(f(&self.meta.borrow()))),
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

    pub fn get(&self) -> Ref<'_, T> {
        self.elt.borrow()
    }

    pub fn get_mut(&self) -> RefMut<'_, T> {
        self.elt.borrow_mut()
    }

    pub fn as_weak(&self) -> NodeWeak<T, M> {
        NodeWeak {
            elt: Rc::downgrade(&self.elt),
            meta: Rc::downgrade(&self.meta),
        }
    }
}

#[derive(Clone, Debug)]
pub struct NodeWeak<T: NodeElt, M: NodeMeta> {
    pub elt: Weak<RefCell<T>>,
    pub meta: Weak<RefCell<M>>,
}

impl<T: NodeElt, M: NodeMeta> NodeWeak<T, M> {
    pub fn upgrade(&self) -> Option<Node<T, M>> {
        match (self.elt.upgrade(), self.meta.upgrade()) {
            (Some(elt), Some(meta)) => Some(Node { elt, meta }),
            _ => None,
        }
    }
}

// prevents infinite recursion in equality comparison
impl<T: NodeElt, M: NodeMeta> PartialEq for NodeWeak<T, M> {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}
impl<T: NodeElt, M: NodeMeta> Eq for NodeWeak<T, M> {}

pub trait NodeExt: NodeElt + Sized {
    fn elt<M: NodeMeta>(self) -> Node<Self, M> {
        Node::new(self, M::default())
    }

    fn node<M: NodeMeta>(self, meta: M) -> Node<Self, M> {
        Node::new(self, meta)
    }
}
impl<T: NodeElt + Sized> NodeExt for T {}
