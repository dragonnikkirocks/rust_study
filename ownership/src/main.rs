fn main() {
    println!("Chap 4 Ownership");

    //Rust uses ownership model to handle memory management 
    //pro :
    //- control over memory, error free, faster runtime, smaller programm size
    //cons:
    //- slower write time, steeper learning curve
       
       //Box::new - creates an array of 1 million elements on heap
       //if a = 0;1000000 - its defined in stack - so in variable is created in frames
       //here by using Box::new - its defined in heap
       //Heap is not tied to a specific frame
        let a = Box::new([0; 1_000_000]);
        let b = a;


//Stack and heap
//stacks are of fixed size and cannot be changed during runtime
// stacks stores the local variables of the functions being execueted
// size of stack frames  are calculated in the runtime
// variables inside the stack frame is only valid in the lifetime or scope 
//of the function - whereas the heap is valid through out the whole lifetime of
// the programm
//

// HEAP
//can grow or shrink during runtime
//data stored in heap can be dynamic in size
// data stored in heap can be large amounts in size
// zthats why String ( whose size is not known ), vector etc is stored in heap


//disadvantages of heap
//1. pushing data to heap is slower than pushing to stack, as heap has
// to look for space or memory where data can be stored
//2. also getting data from stack is faster, as on heap - we have to follow the pointer
// to find the location where data is stored


//OWNERSHIP RULES
//1. Each value in Rust will have one owner
// 2. variable can have only one owner
// 3. when owner goes out of scope, value will be dropped


//how variables and data interact with each other

let x: i32 =9;
let y: i32 = x; //copy

let s1: String = String::from("niki");
let s2 : String = s1; //move - not shallow copy i.e s1 is invalidated here

println!("{}, makri s2 ",s2);

// println!("{}, makri s1 ",s1); - tried to access s1 after moving

let s3 : String = s2.clone(); //here s2 is not moved not cloned - so s2 exists after this line
println!("{}, makri s3",s3);
println!("{}, makri s2 after clone ",s2);


//passing ownership
takes_ownwership(s2,x);
//error - passing values as parameters to function is also like moving
//println!("{}, makri s2 after passing ownership ",s2);

//works here as integers are copied not moved
println!("{}, makri x after clone ",x);




      

        
}

fn takes_ownwership(s:String,x:i32)
{
    println!("{},takes_ownership",s);

}