use std::io::{self, Read};
struct Todo{
    task:String,
    completion:bool,
}
fn add_todo(todos:&mut Vec<Todo>){
    println!("enter the todo you want to add");
    let mut task=String::new();
    io::stdin().read_line(&mut task).expect("please enter a valid todo");
    let new_todo=build_todo(task.trim().to_string());
    todos.push(new_todo);
    println!("Todo added succesfully");
}
fn build_todo(task:String)->Todo{
    Todo{
        task,
        completion:false,
    }
}

fn edit_completion(todos:&mut Vec<Todo>){
    if todos.is_empty(){
        println!("there is no todos to be edited");
        return;
    }
    view(todos);
    println!("Enter the index of the todo you want to change the completion");
    let mut index=String::new();
    io::stdin().read_line(&mut index).expect("enter a number");
    let index:usize=index.trim().parse().expect("please enter a numner");
    if(index==0||index>todos.len()){
        println!("you choose a index that is not possible");
        return;
    }
   
    todos[index-1].completion=true;
    println!("todo mark as completed");
}
fn view(todos:&Vec<Todo>){
    if todos.is_empty(){
        println!("todos is empty make a new one");
        return;
    }
    println!("your todo list\n");
    for(i,todo)in todos.iter().enumerate(){
        let status=if todo.completion{"[✔]"}else{"[ ]"};
        println!("{}.{} {}",i+1,status,todo.task)
    }

}
fn main() {
    let mut todos:Vec<Todo>=Vec::new();
    loop{
    print!("TODO\n1.New TODO\n2.Edit Completion Status\n3.View\n4.Exit\n");
    let mut a=String::new();
    io::stdin().read_line(&mut a).expect("failed to read the line");
    let a:u32=a.trim().parse().expect("please enter a number");
    match a{
        1=>add_todo(&mut todos),
        2=>edit_completion(&mut todos),
        3=>view(&todos),
        4=>{
            println!("Byee");
            break;
        },
        _=>println!("please select from the choice above"),
    }
}
}
