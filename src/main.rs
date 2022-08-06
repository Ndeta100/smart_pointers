fn main() {
    let s = vec!["Germany", "Estonia"];
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
}
#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}
