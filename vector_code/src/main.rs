
fn main() {
//     let v:Vec<i32>=Vec::new();
//     println!("{v:?}");
//     let mut v=vec![0,1,2,3];
//     println!("{v:?}");
//     v.push(3);
//     v.push(11);
//     v.push(5);
//     v.push(9);
//     println!("{v:?}");
//     let z=vec![3,5,6,7,8];
//     let third:&i32=&z[2];
//     println!("{}",third);
//     let four:Option<i32>=z.get(3).copied();
//     match four{
//         Some(four)=>println!("{four:?}"),
//         None=>println!("error"),
//     }
    
//     let mut c:Vec<u128>=vec![4,5,6,7,8,9];
//     println!("{c:?}");
//     for i in &mut c{
//         *i+=5_009_594_039_302_920u128;
//  }
//  println!("{c:?}");
#[derive(Debug)]
enum SpreadsheetCell{
    INT(i32),
    Float(f64),
    Text(String),
}
let row=vec![
    SpreadsheetCell::INT(3),
    SpreadsheetCell::Float(4.5),
    SpreadsheetCell::Text(String::from("hello world")),
];
println!("{row:?}");
}

