use std::io;
#[derive(Debug,PartialEq)]
enum Priority{
  Low,
  Medium,
  High
}
#[derive(Debug)]
struct Task{
  title:String,
  priority:Priority,
  done:bool,
}
#[derive(Debug)]
struct Inventory{
  tasks:Vec<Task>,
}
fn add_task(task_inventory:&mut Inventory){
  let task1=Task{
    title:String::from("Learn Rust"),
    priority:Priority::High,
    done:false,
  };
  let task2=Task{
    title:String::from("go to college"),
    priority:Priority::Medium,
    done:false,
  };
  task_inventory.tasks.push(task1);
  task_inventory.tasks.push(task2);
  println!("{:#?}",task_inventory.tasks);
}
fn sort_task(task_inventory:&mut Inventory){
   for task in &task_inventory.tasks{
    if task.priority==Priority::High{
      println!("{:?}",task);
    }
   }
}
fn main(){
  let mut inventory=Inventory{
    tasks:Vec::new(),
  };
  loop{
  println!("TODO APP");
  println!("1.Add task");
  println!("2.Sort by priority");
  let mut number=String::new();
  io::stdin().read_line(&mut number).expect("failed to read the line");
  let n:i32=number.trim().parse().expect("please type a number");
 
  match n{
    1=>add_task(&mut inventory),
    2=>{sort_task(&mut inventory);},
    _=>println!("try again"),
  };
  }
}
