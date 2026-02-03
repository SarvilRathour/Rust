struct CustomSmartPointer{
  data:String,
}
impl Drop for CustomSmartPointer{
  fn drop(&mut self){
    println!("Dropping Smart pointer with data `{}`",self.data);
  }
}
fn main(){
    let c=CustomSmartPointer{
      data:String::from("hello world"),
    };
    println!("customsmartpointer created");
}
// struct Node<T>{
//   value:T,
//   left:Box<Node<T>>,
//   right:Box<Node<T>>,
// }
// fn main() {
//     let b=Box::new(5);
       
//     println!("b={b}");
// }
  

