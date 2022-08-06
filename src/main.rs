use std::ops::Deref;

fn main() {
    let s = vec!["Germany", "&Estonia"];
    let b = Box::new(s);
    println!("b={:?}", b);
    let list = List::Cons(
        3,
        Box::new(List::Cons(
            10,
            Box::new(List::Cons(12, Box::new(List::Nil))),
        )),
    );
    println!("{:?}", list);
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
    //Using Box<T> Like a reference
    let t = 5;
    let u = Box::new(x);
    assert_eq!(5, t);
    assert_eq!(5, *u);
    //Using our own MyBox implemetation
    let h = 5;
    let k = MyBox::new(h);
    assert_eq!(5, h);
    assert_eq!(5, *k);
    fn hello(m: &str) {
        println!("Hello, {}", m);
    }
    let m = MyBox::new(String::from("Ndeta"));
    hello(&m);
    let c = CustomSmartPointer {
        data: String::from("Do stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("Do stuff ott=her stuff"),
    };
    println!("Custome Smart pointer created.");
}
struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping custom smart pointer with data {}", self.data);
    }
}
//Defining our own smart pointer
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
//Treating a type like a reference by implementing the Deref trait
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}
#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
