// pub fn add(right:u64,left:u64)->u64{
//     left+right
// }
// #[cfg(test)]
// mod test{
//     use super::*;
//     #[test]
//     fn it_works(){
//         let result=add(2,2);
//         assert_eq!(result,4);
//     }
//     #[test]
//     fn dont_work(){
//         let result=add(5,5);
//         assert_eq!(result,11);
//     }
// }
// struct Rectangle{
//     width:i32,
//     height:i32,
// }
// impl Rectangle{
//     fn can_hold(&self,other:&Rectangle)->bool{
//             self.width>other.width&&self.height>other.height
//     }
// }
// #[cfg(test)]
// mod tests{
//     use super::*;
//     #[test]
//     fn larger_can_hold_smaller(){
//         let rect1=Rectangle{
//         width:8,
//         height:9,
//     };
//     let rect2=Rectangle{
//         width:1,
//         height:1,
//    };
//    assert!(rect1.can_hold(&rect2));
//     }
// }
// fn add_two(a:u64)->u64{
//     a+2
// }
// #[cfg(test)]
// mod tests{
// use super::*;
// #[test]
// fn testing(){
//     let two=add_two(2);
//     assert_ne!(two,5);
// }
// }
struct Guess{
    value:u32,
}
impl Guess{
    fn new(value:u32)->Guess{
        if value>100{
            panic!("value must be below than 100")
        }
        Guess{value}
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    #[should_panic]
    fn value_test(){
        Guess::new(9);
    }
}
