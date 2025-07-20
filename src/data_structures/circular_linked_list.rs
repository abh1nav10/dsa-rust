use std::marker::PhantomData;
use std::fmt::{self, Formatter, Display};
use std::ptr::NonNull;

pub struct Node<T> {
    value : T,
    next : Option<NonNull<Node<T>>>,
    prev : Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(t : T) -> Self {
        Self {
            value : t,
            next : None,
            prev : None,
        }
    }
}

pub struct CircularLinkedList<T> {
    head : Option<NonNull<Node<T>>>,
    tail : Option<NonNull<Node<T>>>,
    current_length : u32,
    max_length : u32,
    marker : PhantomData<Box<Node<T>>>,
}    

impl<T> CircularLinkedList<T> {
    pub fn new(max_length : u32) -> Self {
        Self {
            head : None,
            tail : None,
            current_length : 0,
            max_length : max_length,
            marker: PhantomData,
        }
    }

    pub fn insert_at_head(&mut self, object : T) {
        let mut node = Box::new(Node::new(object));
        node.next = self.head;
        if self.current_length < self.max_length - 1 {
            node.prev = None;
            let node_ptr = NonNull::new(Box::into_raw(node));
            match self.head {
                Some(something) => {
                    unsafe {
                        (*something.as_ptr()).prev = node_ptr;
                    }
                }
                None => self.tail = node_ptr,
            }
            self.head = node_ptr;
            self.current_length = self.current_length + 1;
            return;
        }
        if self.current_length == self.max_length - 1 {
            if let Some(mut a) = self.head {
                for _ in 0..(self.max_length - 1) {
                    unsafe {
                        a = ((*a.as_ptr()).next).expect("Out of bounds");
                    }
                }
                node.prev = Some(a);
                let node_ptr = NonNull::new(Box::into_raw(node));
                if let Some(b) = self.head {
                    unsafe {
                        (*b.as_ptr()).prev = node_ptr;
                    }
                    self.head = node_ptr;
                }
               
                    unsafe {
                        (*a.as_ptr()).next = self.head;
                    }
                
                self.current_length = self.max_length;
            }
            return;
        }

        if self.current_length == self.max_length {

            if let Some(mut p) = self.head {
                for _ in 0..self.max_length {
                    unsafe {
                        p = ((*p.as_ptr()).next).expect("Node not found");
                    }
                }
                node.prev = unsafe {
                    (*p.as_ptr()).prev
                };
                let node_ptr = NonNull::new(Box::into_raw(node));
                if let Some(k) = self.head {
                    unsafe {
                        (*k.as_ptr()).prev = node_ptr;
                    }
                    self.head = node_ptr;
                } 
                
                
                    let mut drop = unsafe{Box::from_raw(p.as_ptr())};
                    unsafe{match drop.prev {
                        Some(mut some) => some.as_mut().next = self.head,
                        None => unreachable!(),
                    }}
                    self.tail = drop.prev;
                
            }
            return;
        }
    }    
}