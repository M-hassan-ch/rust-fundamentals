fn main() {
    let mut s1 = String::from("Hello");
    write_operation(&mut s1); // borrowing mutable reference
    let length = read_operation(&s1); // borrowing immutable reference
    println!("{s1} length is: {length}")
}

fn write_operation(param: &mut String) {
    param.push_str(" World!");
}

fn read_operation(param: &String)-> usize {
   return param.len();
}
