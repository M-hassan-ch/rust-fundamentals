fn main() {
    //fixed size
    let str_var = "Hello World";
    let tuple_var = ("Hassan", 18);
    let int_var = 100;
    let float_var = 100.23;
    let array = [1, 2, 3, 4];
    let array_length = array.len();

    // dynamic sized
    let str_var2 = String::from("Hello");
    let vector_var = vec![1, 2, 3, 4];

    for item in vector_var {
        println!("Item in vector {item}")
    }

    let mut i: usize = 0;
    while i < array_length {
        println!("Item in array {}", array[i]);
        i += 1;
    }

    if int_var == 100 {
        println!("int_var value is equal to 100");
    }
}
