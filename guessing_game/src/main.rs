use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guessing game");

    //loop is like while loop
    loop
    {
    let secret_number = rand::thread_rng().gen_range(1..=10);
    println!("enter your guess");
    
    let mut guess_number = String::new();
    io::stdin().read_line(&mut guess_number).expect("Not able to read line");

    //trim is for stripping white spaces before and after input
    //parse is method for converting string to numbers
    //parse will convert only those strings which are logically convertible
    //otherwise it wont convert and returns an error
    //example "<<<" as input to parse will cause it to return error
    //here this is caught in catch all error Err(_)
    let guess_number: u32 = match guess_number.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("wrong input");
            continue;
        },
    };

 
    println!("The number you gave is {}", guess_number);

    //match is like switch case
    match guess_number.cmp(&secret_number){
        Ordering::Less => println!("you guessed a lower number"),
        Ordering::Greater => println!("you guessed a higher number"),
        Ordering::Equal => {
            println!("you guessed correctly !!");
            println!("The number you gave is {}", guess_number);
            println!("The secret number  is {}", secret_number);
            break;
        }
    }
x^x

   }


}
