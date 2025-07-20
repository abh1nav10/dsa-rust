// this is an implementation of the binary search search tree data structure
// efficient of finding values and insertion of data

use std::cmp::Ordering;

pub struct BinarySearchTree<T> where T: Ord, {
    value : Option<T>,
    left : Option<Box<BinarySearchTree<T>>>,
    right : Option<Box<BinarySearchTree<T>>>,
}

impl<T> Default for BinarySearchTree<T> where T : Ord, {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> BinarySearchTree<T>
where 
    T : Ord,
{
    pub fn new() -> Self {
        Self {
            value : None,
            left : None,
            right : None,
        }
    }

    pub fn search(&self, value : T) -> bool {
        match &self.value {
            Some(node) => {
                match node.cmp(&value) {
                    Ordering::Equal => {
                        true
                    }
                    Ordering::Greater => {
                        match &self.left {
                            Some(thing) => {
                                thing.search(value)
                            }
                            None => {
                                false
                            }
                        }
                    }
                    Ordering::Less => {
                        match &self.right {
                            Some(righty) => {
                                righty.search(value)
                            }
                            None => {
                                false
                            }
                        }
                    }
                }
            }
            None => {
                false
            }
        }
    } 

    pub fn insert(&mut self, value : T) {
        match &self.value {
            None => self.value = Some(value),
            Some(node) => {
                let mut target_node = if value < *node {
                    &mut self.left
                } else {
                    &mut self.right
                };
                match target_node {
                    Some(key) => {
                        key.insert(value);
                    }
                    None => {
                        let mut new = BinarySearchTree::new();
                        new.value = Some(value);
                        let alloc = Box::new(new);
                        *target_node = Some(alloc);
                    }
                } 
            }
        }
    }

    pub fn minimum(&self) -> Option<&T> {
        match &self.left {
            None => {
                match &self.value {
                    None => return None,
                    Some(key) => return Some(&key),
                }
            }
            Some(node) => {
                node.minimum()
            }
        }
    }

    pub fn maximum(&self) -> Option<&T> {
        match &self.right {
            None => {
                match &self.value {
                    None => return None,
                    Some(key) => return Some(&key),
                }
            }
            Some(node) => {
                node.maximum()
            }
        }
    }

    // floor function returns the largest value in the tree smaller than the given value
    pub fn floor(&self, value : T) -> Option<&T> {
        match &self.value {
            None => return None,
            Some(val) => {
                match val.cmp(&value) {
                    Ordering::Equal => {
                        Some(&val)
                    }
                    Ordering::Greater => {
                        match &self.left {
                            None => return None,
                            Some(key) => {
                                key.floor(value)
                            }
                        }
                    }
                    Ordering::Less => {
                        match &self.right {
                            None => return Some(&val),
                            Some(node) => {
                                let result = node.floor(value);
                                match result {
                                    None => return Some(&val),
                                    Some(_) => return result,
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn ceiling(&self, value : T) -> Option<&T> {
        match &self.value {
            None => return None,
            Some(key) => {
                match key.cmp(&value) {
                    Ordering::Equal => return Some(&key),
                    Ordering::Greater => {
                        match &self.left {
                            None => return Some(&key),
                            Some(node) => {
                                let result = node.ceiling(value);
                                match result {
                                    None => return Some(&key),
                                    Some(_) => return result,
                                }
                            }
                        }
                    }
                    Ordering::Less => {
                        match &self.right {
                            None => None,
                            Some(val) => {
                                val.ceiling(value)
                            }
                        }
                    }
                }
            }
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = &T>  /*  yields references to values of type T */ {
        BinarySearchTreeIterator::new(self)
    }
}

pub struct BinarySearchTreeIterator<'a, T> where T : Ord, {
    stack : Vec<&'a BinarySearchTree<T>>,
}

impl<T> BinarySearchTreeIterator<'_, T> where T : Ord, {
    pub fn new(tree : &BinarySearchTree<T>) -> BinarySearchTreeIterator<T> {
        let mut iter = BinarySearchTreeIterator {
            stack : vec![tree],
        };
        // we can push all the left side elements inside the stack right here..we can have a method for it as 
        // we will need it later
        iter.push_to_stack();
        return iter;
    }
    fn push_to_stack(&mut self) {
        while let Some(node) = &self.stack.last().unwrap().left {
        self.stack.push(node);
    }
}
}


impl<'a, T> Iterator for BinarySearchTreeIterator<'a, T> where T : Ord,  {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            return None;
        } else {
            let node = self.stack.pop().unwrap();
            if let Some(key) = &node.right {
                self.stack.push(key);
                self.push_to_stack();
            }
            return node.value.as_ref();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut tree = BinarySearchTree::default();
        tree.insert(7);
        tree.insert(98);
        tree.insert(67);
        tree.insert(54);
        tree.insert(34);
        tree.insert(29);
        tree.insert(19);
        tree.insert(93);
        tree.insert(77);
        assert_eq!(true, tree.search(67));
        assert_eq!(false, tree.search(11));
        let values : Vec<_> = tree.iter().collect();
        println!("{:?}", values);
    } 
}