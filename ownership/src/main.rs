fn main() {
    let mut s1: String = String::from("Hello!");
    let length: usize;
    (length, s1) = calculate_length(s1);  // Here ownership of s1 transferred to param but ownership returned back to s1 as we returned both string length and its reference
    println!("Length of {s1} is {length}");
}

fn calculate_length(param: String) -> (usize, String) {
    return (param.len(), param);
}
