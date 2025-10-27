// #[derive(Debug,Clone)]
// enum ShirtColor{
//   Red,
//   Blue,
// }
// #[derive(Debug,Clone)]
// struct Inventory{
//   store:Vec<ShirtColor>,
// }
// impl Inventory{
//   fn dispatch(&self,pref:Option<ShirtColor>)->ShirtColor{
//        pref.unwrap_or_else(||self.stocked())
//   }
//   fn stocked(&self)->ShirtColor{
//       let mut num_red=0;
//       let mut num_blue=0;
//       for color in &self.store{
//         match color{
//           ShirtColor::Red=>num_red+1,
//           ShirtColor::Blue=>num_blue+1,
//         };
//       }
//       if num_red>num_blue{
//         ShirtColor::Red
//       }else{
//         ShirtColor::Blue
//       }
//   }
// }
// fn main(){
//   let shirts=Inventory{store:vec![ShirtColor::Blue,ShirtColor::Red,ShirtColor::Blue]};
//   let mut user_pref=Some(ShirtColor::Red);
//   let mut value=shirts.dispatch(user_pref);
//   println!("{:?}",value);
//   let user_pref2:Option<ShirtColor>=None;
//   let mut value1=shirts.dispatch(user_pref2);
//   println!("{:?}",value1);
// }
// use std::thread;
// use std::time::Duration;

// fn generate_workout(intensity: u32, random_number: u32) {
//     let expensive_closure = |num: u32| -> u32 {
//         println!("calculating slowly...");
//         thread::sleep(Duration::from_secs(10));
//         num
//     };

//     if intensity < 25 {
//         println!("Today, do {} pushups!", expensive_closure(intensity));
//         println!("Next, do {} situps!", expensive_closure(intensity));
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!(
//                 "Today, run for {} minutes!",
//                 expensive_closure(intensity)
//             );
//         }
//     }
// }

// fn main() {
//     let simulated_user_specified_value = 10;
//     let simulated_random_number = 7;

//     generate_workout(simulated_user_specified_value, simulated_random_number);
// }
// fn main(){
//   let mut lists=vec![1,2,3];
//   println!("Before defining the clousure:{lists:?}");
//   let mut borrows=|| lists.push(8);
//     // println!("Before calling closure: {lists:?}");
//     borrows();
//     println!("After calling closure: {lists:?}");
// }
// fn main(){
//   let list=vec![1,2,3];
//    println!("Before moving into thread: {list:?}");
//    use std::thread;
//    let handle=thread::spawn(move||{
//            println!("From thread: {list:?}");
//            list
//    });
//    let returned_list=handle.join().unwrap();
//    println!("After getting it back from thread: {returned_list:?}");
// }
#[derive(Debug)]
struct Rectangle{
   width:u32,
   height:u32,
}
fn main(){
      let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];
    list.sort_by_key(|r| r.width);
    println!("{list:#?}");
}

