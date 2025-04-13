use std::collections::{btree_map::Keys, HashMap};
fn main() {
    
    // let mut scores=HashMap::new();
    let scores_1=(String::from("blue"),10);
    let scores_2=(String::from("red"),20);
    // let team_name:i32=10;
    // let score:String=scores.get(&team_name).copied().unwrap_or(String::from("not found")).to_string();
    // println!("{}",score);
//     for (keys,values) in &scores{
// println!("{keys}::{values}");
//     }
    let mut owner=HashMap::new();
    owner.insert(scores_1, scores_2);
    println!("{owner:?}");

}
