/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T: PartialEq + PartialOrd + Clone> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T: PartialEq + PartialOrd + Clone> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T: PartialEq + PartialOrd + Clone> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T: PartialEq + PartialOrd + Clone> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: PartialEq + PartialOrd + Clone> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    fn get_ith_node_ptr(&mut self, node: Option<NonNull<Node<T>>>, idx: u32) -> Option<NonNull<Node<T>>> {
        match node {
            None => None,
            Some(ptr) => match idx {
                0 => Some(ptr),
                _ => self.get_ith_node_ptr(unsafe { (*ptr.as_ptr()).next } , idx - 1)
            },
        } 
    }

    pub fn remove(&mut self, idx: u32) -> Option<NonNull<Node<T>>> {
        if self.start.is_none() || (idx + 1) > self.length {
            return None;
        }
        let node = self.start;
        if idx == 0 {
            self.start = unsafe { (*node.unwrap().as_ptr()).next };
            unsafe { (*node.unwrap().as_ptr()).next = None; }
            if self.start.is_none() {
                self.end = None;
            }
            self.length -= 1; 
            return node;
        }
        let pre = self.get_ith_node_ptr(node, idx - 1);
        let node = self.get_ith_node_ptr(node, idx);
     
        if node.is_some() {
            unsafe { (*pre.unwrap().as_ptr()).next = (*node.unwrap().as_ptr()).next };
            if self.end.unwrap().as_ptr() == node.unwrap().as_ptr() {
                // self.end = unsafe { (*node.unwrap().as_ptr()).next };
                self.end = pre;
            }
            unsafe { (*node.unwrap().as_ptr()).next = None };
            self.length -= 1; 
        }

        node
    }
    
	pub fn merge(mut list_a:LinkedList<T>, mut list_b:LinkedList<T>) -> Self
	{
        let mut newlst = LinkedList::<T>::new();
        while list_a.length > 0 && list_b.length > 0 {
            let afirst = list_a.start;
            // println!("list a val: {:?}, {:?}", afirst.unwrap().as_ptr(), unsafe { (*afirst.unwrap().as_ptr()).next });
            let bfirst = list_b.start;

            if unsafe { (*afirst.unwrap().as_ptr()).val <= (*bfirst.unwrap().as_ptr()).val } {
                let afirst = list_a.remove(0);
                newlst.add(unsafe { (*afirst.unwrap().as_ptr()).val.clone() });
            } else {
                let bfirst = list_b.remove(0);
                newlst.add(unsafe { (*bfirst.unwrap().as_ptr()).val.clone() });
            }
            
        }        

        if list_a.length > 0 {
            unsafe { (*newlst.end.unwrap().as_ptr()).next = list_a.start; };
            newlst.end = list_a.end;
            newlst.length += list_a.length;
            list_a.length = 0;
            list_a.start = None;
            list_a.end = None;
        }

        if list_b.length > 0 {
            unsafe { (*newlst.end.unwrap().as_ptr()).next = list_b.start; };
            newlst.end = list_b.end;
            newlst.length += list_b.length;
            list_b.length = 0;
            list_b.start = None;
            list_b.end = None;
        }

        newlst
	}
}

impl<T: PartialEq + PartialOrd + Clone> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T: PartialEq + PartialOrd + Clone> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn list_remove() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        list_str.remove(2);
        // println!("Linked List is {}", list_str);
        // println!("Linked List is {:?}", list_str);
        assert_eq!(2, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}