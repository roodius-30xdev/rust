// use core::fmt;
// use std::{collections::btree_map::Values, fmt::{Display, Formatter, write}};

// impl<T> Display for Node<T> {
//     fn fmt(&self, f:&mut Formatter) -> Result {
        
//     }
// }   
use std::collections::VecDeque;
use std::fmt::Display;

pub struct Node<T> {
    data:T,
    left:Option<Box<Node<T>>>,
    right:Option<Box<Node<T>>>
}  

impl<T:Ord + Display> Node<T> {
    
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