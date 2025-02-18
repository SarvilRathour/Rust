fn main() {
//    let mut s1=String::from("hello i am sarvil");
//    checking_s1(&mut s1); 
    
    
// }
// fn checking_s1(s3:&mut String){
//     s3.push_str(",hey");
//     println!("{s3}");
let mut s =String::from("sarvil");
{
let s1=checking(&mut s);
println!("s1={}",s1);
}
fn checking(s2:&mut String)->&mut String{
    s2.push_str(";;;;;");
    s2
}
println!("s={}",s);
}
