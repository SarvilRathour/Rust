use std::env;
use std::fs;
struct Config{
  query:String,
  file_path:String,
}
fn parse_config(args:&[String])->Config{
let query = &args[1].clone();
let file_path = &args[2].clone();
  Config{query,file_path}
}
fn main(){
  let args:Vec<String>=env::args().collect();
  let (query,file_path)=parse_config(&args);
  println!("query:{}",query);
  println!("file path:{}",file_path);
  println!("Reading file");
  let content=fs::read_to_string(file_path).expect("invalid file path");
  println!("Contents:\n{content}");
}
