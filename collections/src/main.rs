fn main() {
    println!("Rust collections");

    let mut vector_1 : Vec<i32> = Vec::new();
    vector_1.push(1);
    vector_1.push(2);
    let vector_2 = vec![1, 2, 3, 4, 5];


    let third_element: Option<&i32> = vector_2.get(2);
    match third_element {
        Some(value) => println!("The third element is {}", value),
        None => println!("Third element does not exist"),
    }

        let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}"); //format! macro is similar to println! but it returns a String instead of printing


    println!("The formatted string is: {}", s);


    let hello = "Здравствуйте";
let answer = &hello[0];
}
