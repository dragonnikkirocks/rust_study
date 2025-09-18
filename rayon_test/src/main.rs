#[warn(unused_assignments)]
fn main() {
    println!("Closure test");
    //https://github.com/locka99/cpp-to-rust-book/blob/master/09_features_compared/closures.md
    let mut x = 100;
{
  let square = || x * x;
  println!("square = {}", square());
}
x = 200;

let mut v1 = 10.0;
let v2 = 2.0;
let multiply = move || v1 * v2; //we can move ownership to the closure here - not possible in cpp
let sum = |x: &f64, y: &f64| x + y;
println!("multiply {}", multiply());
println!("sum {}", sum(&v1, &v2));
v1 = 99.0;
println!("multiply {}", multiply());
println!("sum {}", sum(&v1, &v2));

let z = 101;
let add: fn(i32, i32) -> i32 = |x: i32, y:i32| {x + y};  //will compile

//following will have error: closures can only be coerced to `fn` types if they do not capture any variables
// the fn function should implement trait Fn to allow closure to capture any variables
//let add: fn(i32, i32) -> i32 = |x: i32, y:i32| {x + y+ z};

//this works
let add_works = foo();

println!("test closure {}", add(1,2));
println!("test closure 2 {}", add_works(1,2));


}
fn foo() -> impl Fn (i32,i32) -> i32 {

    let z = 101;
   move |a,b| {a+b+z}
}