use std::io;
fn main() {
    println!("please enter a line");
    let mut line=String::new();
   io::stdin()
   .read_line(&mut line)
   .expect("failed to read line");
   let mut word=String::new();
   let mut longest_word=String::new();
   let lines=line.as_bytes();
   for &item in lines.iter(){
    if item!=b' ' && item!=b'\n'{
        word.push(item as char);
    }
    else{
        if word.len()>longest_word.len(){
            longest_word=word.clone();
        }
        word.clear();
    }
   } 
   println!("the longest word is {}",longest_word);
}
