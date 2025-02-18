pub fn reverse_string(input:&str)->String{
    input.chars().rev().collect()
}
pub fn longest_word(input:&str)->String{
    let mut word=String::new();
    let mut longest_word=String::new();
    let inputs=input.as_bytes();
    for &item in inputs.iter(){
        if item!=b' ' && item!=b'\n'{
            word.push(item as char);
        } else{
            if word.len()>longest_word.len(){
                longest_word=word.clone();
            }
            word.clear();
        }
    }
   longest_word

}