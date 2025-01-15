fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    println!("Hello, world!");

    let f = 4;
    let f = f*2;
    {
        let f = 6;
        println!("inner loop {f}");
    }
    println!("{f}");
print_type_of(&f);
print_type_of(&main);

//TUPLE
let mut tup: (i32, f64, u8) = (500, 6.4, 1);
println!("third val of tup is {} ", tup.2);

//I can change tuple value like this after defining it once
tup.2 = 5;
println!("third val of tup after change is {} ", tup.2);

//this is possible too
let mut tup2:(i32,u8);
tup2 = (1,2);
//this not possible - both needs to be defined together 
//tup2.1 = 6;
//tup2.0 = 5;
//this not possible  - i.e. change type
// tup2.1 = "string blah";
//this not possible - i.e. increase size after declaration
//tup2.3 = 5;


//ARRAY
let a: [i32; 5] = [1, 2, 3, 4, 5];
//array -everything has to be of same type, size cannot be changed, square brackets

print_labeled_measurement(5,'f');


}

//type of function arguments needs to be specified mandatorily
fn print_labeled_measurement(value: i32, unit_label:char) {
    println!("The measurement is: {value}{unit_label}");
}