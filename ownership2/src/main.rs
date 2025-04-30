fn main() {
    println!("Ownership model in Rust!");
     //Rust uses ownership model to handle memory management 
    //pro :
    //- control over memory, error free, faster runtime, smaller programm size
    //cons:
    //- slower write time, steeper learning curve

    //OWNERSHIP RULES
//1. Each value in Rust will have one owner
// 2. variable can have only one owner
// 3. when owner goes out of scope, value will be dropped

//s is allocated in heap
//dynamically
// the same if done in cpp - new keyword needed - we have to take care of 
//deallocation of memory
let s = String::from("HiNik");

let mut s2 = s; //s is moved to s2

let s3 = s2.clone(); //s2 is cloned to s3

println!("s2: {}, s3: {}", s2, s3); //s2 is valid here

taking_ownership_of_string(s3); //s3 is moved to the function
//this is not valid as s3 is moved to the functipon
//basically rust does what cpp std::move does for anytging tghats stored dynamically in heap
// as strinf is stored in heap, passing it to a function, automatically moves it
//println!("After function call to taking_ownership_of_string, s3: {}", s3); //s2
 let int_examople =8;

 // passing a int to a function
 //here the int is copied to the function, as int is stored in stack
passing_a_int(int_examople);
println!("After function call to passing_a_int, int_example is still valid in main: {}", int_examople);

println!("The length of the string is {}", calculate_length(&mut s2)); //passing 
//passing second mutable reference wont work - here its passed as immutablöe reference
println!("The length of the string is {}", calculate_length2(& mut s2)); 


//There can be multiple immutable references or one mutable reference
//but not more than one mutable reference





}


fn taking_ownership_of_string(s:String){
    println!("The ownershio of string is givent to this functkion {}",s);
}

fn passing_a_int(s:u32){
    println!("The ownershio of int is not givent to this functkion. Rather a copy is made and passed {}",s);
}

//References are when you want to pass a variable to a function
//without taking ownership of it
//here refence to string is passed - this is called borrowing
// Passing in a refernces to a function is called borrowing
// as we are not taking ownership of the variable, just borrowing 
//references are immutable by default
// to make it mutable, we have to use mut keyword
fn calculate_length(s:& mut String) ->usize{
    s.push_str("Nikkiii");

s.len()
}

//there can be multiple immutable references to a variable
// but only one mutable reference
// this is called aliasing
// this is to provent data race conditions where multiplle threads try to access
// same variable at the same time 
// this is not allowed in rust
// i.e. calculate_length2() and calculate_length() if called at same time from different threads
// it will try to access the same variable at the same time
// to prevent this, rust does not allow multiple mutable references
fn calculate_length2(s:&String) ->usize{
    //s.push_str("Nikkiii"); //this will not work as s is immutable
    s.len()
}




//Dangling references
// fn dangling_reference_example() -> &String{
//    //below code will not work as s is dropped after the function call 
//    //and the reference to s is returned which is left dangling i.e. ref points to a non exisiting var
//    // let s = String::from("Dangling ref example");
//    // s
// }
