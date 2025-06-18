
 use std::cmp::Ordering;

 
 struct Machine{
    name: String,
    index: i32,
    waterusage: bool,
}

pub trait WaterOutlet{
    
    //remember to pass self as function argument
    //unlike "this" keyword in C++ , its not automatically available in methods of a struct - has to be passed
    fn HasWaterInlet(&self) -> bool; // only method signature needed here
    fn HasWaterProofing(&self) -> bool{
        println!("Default implementation for fn HasWaterProofing for tjis trait");
        false
    }
}

//generic trait
pub fn water_summary_function<T: WaterOutlet>( param: &T, param2: &T){

}

//"where"

// pub fn water_summary_function2<T, U> (param:&T, param: &U) -> u32
//         : where T : Copy,
//                U:Copy{}



//implementing trait WaterOutlet for Machine
impl WaterOutlet for Machine{

    fn HasWaterInlet(&self) -> bool {
        if self.name == "CoffeeMachine" {true}
        else { false}
    }

     fn HasWaterProofing(&self) -> bool {
        if self.name == "CoffeeMachine" { false}
        else  {true}
    }
}

impl Machine{
    
    //acts like a constructor - its not an associated function as self is not passed as argument
    fn new(name: String, index:i32, waterusage: bool ) -> Machine{Machine{name,index,waterusage}}
}

struct SanitaryWare{
    name: String,
}


impl WaterOutlet for SanitaryWare{
   fn HasWaterInlet(&self) -> bool {
        if self.name == "WaterTap" {true}
        else { false}
    }

     fn HasWaterProofing(&self) -> bool {
        if self.name == "WaterTap" { true}
        else  {false}
    }
}
fn checkWaterInlet<T: WaterOutlet>(m: T) {
    println!("This Machine has an water inlet?  {}", m.HasWaterInlet());
}

//returning not a Type T , but anything that implements a trait WaterOutlet
fn returningTraits() -> impl WaterOutlet{

    Machine{
        name:"test".to_string(),
        index: 9,
        waterusage:false,
        
    }
}

//https://google.github.io/comprehensive-rust/generics/exercise.html
//T has the trait Ord here
 fn min<T: Ord>(one: T, two:T) -> T{
match one.cmp(&two) {
    Ordering::Less | Ordering:: Equal => one,
    Ordering::Greater => two,
}
}
fn main() {
    println!("Traits!");
    let m = Machine::new("CoffeeMachine".to_string(), 32, true);
    println!("m HasWaterProofing {} ", m.HasWaterProofing());
    println!("m HasWaterInlet {} ", m.HasWaterInlet());

    let m2 = Machine{name:"WaterPurifierMachine".to_string(),index:48, waterusage:true};
    println!("m2 HasWaterProofing {} ", m2.HasWaterProofing());
    println!("m2 HasWaterInlet {} ", m2.HasWaterInlet());
  
  //Anything that has this trait can be passed here
  checkWaterInlet(m2);


  //For example a struct of type SanitaryWare 
  //SanitaryWare and Machine types has the common trait WaterOutlet here
  let s = SanitaryWare{name:"tap".to_string()};
  checkWaterInlet(s);

}

fn FindSmallest<T: Copy>(number_list: Vec<T>){
   

    
}

//Two generic types here T and Y
struct Points<T,Y> {
    x: T,
    y: Y,
}

impl <T,Y> Points<T,Y>{
    fn samplefunction(self) -> T{
       self.x
    }
}

impl <u16,u32> Points<u16,u32>{
    fn samplefunction2(&self) -> &u32{
        &self.y
    }
}