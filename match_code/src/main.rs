use std::collections::btree_map::Values;

// enum Coin{
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }
// fn value(coin:Coin)->i32{
//     match coin{
//         Coin::Nickel=>1,
//         Coin::Penny=>3,
//         Coin::Dime=>5,
//         Coin::Quarter=>8,
//     }
// }
// fn main(){
//     let coin=Coin::Dime;
//     println!("the value of the coin is {}",value(coin));
// }
// fn main(){
//     fn plus_one(x:Option<i32>)->Option<i32>{
//         match x {
//             None=>None,
//             Some(i)=>Some(i+1),
//         }
//     }
//     let five=Some(5);
//     let six=plus_one(five);
//     let none=plus_one(None);
//     match six{
//         Some(value)=>println!("the value is {}",value),
//         None=>println!("the value is none "),
//     }
// }
// fn main(){
//     let dice_roll=9;
//     match dice_roll {
//         3=>add_fancy_animation(),
//         5=>add_asethtic_animation(),
//         _=>reroll()
//     }
//     fn add_fancy_animation(){}
//     fn add_asethtic_animation(){}
//     fn reroll(){}
// }