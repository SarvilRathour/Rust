// //Struct
// enum ColorV{
//   green,
//   blue,
//   red
// }
// impl std::fmt::Display for ColorV{
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//       match self{
//         ColorV::red=>write!(f,"red my fav"),
//         ColorV::blue=>write!(f,"blue not my fav"),
//         ColorV::green=>write!(f,"green is my least fav"),
//       }
//   }
// }
// struct Vehicle{
//   name:String,
//   color:ColorV,
// }
// fn create()->Vehicle{
//   let a1=Vehicle{name:"Sarvil".to_string(),color:ColorV::blue};
//   a1
// }
// impl std::fmt::Display for Vehicle{
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//    write!(f,"({},{})",self.name,self.color)   
//   }
// }
// fn main() {
//   let a =create();
//   println!("The values are: {a}");
// }
// struct Person<T> 
// where
//   T:Animal+NotDangerous
// {
//   first_name:String,
//   pet:T
// }
// trait Animal{
//   fn make_sound(&self);
// }
// trait NotDangerous{}
// struct Dog{}
// impl Animal for Dog{
//   fn make_sound(&self){
//       println!("bark");
//   }
// }
// impl NotDangerous for Dog{}
// fn main(){
//   let my_dog=Dog{};
//   my_dog.make_sound();
//   // let p1=Person{first_name:String::from("sarvil"),pet:my_dog,};
// }
// fn earger_double(nums:Vec<i32>)->Vec<i32>{
//   let mut new_vec=Vec::new();
//   for num in nums{
//     let b=num*2;
//     new_vec.push(b);
//     println!("iteration happening for eager");
//   }
//   println!("iteration completed for eager");
//   new_vec
// }
// fn lazy_double(nums:Vec<i32>){
//   let iter=nums.into_iter().map(|x|{
//   println!("Computing for {}", x);
//         x * 2
//   });
//   for num in iter{
//     println!("{}",num);
//   }
//   println!("Iterator created!");
// }
// fn main(){
//   let a=earger_double(vec![3,6,7,6,5,4,3,2,2]);
//   println!("{:?}",a);
//   let b=lazy_double(vec![3,6,7,6,5,4,3,2,2]);
// }
//A lexer 
use std::io::Error;
struct Token{
  
}
fn lexer(input:&str)->impl Iterator<Item=Result<Token,Error>>{
  
}
fn main(){
 lexer("var x=5");
}
