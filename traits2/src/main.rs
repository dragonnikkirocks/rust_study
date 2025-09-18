use std::ops::Add;


trait myadd{
    fn my_add(&self, other:&Self) -> Self;
}

impl myadd for i32{
    fn my_add(&self, other:&i32) ->i32{
        *self + *other
    }
}

fn add_values<T>(one: &T, two: &T) -> T
  where T : myadd
 {
   one.my_add(two)
}

//what if we need two add tow numbers with different traits
//what if the result or return value has a different trait
//we make the trait itself generic
trait MyAddGeneric<T>
{   type Output; //associated type
    fn my_add(&self, other:&Self) -> Self::Output;
}

#[derive(Clone)]
//this derive is a macro that figures out how to implement tzhe trait Clone for 
//the struczure. Doesnt work for every type
struct Dolly{
    number_of_legs: i32,
}


#[derive(Clone)]
#[derive(Default)]
struct StringAndInteger{
    name:String,
    id:i32,
}

impl Add for StringAndInteger{
    type Output = Self;
    fn add(self, rhs: Self) -> StringAndInteger{
        let n: String = self.name + &rhs.name;
        let i = self.id + rhs.id;
        StringAndInteger{name: n, id:i}
    }
}


fn main() {
    println!("Traits2!");
    let m = StringAndInteger{name:"hello".to_string(),id:3};
        let l = StringAndInteger{name:"world".to_string(),id:3};
    let s= l+m; //note that + operator can be used here
    let sk =  StringAndInteger{name:" nikki".to_string(),id:3};
    println!("n {} and id {}",s.name,s.id);
        println!("n {} and id {}",s.name,s.id);
    let k = s.clone()+sk;
           println!("n {} and id {}",s.name,s.id); //works because in previous line I  am passing a clone of s, instead of s itself
  
    //default trait is enabled
    let m: StringAndInteger = Default::default();
           println!("n {} and id {}",m.name,m.id);



    let a :i32 = 9;
    let b : i32 = 9;
    add_values(&a,&b);
 
    //error as trait myadd not implemented for type i8
    //     let c :i8 = 9;
    // let d : i8 = 9;
    // add_values(&c,&d);

    let dolly  =  Dolly{number_of_legs:87};
    let second_dolly = dolly.clone();
    assert_eq!(dolly.number_of_legs,second_dolly.number_of_legs);

}
