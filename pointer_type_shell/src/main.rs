use std::ops::AddAssign;
fn main() {
 let a:i32=10;
 let a1=&a;
 println!("a value: {}",a);
 println!("a1 value: {:p}",&a1);
 let mut c=10;
 let d=&mut c;
 d.add_assign(8);
 println!("c value:{}",c);
 let t=(12,"sarvil");
 let b=Box::new(t);
 println!("the value of t is: {:?}",t);
 println!("the address of t is: {:p}",&t);
 println!("the value of box is: {:?}",b);
 println!("the address of the box is: {:p}",&b);
}

