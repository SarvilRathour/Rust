use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main(){
    println!("Guess the number");
    let secret_number=rand::thread_rng().gen_range(1..=100);
    // println!("the secret number is {secret_number}");
    loop{
    println!("Please input the number");
    let mut guess=String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("failed to read the line");
    println!("you guessed :{}",guess);
    let guess:u32=match guess.trim().parse(){
        Ok(num)=>num,
        Err(_)=>continue,
    };
    match guess.cmp(&secret_number){
        Ordering::Less=>println!("small"),
        Ordering::Greater=>println!("big"),
        Ordering::Equal=>{
            println!("You win");
            break;
        },
    }
}
}