use std::io;
#[derive(Debug)]
struct UserLib{
    name:String,
email: String,
age: u16,
validity:bool,
}


//Multiple implementations of the same struct
impl UserLib{
        //Constructor - associated function
        fn new(name:String,email:String,age:u16,validity:bool)->UserLib{
            UserLib{name,email,age,validity}
        }
}
impl UserLib{

    //Method - methods are functions that are associated with a struct, always take self as first parameter
    fn email_update(&mut self, changed_email:String){
        self.email = changed_email;
        println!("Email updated to {}",self.email);
    }
}
fn main(){
    println!("Structs");
   let user1 = UserLib{
    name: String::from("Nikita"),
    email: String::from("nikita@gmail.com"),
    age: 29,
    validity: true,
   };

   let user2: UserLib = UserLib {
       name: String::from("Shebin"),
       email: String::from("shebin@gmail.com"),
       age: 35,
       validity: true,
   };

   let user3 =UserLib{
    name:String::from("Noah"),
    email:String::from("noah@gmail.com"),
    age:0,
    validity: false,};
println!("User 1: name: {}, email: {}, age: {}, validity: {}", user1.name, user1.email, user1.age, user1.validity);
println!("User 2: name: {}, email: {}, age: {}, validity: {}", user2.name, user2.email, user2.age, user2.validity);
println!("User 3: name: {}, email: {}, age: {}, validity: {}", user3.name, user3.email, user3.age, user3.validity);


let user1_copy = UserLib{
    name:String::from("NikitaClone"),
    ..user1
};

println!("User 1 Copy: name: {}, email: {}, age: {}, validity: {}", user1_copy.name, user1_copy.email, user1_copy.age, user1_copy.validity);


//Using a constructor 
let mut user4 = UserLib::new("Test".to_string(),"test@gmail.com".to_string(),27,true);

println!("User 4: name: {}, email: {}, age: {}, validity: {}", user4.name, user4.email, user4.age, user4.validity);

user4.email_update(String::from("changed_email@gmail.com"));
println!("User 4 changed: name: {}, email: {}", user4.name, user4.email);


// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
println!("User 4  {:#?}", user4);
}