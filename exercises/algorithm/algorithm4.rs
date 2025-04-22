/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        fn _insert<T: Ord>(node: &mut Option<Box<TreeNode<T>>>, val: T) {
            if node.as_ref().unwrap().value == val {
                return ;
            } else if val < node.as_ref().unwrap().value {
                if node.as_ref().unwrap().left.is_none() {
                    node.as_mut().unwrap().left = Some(Box::new(TreeNode::new(val)));
                    return ;
                }
                _insert(&mut node.as_mut().unwrap().left, val); 
            } else {
                if node.as_ref().unwrap().right.is_none() {
                    node.as_mut().unwrap().right = Some(Box::new(TreeNode::new(val)));
                    return ;
                }
                _insert(&mut node.as_mut().unwrap().right, val); 
            }
        }
        if self.root.is_none() {
            self.root = Some(Box::new(TreeNode::new(value)));
            return ;
        }
        _insert(&mut self.root, value);
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        fn _search<T: Ord>(node: &Option<Box<TreeNode<T>>>, val: &T) -> bool {
            if node.is_none() {
                return false;
            }
            if node.as_ref().unwrap().value == *val {
                return true;
            }
            if _search(&node.as_ref().unwrap().left, val) {
                return true;
            }
            if _search(&node.as_ref().unwrap().right, val) {
                return true;
            }
            
            false
        }
        _search(&self.root, &value)
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


