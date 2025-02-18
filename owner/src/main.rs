fn main() {
    // let mut s =String::from("hello");
    // s.push_str(",(hellworld)");
    // println!("{s}");
// let s=String::from("hello");
// let s1=s.clone();
// println!("s={s},s1={s1}");
// let s=String::from("hello");
// take_ownership(s);

// let x=4;
// take_int(x);
// println!("{x}");
// }
// fn take_ownership(some:String){
//     println!("{some}");
// }
// fn take_int(z:u32){
//     println!("{z}")
// let s1=give_owner();
// println!("{s1}");
// let s2=String::from("hello");
// let s3=take(s2);
// println!("{s3}");
// }
// fn give_owner()->String{
//     let z=String::from("don");
//     println!("{z}"); 
//     z
      
// }
// fn take(b:String)->String{
//     println!("{b}");
//     b
let s1=String::from("hello");
let (s2,len)=calculate(s1);
println!("the size of {s2} is {len}")
   
}
fn calculate(s3:String)->(String,usize){
    let len=s3.len();
(s3,len)
}