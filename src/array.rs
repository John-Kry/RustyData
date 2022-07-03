use std::collections::HashMap;
use std::fmt::Display;

pub struct Array<T> where T: Clone + Display {
    length: usize,
    items: HashMap<usize, T>,
}

impl<T> Array<T> where T: Clone + Display {
    pub fn new()->Self{
       Array{
           length: 0,
           items: HashMap::new(),
       }
    }

    pub fn push(&mut self, item: T){
        self.items.insert(self.length, item);
        self.length +=1;
    }

    pub fn unshift(&mut self, item: T){
        let length = self.length;
        let mut i = 1;
        while self.items.get(&0).is_some(){
            let to_move = self.items.get(&(length - i)).expect("exists");
            self.items.insert(length-i+1, to_move.clone());
            self.items.remove(&(length - i));
            i +=1;
        }
        self.items.insert(0, item);
        self.length +=1;
    }


    pub fn get(&self, index: usize) -> Option<T> {
        if index < self.length {
            return self.items.get(&index).cloned();
        }
        return None;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.length>0{
            self.length -= 1;
            return self.items.get(&(self.length)).cloned()
        }
        return None;
    }
    pub fn length(&self) -> usize {
        return self.length;
    }

    pub fn print(&self){
        for i in 0..self.length {
            println!("{}",i);
        }
    }
}

#[cfg(test)]
mod stack {
    use crate::array::Array;

    #[test]
    fn it_works() {
        let mut arr:Array<i32> = Array::new();
        assert_eq!(arr.length, 0);
        arr.push(10);
        assert_eq!(arr.length, 1);

        arr.push(20);
        assert_eq!(arr.get(0).unwrap(), 10);
        assert_eq!(arr.get(1).unwrap(), 20);
        let result= arr.pop().unwrap();
        assert_eq!(result, 20);
    }

    #[test]
    #[should_panic]
    fn pop_nothing(){
        let mut arr:Array<i32> = Array::new();
        arr.pop().unwrap();
    }
    #[test]
    #[should_panic]
    fn get_oob(){
        let mut arr:Array<i32> = Array::new();
       arr.get(1).unwrap();
    }
    #[test]
    #[should_panic]
    fn get_oob_for_real(){
        let mut arr:Array<i32> = Array::new();
        arr.push(0);
        arr.pop();
        arr.get(0).unwrap();
    }
    #[test]
    fn unshift_works() {
        let mut arr:Array<i32> = Array::new();
        arr.unshift(0);
        arr.unshift(99);


        assert_eq!(arr.get(0).unwrap(), 99);
        assert_eq!(arr.get(1).unwrap(), 0);
        arr.unshift(100);
        assert_eq!(arr.get(0).unwrap(), 100);
        assert_eq!(arr.get(1).unwrap(), 99);
        assert_eq!(arr.get(2).unwrap(), 0);
        assert_eq!(arr.length, 3);
    }

}