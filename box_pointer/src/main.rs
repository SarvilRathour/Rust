struct Node<T>{
  value:T,
  left:Box<Node<T>>,
  right:Box<Node<T>>,
}
fn main() {
    let b=Box::new(5);
       
    println!("b={b}");
}

