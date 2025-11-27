use num::Complex;
fn escape_time(c:Complex<f64>,limit:usize)->Option<usize>{
  let mut z=Complex{re:0,im:0.0};
  for i in 0..limit{
    if z.norm_sqr()>4.0{
      return some(i);
    }
  }
  None
}
#[test]
fn test_points(){
  let limit =255;
  assert!(escape_time(complex{re:0.0,im:0.0}, limit)).is_none());
  
}
fn main() {
    println!("Hello, world!");
}
