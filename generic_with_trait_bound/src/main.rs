fn find_largest<T>(argr:Vec<T>)->T
where
  T:PartialOrd+Clone,
  {
  let mut largest=argr[0].clone();
  for item in argr{
    if item>largest{
        largest=item.clone();
      }
  }
  largest
}
fn main() {
   let s=find_largest(vec![9,8,3,4,7,99,00,4]);
   let r=find_largest(vec![String::from("sarvil"),String::from("rathourr"),String::from("I"),String::from("am"),String::from("the")]);
   println!("{}",s);
   println!("{}",r);
}
