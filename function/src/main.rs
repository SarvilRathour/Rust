// fn main() {
//     another_func(2-7,'h')
// }
// fn another_func(x:i32,label:char){
//     println!("the value of x is {x} ,{label}");
// }
// fn main(){
//     let y={
//         let x=6;
//         x+9
//     };
//     println!("the y is {y}");
// }
fn main(){
    let y=plus_one(8);
    println!("the y is equal to {y}");
}
fn plus_one(x:u32)->u32{
    x+1
}