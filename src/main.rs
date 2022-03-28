fn main() {
    let s1 = String::from("hello");
    hello(&s1);
    let x = 5;
    number(x);
    number(x);
}
fn number(x: i32) {
    print!("{}", x);
}
fn hello(s: &String) {
    println!("the number is {}", s);
}
