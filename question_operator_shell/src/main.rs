use std::fs::File;
use std::io;
fn test_question_mark_operator()->Result<String,io::Error>{
  File::open("nonexistentfile.txt")?;
  // match res{
  //   Ok(file)=>{
  //     println!("file opened");
  //   }
  //   Err(err)=>{return Err(err);}
  // }
  Ok("open file ok".to_string())
}
fn main() {
    let res=test_question_mark_operator();
    match res{
      Ok(s)=>println!("good"),
      Err(e)=>println!("not opened withe error:{:?}",e),
    }
}
