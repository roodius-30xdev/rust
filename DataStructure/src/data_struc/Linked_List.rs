use bitflags::iter;



pub struct Node<T> {
    data:T,
    next:Option<Box<Node<T>>>      // next node
}

pub struct LinkedList<T> {
    head : Option<Box<Node<T>>>,     // linked contain every node
}

pub struct Iter<'a, T> {
    next:Option<&'a Node<T>>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a Node<T>; // we return a Node

    fn next(&mut self) -> Option<Self::Item> {
        // map over the next node
        self.next.map(|node|{
            self.next = node.next.as_ref().map(|next_node| &**next_node);
            return node
        })
    }
  
}


impl<T> LinkedList<T>{

    // linked list created
    pub fn new(&mut self) -> Self {
        return LinkedList {
            head:None
        }
    }

    // push front
    pub fn push_back(&mut self, value:T){
        let newnode = Box::new(Node {
            data:value,
            next:self.head.take(),
        });

        self.head = Some(newnode);
    }

    // pop_front
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let node = *node;
            self.head = node.next;
            return node.data
        })
    }

    // isEmpty
    pub fn isempty(&self) -> bool { 
        return self.head.is_none()
    }

    // top
    pub fn top(&self) -> Iter<'_, T> {
         Iter {
            next: self.head.as_ref().map(|node| return  &**node),
        }
    }

    // for print
    pub fn print(&self) ->Iter<'_,T> {
        Iter {
            next:self.head.as_ref().map(|node|&**node)
        }
    }

    
}



fn main(){


}