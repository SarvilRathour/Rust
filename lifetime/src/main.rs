fn first_word(s:&str)->&str{
    let dis=s.as_bytes();
    for (i,&item) in dis.iter().enumerate(){
        if item==b' '{
            return &s[0..i]
        }
    }
    &s[..] 
}
fn main(){
    let sarvil="world sarvil rathour is the best human being in the world";
    let word=first_word(sarvil);
    println!("{}",word);
}
