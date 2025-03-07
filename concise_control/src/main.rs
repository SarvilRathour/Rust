// fn main(){
// let config_max:Option<u8>=Some(3u8);
// // match config_max {
// //     Some(max )=>println!("the max value is {max}"),
// //     _=>(),
// // }
// if let Some(max)=config_max{
//     println!("the value of max is {max}");
// }
// }
#[derive(Debug)]
enum Usstate{
    Alaska,
    Albama,
}
impl Usstate{
    fn existed_in(&self,year:u16)->bool{
        match self{
            Usstate::Albama=>year>=1819,
            Usstate::Alaska=>year>=1990,
        }
    }
}
enum Coin{
    Penny,
    Quarter(Usstate),
}
fn describe_state_quarter(coin:Coin)->Option<String>{
 if let Coin::Quarter(state)=coin{
    if state.existed_in(1900){
        Some(format!("{state:?} is pretty new"))
    }else {
        Some(format!("{state:?} is pretty old"))
    }
    }else{
        None
    }
 }

fn main(){
    // let coin=Coin::Quarter(Usstate::Alaska);
    // let mut count=0;
    // match coin{
    //     Coin::Quarter(state)=>println!("state quarter from {state:?}"),
    //     _=>count+=1,
    // }
    // println!("the count is {count}");
    if let Some(desc)=describe_state_quarter(Coin::Quarter(Usstate::Albama)){
        print!("{desc}");
    }
}
