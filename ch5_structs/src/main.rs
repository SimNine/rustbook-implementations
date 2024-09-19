fn main() {
    println!("Hello, world!");

    let str_literal = "hello";
    let mut str_var = String::from("hello");
    str_var.push_str(", world!");
}
