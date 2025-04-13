use std::io::{self, Read};
use std::fmt::{Display, Formatter};
use inquire::{
    error::{CustomUserError, InquireResult},
    required, CustomType, MultiSelect, Select, Text,
};

#[derive(Debug)]
enum Category{
    Food,
    Travel,
    Shopping,
    Entertainment,
}
struct Expense{
    amount:f64,
    category:Category,
    date:String,
}
fn get_section() -> Vec<&'static str> {
    vec![
        "Add Expenses",
        "All Expenses",
        "Filter Expenses",
        "Calculate Total Expenses",
        "Exit",
    ]
}
fn get_categories() -> Vec<&'static str> {
    vec![
        "Food",
        "Travel",
        "Shopping",
        "Entertainment",
    ]
}
fn add_expenses(expense:&mut Vec<Expense>){
    let categories_str = Select::new("Enter the Categories:", get_categories()).prompt();{
        Ok(category) >= category; // ✅ Correctly handles successful input
        Err(_) => {
            println!("Error: Failed to select a category.");
            return; // Stops execution if error occurs
    };
}
    //     Ok(category)=>category,
    //     Err(_)=>{
    //         println!("error");
    //         return;
    //     }
    // };
    let mut amount=String::new();
    io::stdin().read_line( &mut amount).expect("number inputting failed");
    let amount:f64=amount.trim().parse().expect("please input a number");
    println!("Please enter the amount you spent:{}",amount);
    println!("Please enter the date in this format--(dd/MM/YYYY");
    let mut date=String::new();
    io::stdin().read_line(&mut date).expect("You got the wrong value");
    let category=match categories_str{
        Ok("Food") => Category::Food,
        Ok("Travel") => Category::Travel,
        Ok("Shopping") => Category::Shopping,
        Ok("Entertainment") => Category::Entertainment,
        _ => {
            println!("Invalid category.");
            return;
        }
    };
   let new_expense= build_expense(amount,date,category);
   expense.push(new_expense);
}
fn build_expense(amount:f64,date:String,category:Category)->Expense{
    Expense{
        amount,
        category,
        date,
    }
}

fn main()  -> InquireResult<()>  {
   let mut expense:Vec<Expense>=Vec::new();
   loop{
   let selection = Select::new("Enter the Section:", get_section()).prompt()?;

   match  selection {
    "Add Expenses"=>add_expenses(&mut expense),
    "All Expenses"=>println!("all expenses"),
    "Filter Expenses"=>println!("filter expenses"),
    "Calculate Total Expenses"=>println!("total expenses"),
    "Exit"=>{println!("BYeeeeee");break;},
    _=>println!("whatever you choose was wong"),
   }
   }
   Ok(())

}
