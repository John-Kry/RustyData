use crate::stack::Stack;

mod stack;
mod array;

fn main() {
    println!("Hello, world!");
    let mut test_stack: Stack<i32> = stack::Stack::new();
    test_stack.push(1);
    test_stack.push(2);
    test_stack.push(3);
    test_stack.print();

    let result = test_stack.peek().value;
    println!("{}" , result);
    if let Some(result2)= test_stack.pop(){
        println!("{}", result2.value);
    }

    let mut pog_stack: Stack<String> = stack::Stack::new();
    pog_stack.push(String::from("Poggers"));
    pog_stack.push(String::from("Champ"));
    pog_stack.print()


}
