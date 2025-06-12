use std::result;

enum MathErrror{
    DivisionByZero,
    NegativeSqrt

}

type MathReslt<T> =Result<T,MathErrror>;
fn main() {
    println!("Hello, world!");
    let  x = 5;
    println!("The value of x is {}", x);
  
  //shadowing
    let x= "std";
    println!("The value of x is {}",x);
  
    //CONST SHOULD BE TYOE SPECIFIED
        const EXAMPLE_CONST : u32= 78;

    println!("The value of EXAMPLE_CONST is {}",EXAMPLE_CONST);

    let  test_x : u8 = 255;
    println!("The value of test_x is {}",test_x);
   

    const RETURN_VALUE_CONST:u32 = testing_constfunctions(3,4);

     println!("The value of return_value is {}",RETURN_VALUE_CONST);

     let controlfunction_returntype = testing_controlfunctions(5,6);

     println!("The value of controlfunction_returntype is {}",controlfunction_returntype);

        let loopfunction_returntype = testing_loopfunctions();
        println!("The value of loopfunction_returntype is {}",loopfunction_returntype);



        let result_return = testing_result(45,0);
        println!("The value of result_return is ok? {:}",result_return.is_ok()); //prints false
        println!("The value of result_return is err? {:}",result_return.is_err()); //prints true
        let result_return2 = testing_result(45,45);
        println!("The value of result_return is ok? {:}",result_return2.is_ok()); //prints true
        println!("The value of result_return is {}",result_return2.)

}

//testing Result

fn testing_result(x:u32,y:u32) -> MathReslt<u32>{
    if y==0{
       return Err(MathErrror::DivisionByZero);

    }
    else{
        return Ok(x/y);
    }
}
//testing const functions
const fn testing_constfunctions(_x:u32,_y:u32) -> u32 {

    let x :u32=5;
    let y:u32=6;

    // println!("testing_constfunctions The value of x is {}",x);
    // println!("testing_constfunctions The value of y is {}",y);

    return x+y;


}
fn testing_controlfunctions(x:u8,y:u8)->bool{


    //the condition shoule  be explicily bool. if x - doesnt work
    //brackets are neccessaey
    if x>y{
        true
    }
    else if y==x{
     false
    }
    else {
       false
    }
    

}

fn testing_loopfunctions()->u8{
let mut x:u8=10;
   let result =  loop{
    x=x+2;
    if x>100{
        
       break x*2;
    }

};
result
}