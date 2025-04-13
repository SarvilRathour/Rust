use std::{io, string};

#[derive(Debug)]
struct Tree{
    value:String,
    left_child:Option<String>,
    right_child:Option<String>,
}
fn insert_left(tree:&mut Tree,value:Option<String>){
    if tree.left_child.is_some(){
        println!("there is some value");
    }else{
        tree.left_child=value;
        println!("Value inserted in left sucessfully");
    }
}
fn insert_right(tree:&mut Tree,value:Option<String>){
    if tree.right_child.is_some(){
        println!("there is some value");
    }else{
        tree.right_child=value;
        println!("value inserted in right sucessfully");
    }
}
fn main() {
    let mut tree=Tree{
        value:String::from("a"),
        right_child:None,
        left_child:None,
    };
    let mut value_string=String::new();
    io::stdin().read_line(&mut value_string).expect("please give a number");
    let mut value:Option<String>=Some(String::from(value_string));
    insert_left(&mut tree, value.clone());
    insert_right(&mut tree,  value.clone());
    println!("The tree value is {tree:?}");
    println!("Left child: {:?}", tree.left_child);
    println!("Right child: {:?}", tree.right_child);
// println!("the left child is :{:?}",tree.left_child);
}
