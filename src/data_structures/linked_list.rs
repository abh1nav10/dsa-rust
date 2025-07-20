use std::ptr::NonNull;
use std::marker::PhantomData;
use std::fmt::{self, Display, Formatter};

pub struct Node<T> {
    pub value : T,
    pub next : Option<NonNull<Node<T>>>,
    pub prev : Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(t: T) -> Node<T> {
        Node {
            value : t,
            next : None,
            prev : None,
        }
    }
}

pub struct LinkedList<T> {
    pub length : u32,
    pub head : Option<NonNull<Node<T>>>,
    pub tail : Option<NonNull<Node<T>>>,
    marker : PhantomData<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length : 0,
            head : None,
            tail : None,
            marker : PhantomData,
        }
    }
    pub fn insert_at_head(&mut self, object : T) {
        let mut node = Box::new(Node::new(object));
        node.next = self.head;
        node.prev = None;
        let non_null = NonNull::new(Box::into_raw(node));
        match self.head {
            None => self.tail = non_null,
            Some(head_ptr) => {
                unsafe {
                    (*head_ptr.as_ptr()).prev = non_null;
                }
            }
        }
        self.head = non_null;
        self.length = self.length + 1;
    }

    pub fn insert_at_tail(&mut self, object : T) {
        let mut node = Box::new(Node::new(object));
        node.prev = self.tail;
        node.next = None;
        let non_null = NonNull::new(Box::into_raw(node));
        match self.tail {
            None => self.head = non_null,
            Some(tail_ptr) => {
                unsafe {
                    (*tail_ptr.as_ptr()).next = non_null;
                }
            }
        }
        self.tail = non_null;
        self.length = self.length + 1;
    }

    pub fn insert_at_ith(&mut self, index: u32, object : T) {
        if self.length < index {
            panic!("Index out of bounds");
        }
        if index == 0 || self.head.is_none() {
            self.insert_at_head(object);
            return;
        }
        if self.length == index {
            self.insert_at_tail(object);
            return;
        }
        if let Some(mut ith_node) = self.head {
            for _ in 0..index {
                unsafe {
                    match (*ith_node.as_ptr()).next {
                        None => panic!("Index out of bounds"),
                        Some(ith) => {
                            ith_node = ith;
                        }
                    }
                }
            }
            let mut node = Box::new(Node::new(object));           
            unsafe{
                node.prev = (*ith_node.as_ptr()).prev;
                node.next = Some(ith_node);
                if let Some(mut hth_node) = (*ith_node.as_ptr()).prev {
                    let non_null = NonNull::new(Box::into_raw(node));
                    (*hth_node.as_ptr()).next = non_null;
                    (*ith_node.as_ptr()).prev = non_null;
                    self.length = self.length + 1;
                }
            }
        }
    }

    pub fn delete_head(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }
        self.head.map(|head_ptr| unsafe {
            let old_head = Box::from_raw(head_ptr.as_ptr());
            match old_head.next {
                Some(mut n) => n.as_mut().prev = None,
                None => self.tail = None,
            }
            self.head = old_head.next;
            self.length.checked_add_signed(-1).unwrap_or(0);
            old_head.value
        })
    }

    pub fn delete_tail(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }
        self.tail.map(|tail_ptr| unsafe {
            let old_tail = Box::from_raw(tail_ptr.as_ptr());
            match old_tail.prev {
                None => self.head = None,
                Some(mut n) => n.as_mut().next = None,
            }
            self.tail = old_tail.prev;
            self.length = self.length - 1;
            old_tail.value
        })
    }

    pub fn delete_ith(&mut self, index : u32,) -> Option<T> {
        if self.length <= index {
            panic!("Index out of bounds");
        }
        if index == 0 || self.head.is_none() {
            self.delete_head();
        }
        if self.length - 1 == index {
            self.delete_tail();
        }
        if let Some(mut n) = self.head {
            for _ in 0..index {
                unsafe {
                    match (*n.as_ptr()).next {
                        None => panic!(),
                        Some(mth) => n = mth,
                    }
                }
            }
            unsafe {
                let node = Box::from_raw(n.as_ptr());
                if let Some(mut k) = node.prev {
                    k.as_mut().next = node.next;
                }
                if let Some(mut m) = node.next {
                    m.as_mut().prev = node.prev;
                    self.length = self.length - 1;
                    return Some(node.value);
                } else {
                    return None;
                }
            } 
        } else {
            return None;
        }
    }

    pub fn get_ith_node(node : Option<NonNull<Node<T>>>, index : u32) -> Option<NonNull<Node<T>>> {
        match node {
            None => None,
            Some(next_ptr) => {
                match index {
                    0 => Some(next_ptr),
                    _ => Self::get_ith_node(unsafe{(*next_ptr.as_ptr()).next} , index - 1),
                }
            }
        }
    }

    pub fn get(&mut self, index : u32) -> Option<&T> {
        Self::get_ith_node(self.head, index).map(|ptr| {
            unsafe {
                &(*ptr.as_ptr()).value
            }
        })
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.delete_head().is_some() {}
    }
}

impl<T> Display for LinkedList<T> where T : Display, {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.head {
            Some(node) => write!(f, "{}", unsafe{node.as_ref()}),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T> where T : Display, {
    fn fmt(&self, f : &mut Formatter) -> fmt::Result {
        match self.next {
            Some(thing) => write!(f, "{}, {}", self.value, unsafe{thing.as_ref()}),
            None => write!(f, "{}", self.value),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn display() {
        let mut linked_list = LinkedList::new();
        linked_list.insert_at_head(3);
        linked_list.insert_at_tail(6);
        linked_list.insert_at_head(86);
        linked_list.insert_at_head(67);
        linked_list.insert_at_head(8);
        linked_list.insert_at_ith(3, 24);
        println!("{}", linked_list);
    }
}