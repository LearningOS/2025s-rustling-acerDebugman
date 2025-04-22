/*
	heap
	This question requires you to implement a binary heap function
*/
#![feature(core_intrinsics)]

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.count += 1;
        if self.items.len() > self.count {
            self.items[self.count] = value;    
        } else {
            self.items.push(value);
        }
        if self.count == 1 {
            return ;
        }
        self.sift_up(self.count);
    }

    fn sift_up(&mut self, idx: usize) {
        if idx == 1 {
            return;
        }
        let parent = self.parent_idx(idx);
        let best_child = self.best_child_idx(parent);
        if (self.comparator)(&self.items[parent], &self.items[best_child]) {
            return ;
        }
        self.items.swap(best_child, parent);
        self.sift_up(parent);
        // let (left, right) = (self.left_child_idx(parent), self.right_child_idx(parent));
        // match ((self.comparator)(&self.items[parent], &self.items[left]), (self.comparator)(&self.items[parent], &self.items[right])) {
        //     (true, true) => return,
        //     (false, true) => {
        //         self.items.swap(parent, left);
        //         // self.sift_up(parent);
        //     },
        //     (true, false) => {
        //         self.items.swap(parent, right);
        //         // self.sift_up(parent);
        //     },
        //     (false, false) => {
        //         let best_child = if (self.comparator)(&self.items[left], &self.items[right]) {
        //             left
        //         } else {
        //             right
        //         };
        //         self.items.swap(parent, best_child);
        //         // self.sift_up(parent);
        //     },
        // }
        // self.sift_up(parent);
    }

    fn sift_down(&mut self, idx: usize) {
        // let a = 10.0;
        // let k = unsafe { log2f32(a) };
        if !self.children_present(idx) {
            return ;
        }
        let parent = idx;
        let best_child = self.best_child_idx(parent);
        if (self.comparator)(&self.items[parent], &self.items[best_child]) {
            return; 
        }
        self.items.swap(parent, best_child);
        self.sift_down(best_child);
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn best_child_idx(&self, idx: usize) -> usize {
        let (left, right) = (self.left_child_idx(idx), self.right_child_idx(idx));
        if right > self.count {
            return left;
        }
        if (self.comparator)(&self.items[left], &self.items[right]) {
            left
        } else {
            right
        }
    }
    //应该改为best, smallest 还是 biggest 都是comparator决定的
    fn smallest_child_idx(&self, idx: usize) -> usize {
        self.best_child_idx(idx)
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Clone + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }
        let rs = Some(self.items[1]);
        self.items.swap(1, self.count);
        self.count -= 1;
        self.sift_down(1);
        rs
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        // println!("{:?}", heap.items);
        assert_eq!(heap.next(), Some(2));
    }
}