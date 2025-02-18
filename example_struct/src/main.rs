// fn main() {
//     let rect=(50,60,70);
//     println!("the area fo the rectangle {}",area(rect));
// }
// fn area(dimensions:(u32,u32,u32))->u32{
//     dimensions.0*dimensions.1*dimensions.2
// }
// fn main(){
//     struct Rectangle{
//         width:u32,
//         height:u32,
//     }
//     let rect1=Rectangle{
//         width:5,
//         height:6,
//     };
//     println!("{}",area(&rect1))
// }
// fn area(rectangle:&Rectangle)->u32{
//     rectangle.width*rectangle.height
// }
#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}
fn main(){
    let scale=2;
    let react=Rectangle{
        width:dbg!(30*scale),
        height:44,
    };
dbg!(&react);
}