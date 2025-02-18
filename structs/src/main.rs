struct User{
    username:String,
    email:String,
    active:bool,
    sign_in_count:u64,
}
// fn main() {
//     let user1=User{
//         email:String::from("another@gmail.com"),
//         username:String::from("another"),
//         active:true,
//         sign_in_count:1,
//     };
//     let user2=User{
//         email:String::from("another1@gmail.com"),
//         ..user1
//     };
//     // println!("{}",user1.username);
//     println!("{}",user1.email);
//     println!("{}",user2.username);
// }
fn build_user(email:String,username:String)->User{
    User{
        username,
        email,
        active:true,
        sign_in_count:1,
    }
}
fn main(){
    let user1=build_user(
        String::from("another"),
        String::from("another@gmail.com")
    );
    println!("{}",user1.username);
}