use std::fmt::Display;

pub struct Stack<T> where T: Clone + Display {
    top:Option<Box<Node<T>>>,
}

#[derive(Clone)]
pub struct Node<T> where T: Clone + Display{
    pub prev: Option<Box<Node<T>>>,
    pub value: T
}

impl<T : Clone + Display> Node<T>{
    pub fn new(value:T)->Self{
       Node{
           prev: None,
           value
       }
    }
}

impl<T : Clone + Display> Stack<T> {
    pub fn new()->Self{
        Stack{
            top: None
        }
    }
    pub fn push(&mut self, item: T){
        let mut new_top:Node<T> = Node::new(item);
        if let Some(curr_top) = self.top.clone(){
            new_top.prev = Option::from(curr_top.clone());
        }
        self.top = Option::from(Box::from(new_top));
    }

    pub fn pop(&mut self) -> Option<Box<Node<T>>> {
        let top = self.top.clone();
        if top.is_none() {
           return None;
        }
            self.top = self.top.clone().unwrap().prev;
            return top;
    }

    pub fn peek(&self) -> Box<Node<T>> {
        return self.top.clone().unwrap();
    }

    pub fn swap(&mut self){
       let top = self.top.clone();
        if top.is_none() {
            return;
        }
        if let Some(mut prev) = self.top.clone().unwrap().prev{
            prev.prev = Option::from(top.unwrap().clone());
            self.top = Option::from(prev);
        }
    }

    pub fn print(&self){
        let mut curr = self.top.clone();
        while curr.is_some() {
            println!("{}", curr.clone().unwrap().value);
            curr=curr.clone().unwrap().prev;
        }
    }
}

#[cfg(test)]
mod stack {
    use crate::Stack;

    #[test]
    fn it_works() {
        let mut stack:Stack<i32>= Stack::new();
        stack.push(10);
        assert_eq!(stack.peek().value, 10);
        assert_eq!(stack.pop().unwrap().value, 10);
    }

    #[test]
    #[should_panic]
    fn it_panics(){
        let mut stack:Stack<i32>= Stack::new();
        stack.pop().unwrap();
    }

    #[test]
    fn swap(){
        let mut stack:Stack<i32>= Stack::new();
        stack.push(1);
        stack.push(2);

        stack.swap();

        assert_eq!(stack.peek().value, 1);
        assert_eq!(stack.pop().unwrap().value, 1);
        assert_eq!(stack.pop().unwrap().value, 2);

        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop().unwrap().value, 2);
        assert_eq!(stack.pop().unwrap().value, 1);

    }

}