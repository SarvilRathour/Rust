fn first_word(s:&str)->&str{
    let bytes=s.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if item==b' '{
            return &s[0..i];
        }
    }
    &s[..]
}
fn main() {
//     let a=String::from("helloh world");
//     let b=first_word(&a);
//     println!("{b}");
// }
// fn first_word(s:&String)->&str{
//     let bytes=s.as_bytes();
//     for (i,&item) in bytes.iter().enumerate(){
//         if item==b'd'{
//             return &s[..i];
//         }
//     }
//     &s[..]
let my_string=String::from("helloiamsarvil");
let word=first_word(&my_string[0..9]);
let word=first_word(&my_string[..]);
let word=first_word(&my_string);
println!("{word}");
}
