fn main() {
    println!("Lifetime bounds");

   let s = between("nikki is a good girl", "a","girl");
   println!("the output is {s}");


   let input = String::from("Mummy is a makri baby");
   let left = String::from("a");
   let right= String::from("baby");
   
   let answer = between(&input,&left,&right);
    //this drop works here because input and left have different lifietimes
   drop(left); //calling destructir trait
   //drop(input); //this drop is not possinble. Here vars input and return type of between of f
   println!("the var is {answer}");

   annotation_test("test");
}
//this function returns a reference - we have to tell rust some info about the lifetime of the variable which 
// we are returning
//rust compiler is normally able to figure out lifetime of the return
//caller of function between should know what they van do with refrences that is passed in
//'a indicates the lifetime annotation of the variables
fn between<'a,'b>(input :& 'a str, start_marker: & 'b str, end_marker: & 'b str) -> & 'a str
{
    let Some(mut start) = input.find(start_marker) else { return ""};
    start=start+start_marker.len();
    let Some(end) = input[start..].find(end_marker)else{ return " "};
    &input[start..start+end]

}
//here since only one input parameter is there - output param will have same lifetime as
// the output
fn annotation_test(input:&str) ->&str{
    &input
}

//here since more than one param is passed to the function, lifetime has to be specified
fn annotation_test2<'a>(input: &'a str, input_second : &'a str) -> &'a str{
    &input
}