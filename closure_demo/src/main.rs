fn function_with_closure<G>(f:G) where G:FnOnce(String){
  f("hello world".to_string());
  
}
fn main() {
    let s="the content of x is: ";
    let print_x_clousre=|x:String|{
      println!("{} {}",s,x);
    };
    function_with_closure(print_x_clousre);
}
