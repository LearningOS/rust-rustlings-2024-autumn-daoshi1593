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
    fn insert(&mut self, value: T)
    where
        T: Ord + Clone,
    {
        let new_node = TreeNode::new(value.clone());
        match self.root {
            Some(ref mut node) => {
                let mut current = node;
                loop {
                    match current.value.cmp(&value) {
                        Ordering::Less => match current.right {
                            Some(ref mut right) => {
                                current = right;
                            }
                            None => {
                                current.right = Some(Box::new(new_node));
                                break;
                            }
                        },
                        Ordering::Greater => match current.left {
                            Some(ref mut left) => {
                                current = left;
                            }
                            None => {
                                current.left = Some(Box::new(new_node));
                                break;
                            }
                        },
                        Ordering::Equal => {
                            break;
                        }
                    }
                }
            }
            None => {
                self.root = Some(Box::new(new_node));
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        match self.root {
            Some(ref node) => {
                let mut current = node;
                loop {
                    match current.value.cmp(&value) {
                        Ordering::Less => match current.right {
                            Some(ref right) => {
                                current = right;
                            }
                            None => {
                                return false;
                            }
                        },
                        Ordering::Greater => match current.left {
                            Some(ref left) => {
                                current = left;
                            }
                            None => {
                                return false;
                            }
                        },
                        Ordering::Equal => {
                            return true;
                        }
                    }
                }
            }
            None => false,
        }
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
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}
