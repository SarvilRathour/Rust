use std;
use std::thread;
fn run_thread(){
  let s="string in the run thread";
  let mut v=vec![1,2,3];
  let handle=thread::spawn(move ||{
    thread::sleep(std::time::Duration::from_millis(2000));
    println!("the content of s is:{}",s);
    v.push(4);
    println!("the content of s is {:?}",v);
    println!("error here: {}",v[6]);
  });
  // handle.join().unwrap();
  handle.join().expect("the child thread has an error");
}
fn main(){
  run_thread();
}
