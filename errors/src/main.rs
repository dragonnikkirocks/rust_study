
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    println!("Errors");

   let file_open_result = File::open("test.txt");
   let _file_opened = match file_open_result{
    Ok(file) => file,
    Err(error) => match error.kind(){
        ErrorKind::NotFound => {
            match File::create("test.txt"){
                Ok(fc) => fc,
                Err(_error) => panic!("Not able to create new file")
            }
        },
        _=> panic!(" Error!!")
    },
   };

   //PREFER EXPECT INSTEAD OF UNWRAP
//    let greeting_file = File::open("hello.txt")
//         .expect("hello.txt should be included in this project");


//let test_file = File::open("testw.txt").unwrap();


}

use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
   //? is used to propagate errors - here it will take the Result value if its Ok, else will propage the error
   //? cannot be used in fn main- as it will return io::Error if anything fails, but in fn main, return type is void
    File::open("hello.txt")?.read_to_string(&mut username)?;

    

    Ok(username)
}
