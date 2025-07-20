// used to detect cycles in data structures like linked lists
// gets two pointers to the head and makes one move faster than the other, if there is a cycle the 
//fast pointer will eventually catch up with the slow pointer

use crate::data_structures::linked_list::LinkedList;

pub fn has_cycle<T>(linked_list : &LinkedList<T>) -> bool {
    let mut fast = linked_list.head;
    let mut slow = linked_list.head;

    while let (Some(fast_ptr), Some(slow_ptr)) = (fast, slow) {
        unsafe {
            slow = slow_ptr.as_ref().next;
            fast = fast_ptr.as_ref().next;
        }

        if let Some(fast_next) = fast {
            unsafe {
                fast = fast_next.as_ref().next;
            } 
        } else {
            return false;
        }
        if slow == fast {
            return true;
        }
    } 
    return false;
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut list = LinkedList::new();
        list.insert_at_head(5);
        list.insert_at_head(64);
        list.insert_at_head(714);
        list.insert_at_head(89);
        list.insert_at_head(94);
        list.insert_at_head(76);
        list.insert_at_head(67);
        list.insert_at_head(35);
        list.insert_at_head(58);
        list.insert_at_head(379);
        let check = has_cycle(&list);
        println!("{}", check);
    }
}