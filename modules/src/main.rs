use crate::kitchen::veg_kitchen::hosting;

//telling compiler that this is a module and to include this file
pub mod kitchen;
pub mod test_restaurant
{
    pub fn front_desk(){
        println!("Front desk is open");
    }
}


fn main() {
    println!("Hello, world!");
    //can be run only if function is public
    hosting::add_to_waitlist();
}
