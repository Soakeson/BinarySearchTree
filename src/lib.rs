use std::cmp::Ordering;
use std::fmt::Display;

#[derive(Debug)]
pub struct Node<T: Ord + Copy> {
    pub value: T,
    pub right: Link<T>,
    pub left: Link<T>
}

#[derive(Debug)]
pub struct Link<T: Ord + Copy> (Option<Box<Node<T>>>);

pub struct BinarySearchTree<T: Ord + Copy>    
{
    pub root: Link<T>
}

impl<T> Node<T> 
where 
    T: Ord + Copy 
{
    fn new(value: T) -> Node<T> {
        Node {
            value: value,
            left: Link(None),
            right: Link(None),
        }
    }
}

impl<T> BinarySearchTree<T> 
where 
    T: Ord + Copy 
{
    pub fn new() -> BinarySearchTree<T> {
        BinarySearchTree { root: Link(None) }
    }
}


impl<T> Link<T>
where 
    T: Ord + Copy + Display 
{
    pub fn insert(&mut self, value: T ) {
        match &mut self.0 {
            None => self.0 = Some(Box::new(Node::new(value))),
            Some(ref mut node) => {
                match value.cmp(&node.value) {
                    Ordering::Equal => (),
                    Ordering::Greater => node.right.insert(value),
                    Ordering::Less => node.left.insert(value)
                }
            }
        }
    }

    pub fn search(&self, querry: &T) -> bool {
        let mut current = self;
        while current.0.is_some() {
            if current.0.as_ref().unwrap().value != *querry {
                if current.0.as_ref().unwrap().value > *querry {
                    current = &current.0.as_ref().unwrap().left;
                } else {
                    current = &current.0.as_ref().unwrap().right;
                }
            } else {
                return true
            }
        }
        false
    }

    pub fn min(&self) -> Option<T> {
        if self.0.is_some() {
            let mut current = self;
            while current.0.as_ref().unwrap().left.0.is_some() {
                current = &current.0.as_ref().unwrap().left
                }
                Some(current.0.as_ref().unwrap().value)
            } else {
                None
            }
        }
    
    pub fn max(&self) -> Option<T> {
        if self.0.is_some() {
            let mut current = self;
            while current.0.as_ref().unwrap().right.0.is_some() {
                current = &current.0.as_ref().unwrap().right
                }
                Some(current.0.as_ref().unwrap().value)
            } else {
                None
            }
        }

    pub fn delete(&mut self, target: &T) {
        let mut current = self;
        while let Some(ref mut node) = current.0 {
            match target.cmp(&node.value) {
                Ordering::Less => current = &mut current.0.as_mut().unwrap().left,
                Ordering::Greater => current = &mut current.0.as_mut().unwrap().right,
                Ordering::Equal => match (node.left.0.as_mut(), node.right.0.as_mut()) {
                    (None, None) => current.0 = None,
                    (Some(_), None) => current.0 = node.left.0.take(),
                    (None, Some(_)) => current.0 = node.right.0.take(),
                    (Some(_), Some(_)) => {
                        let successor = node.right.min();
                        current.delete(&successor.unwrap());
                        current.0.as_mut().unwrap().value = successor.unwrap();
                    }
                }
            }
        }
    }

    pub fn display(&self) {
        match &self.0 {
            None => (),
            Some(node) => {
                node.left.display();
                println!("{}", node.value);
                node.right.display();
            }
        }   
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_test() {
        let mut tree: BinarySearchTree<i32> = BinarySearchTree::new();
        tree.root.insert(12);
        tree.root.insert(14);
        tree.root.insert(15);
        tree.root.insert(13);
        tree.root.insert(10);
        assert_eq!(true, tree.root.search(&12));
        assert_eq!(false, tree.root.search(&11));
    }

    #[test]
    fn delete_test() {
        let mut tree: BinarySearchTree<&str> = BinarySearchTree::new();
        tree.root.insert("hello");
        tree.root.insert("there");
        tree.root.insert("Kenobi");
        tree.root.delete(&"hello");
        assert_eq!(false, tree.root.search(&"hello"))
    }

    #[test]
    fn min_max_test() {
        let mut tree: BinarySearchTree<i32> = BinarySearchTree::new();
        tree.root.insert(12);
        tree.root.insert(14);
        tree.root.insert(15);
        tree.root.insert(13);
        tree.root.insert(10);
        tree.root.insert(-12);
        tree.root.insert(-4);
        assert_eq!(Some(15), tree.root.max());
        assert_eq!(Some(-12), tree.root.min());
    }
}
