// use core::fmt;
// use std::{collections::btree_map::Values, fmt::{Display, Formatter, write}};

use std::boxed;
// impl<T> Display for Node<T> {
//     fn fmt(&self, f:&mut Formatter) -> Result {
        
//     }
// }   
use std::collections::VecDeque;
use std::fmt::Display;
use std::ops::Not;

pub struct Node<T> {
    data:T,
    left:Option<Box<Node<T>>>,
    right:Option<Box<Node<T>>>
}  

impl<T:Ord + Display + Not + Copy> Node<T> {
    
    pub fn new(data:T) -> Self {
        Node {
            data,
            left:None,
            right:None
        }
    }

    fn insert(&mut self, value:T){
        if value < self.data {

            match self.left {
                Some(ref mut left_node ) => left_node.insert(value),
                None => self.left = Some(Box::new(Node::new(value)))                
            }

        } else if value > self.data {

                match self.right {
                    Some(ref mut right_node)  => right_node.insert(value),
                    None => self.right = Some(Box::new(Node::new(value)))                   
            }

        }
    }

    pub fn inorder(&self){
        if let Some(ref left) = self.left {
            left.inorder();
        } 
        print!("-> {}",self.data);

        if let Some(ref right) = self.right {
            right.inorder();
        }
    }

    pub fn postorder(&self) {
        if let Some(ref left) = self.left {
            left.postorder();
        }

        if let Some(ref right) = self.right {
            right.postorder();
        }

        print!("-> {}",self.data);
    }

    pub fn preorder(&self) {

        print!("-> {}",self.data);

        if let Some(ref left) = self.left {
            left.preorder();
        }

        if let Some(ref right) = self.right {
            right.preorder();
        }
    }

    pub fn level_order(&self) {
        let mut queue:VecDeque<&Node<T>> = VecDeque::new();

        queue.push_back(self);

        while let Some(node) = queue.pop_front() {
            print!("-> {}",node.data);

            if let Some(ref left) = node.left {
                queue.push_back(left);
            }

            if let Some(ref right) = node.right {
                queue.push_back(right);
            }
        }

    }

    pub fn delete(node: Option<Box<Node<T>>>, value: T) -> Option<Box<Node<T>>> {

        match node {
            None => None,
            Some(mut boxed_node) => {
                if value < boxed_node.data {
                    boxed_node.left = Node::delete(boxed_node.left, value);
                    Some(boxed_node)
                } else if value > boxed_node.data {
                    boxed_node.right = Node::delete(boxed_node.right, value);
                    Some(boxed_node)
                } else {
                    
                    // Node found — now handle deletion cases
                    match (boxed_node.left.take(), boxed_node.right.take()) {
                        (None, None) => None, // no children
                        (Some(left_child), None) => Some(left_child), // only left
                        (None, Some(right_child)) => Some(right_child), // only right
                        (Some(left_child), Some(mut right_child)) => {
                            // find min in right subtree
                            let min = Node::find_min(&right_child);
                            boxed_node.data = min;
                            right_child = Node::delete(Some(right_child), min).unwrap();
                            boxed_node.left = Some(left_child);
                            boxed_node.right = Some(Box::new(right_child));
                            Some(boxed_node)
                        }
                    }
                }
            }
        }
    }

    fn find_min(node: &Node<T>) -> T {
        match node.left {
            Some(ref left) => Node::find_min(left),
            None => node.data,
        }
    }
}

    



fn main(){
    let mut root = Node::new(10);
    root.insert(4);
    root.insert(8);
    root.insert(7);
    root.insert(2);
    root.insert(1);
    root.insert(9);

    println!("Inorder Traversal ...");

    root.inorder();
    println!();
    root.postorder();
    println!();

    root.preorder();
    println!();

    root.level_order();
    println!();
}   