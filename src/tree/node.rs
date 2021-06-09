#![allow(dead_code)]

use raw_pointer::Pointer;
use std::cmp::Ordering;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Color {
    Red,
    Black
}

struct Node<K: Ord, V> {
    color: Color,
    left: NodePtr<K, V>,
    right: NodePtr<K, V>,
    parent: NodePtr<K, V>,
    key: K,
    value: V,
}

impl<K: Ord, V> Node<K, V> {
    #[inline]
    fn pair(self) -> (K, V) {
        (self.key, self.value)
    }
}

pub struct NodePtr<K: Ord, V>(Pointer<Node<K, V>>);

impl<K: Ord, V> NodePtr<K, V> {
    pub fn new(key: K, value: V) -> NodePtr<K, V> {
        let mut node = Box::new(Node {
            color: Color::Black,
            left: NodePtr::null(),
            right: NodePtr::null(),
            parent: NodePtr::null(),
            key, value
        });
        NodePtr(Pointer::<Node<K, V>>::new(node.as_mut()))
    }

    #[inline]
    pub fn set_color(&mut self, color: Color) {
        if self.is_null() {
            return;
        }
        self.0.color = color
    }

    #[inline]
    pub fn set_color_red(&mut self) {
        self.set_color(Color::Red)
    }

    #[inline]
    pub fn set_color_black(&mut self) {
        self.set_color(Color::Black)
    }

    #[inline]
    pub fn flip_color(&mut self) {
        match self.0.color {
            Color::Black => self.set_color_red(),
            Color::Red => self.set_color_black()
        }
    }

    #[inline]
    pub fn get_color(&self) -> Color {
        if self.is_null() {
            return Color::Black;
        }
        self.0.color
    }

    #[inline]
    pub fn is_color_black(&self) -> bool {
        self.get_color() == Color::Black
    }

    #[inline]
    pub fn is_color_red(&self) -> bool {
        !self.is_color_black()
    }

    #[inline]
    pub fn is_left_child(&self) -> bool {
        self.parent().left() == *self
    }

    #[inline]
    pub fn is_right_child(&self) -> bool {
        self.parent().right() == *self
    }

    #[inline]
    pub fn min_node(self) -> NodePtr<K, V> {
        let mut temp = self.clone();
        while !temp.left().is_null() {
            temp = temp.left();
        }

        temp
    }

    #[inline]
    pub fn max_node(self) -> NodePtr<K, V> {
        let mut temp = self.clone();
        while !temp.right().is_null() {
            temp = temp.right();
        }

        temp
    }

    #[inline]
    pub fn next(self) -> NodePtr<K, V> {
        if !self.right().is_null() {
            self.right().min_node()
        } else {
            let mut temp = self;
            loop {
                if temp.parent().is_null() {
                    return NodePtr::null();
                }
                if temp.is_left_child() {
                    return temp.parent();
                }
                temp = temp.parent();
            }
        }
    }

    #[inline]
    pub fn prev(self) -> NodePtr<K, V> {
        if !self.left().is_null() {
            self.left().max_node()
        } else {
            let mut temp = self;
            loop {
                if temp.parent().is_null() {
                    return NodePtr::null();
                }
                if temp.is_right_child() {
                    return temp.parent();
                }
                temp = temp.parent();
            }
        }
    }

    #[inline]
    pub fn set_left(&mut self, left: NodePtr<K, V>) {
        if self.is_null() {
            return;
        }
        self.0.left = left
    }

    #[inline]
    pub fn set_right(&mut self, right: NodePtr<K, V>) {
        if self.is_null() {
            return;
        }
        self.0.right = right
    }

    #[inline]
    pub fn set_parent(&mut self, parent: NodePtr<K, V>) {
        if self.is_null() {
            return;
        }
        self.0.parent = parent
    }

    #[inline]
    pub fn left(&self) -> NodePtr<K, V> {
        if self.is_null() {
            return NodePtr::null();
        }
        self.0.left.clone()
    }

    #[inline]
    pub fn right(&self) -> NodePtr<K, V> {
        if self.is_null() {
            return NodePtr::null();
        }
        self.0.right.clone()
    }

    #[inline]
    pub fn parent(&self) -> NodePtr<K, V> {
        if self.is_null() {
            return NodePtr::null();
        }
        self.0.parent.clone()
    }

    #[inline]
    pub fn null() -> NodePtr<K, V> {
        NodePtr::<K, V>(Pointer::<Node<K, V>> {
            ptr: std::ptr::null_mut() as *mut Node<K, V>
        })
    }

    #[inline]
    pub fn is_null(&self) -> bool {
        self.0.ptr.is_null()
    }
}

impl<K: Ord, V> Clone for NodePtr<K, V> {
    fn clone(&self) -> NodePtr<K, V> {
        NodePtr(Pointer::<Node<K, V>> {
            ptr: self.0.ptr
        })
    }
}

impl<K: Ord, V> Ord for NodePtr<K, V> {
    fn cmp(&self, other: &NodePtr<K, V>) -> Ordering {
        self.0.key.cmp(&(other.0.key))
    }
}

impl<K: Ord, V> PartialOrd for NodePtr<K, V> {
    fn partial_cmp(&self, other: &NodePtr<K, V>) -> Option<Ordering> {
        Some(self.0.key.cmp(&(other.0.key)))
    }
}

impl<K: Ord, V> PartialEq for NodePtr<K, V> {
    fn eq(&self, other: &NodePtr<K, V>) -> bool {
        self.0.ptr == other.0.ptr
    }
}

impl<K: Ord, V> Eq for NodePtr<K, V> {}
