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
        //TODO
        if self.root.is_none() {
            self.root = Some(Box::new(TreeNode::new(value)));
            return;
        }

        let current = self.root.as_mut().unwrap();
        if value < current.value {
            match current.left {
                // pay attention to the ref here, which is usually used to get the reference of the value
                Some(ref mut node) => {
                    node.insert(value);
                },
                None => {
                    current.left = Some(Box::new(TreeNode::new(value)));
                }
            }
        } else if value > current.value {
            match current.right {
                Some(ref mut node) => {
                    node.insert(value);
                },
                None => {
                    current.right = Some(Box::new(TreeNode::new(value)));
                }
            }
        } else {
            // if the value is equal to the current node, do nothing
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool 
    {
        //TODO
        let mut current = self.root.as_ref();
        let mut found = false;
        while current.is_some() {
            // println!("current: {}", current.unwrap().value);
            if value < current.unwrap().value {
                current = current.unwrap().left.as_ref();
            } else if value > current.unwrap().value {
                current = current.unwrap().right.as_ref();
            } else {
                found = true;
                break;
            }
        }
        return found;
    }

}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        if value < self.value {
            match self.left {
                Some(ref mut node) => {
                    node.insert(value);
                },
                None => {
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
            }
        } else if value > self.value {
            match self.right {
                Some(ref mut node) => {
                    node.insert(value);
                },
                None => {
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
            }
        } else {
            // if the value is equal to the current node, do nothing
        }
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


