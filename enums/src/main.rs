
#[derive(Debug)]
enum IpAddress{
    V4(String),
    V6(String),
}
 //Pattern matching
 enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
//remember to derive Debug for each enum
//to be able to print the enum
#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
    Arizona,

}

fn add_one(x:Option<i32>) ->Option<i32>{
    match x{
        Some(i) => Some(i+1),
        //This None is needed
        // or use _ to consider all other cases
        None =>None,
    }
}
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        //state binds to the UsState stored in the Quarter variant
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}
impl IpAddress{
    fn new(ip_str:String) -> IpAddress{
        if ip_str.len() >15{
            IpAddress::V6(ip_str)
        }
        else{
            IpAddress::V4(ip_str)
        }
    }

    fn change_address(&mut self, new_ip_string:String){
        *self =  IpAddress::new(new_ip_string);
    }
}

fn main() {
    println!("Enums");

    let mut ip1 = IpAddress::new("10.4.2.1".to_string());
    let _ip2 = IpAddress::new("2001:0db8:85a3:0000:0000:8a2e:0370:7334".to_string());
     
    println!("ip1: {:?}", ip1);
    ip1.change_address("2001:0db8:85a3:0000:0000:8a2e:0370:7334".to_string());
    println!("changed ip1: {:?}", ip1);


    //Option enum
    let some_number = Some(5);
    let some_char = Some('e');
    
    //here type is defined as i32, as Rust cannot infer it
    let absent_number: Option<i32> = None;

  let c = Coin::Quarter(UsState::Alaska);
    println!("value in cents: {}", value_in_cents(c));
    

   
   println!("add_one: {:?}", add_one(Some(5)));

}
