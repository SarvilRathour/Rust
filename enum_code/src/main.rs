// #[derive(Debug)]
// enum IPkind{
//     V4,
//     V6,
// }
// #[derive(Debug)]

// struct IPAddr{
//     kind:IPkind,
//     address:String,
// }
// fn main() {
//     let home=IPAddr{
//         kind:IPkind::V4,
//         address:String::from("127.0.0.1")
//     };
//     let home2=IPAddr{
//         kind:IPkind::V6,
//         address:String::from("::1"),
//     };
//     println!("The first home is {home:?},The second home is {home2:?}"); 

// }
// #[derive(Debug)]
// enum IPking{
//     V4(String),
//     V6(String),
// }
// fn main(){
// let home=IPking::V4(String::from("127.0.0.1"));
// let loopback=IPking::V6(String::from("::1"));
// println!("Home={home:?},Loopback={loopback:?}");
// }
// enum Message{
//     Quit,
//     Move(x:i32,y:i32),
//     Write(String),
//     ChangeColor(i32,i32,i32)
// }
// impl Message{
//     fn call(&self){

//     }
// }
// fn main(){
//     let m=Message::Write(String::from("hello"));
//     m.call();
// }
#[derive(Debug)]
enum Option<T>{
    None,
    Some(T),
}
fn main(){
    let some_number=Option::Some(5);
    println!("the number is {some_number:?}");
    let null_value : Option<i32> =Option::None;
    println!("the number is {null_value:?}");
}