//Entry point to the program returns "()"
//const be known at compile time and must be declared in global scope
use std::io;
const MINUTES_IN_Day:u32=24*60;
fn main() {
    let user:(&str,i8,char,bool)=("FADDY",-1,'m',true);
    //deconstructing the tuple
    let (a,b,c,d)=user;
    let age=if b>0{
      b
    }else{
      0
    };
    let mut i=0;
    //this is how you name loops
    'age_finding:loop{
      i+=1;
      if i==5{
        continue;
      }
      if i==10{
        break;
      }
    }
    // for num in (1..11).rev(){
    //   println!("");
    // }
    // let mut guess=String::new();
    // io::stdin().read_line(&mut guess).expect("unable to read line");
    let a=String::from("hello");
    let b=a;
    println!("{}",a);
    //println is a macro
}
