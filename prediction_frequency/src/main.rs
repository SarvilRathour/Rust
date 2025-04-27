use std::{collections::HashMap, io::Read};
use lopdf::Document;
use regex::Regex;
use std::io;
fn frequency(m:Vec<&str>)->HashMap<String, u32>{
    let mut frequency: HashMap<String, u32> = HashMap::new();
    let re=Regex::new(r"[^a-zA-Z\s]").expect("it is not working");
    for words in m{
        let cleaned_word=re.replace_all(words,"").to_string();
        *frequency.entry(cleaned_word).or_insert(0) += 1;
    }
    frequency
}
fn to_get_largest(largest:HashMap<String,u32>)->String{
        let mut max_count=0;
        let mut max_key=String::new();
        for(k,v) in largest{
            // println!("{k}-----{v}");
            if v>max_count{
                max_count=v;
                max_key=k;
            }
        }
        // println!("The max count is {}",max_count);
       max_key.to_string()
}
fn ordered(order:HashMap<String,u32>)->Vec<(String,u32)>{
    let mut items:Vec<(String,u32)>=order.into_iter().collect();
    items.sort_by(|a,b|b.1.cmp(&a.1));
    items
}
fn prediction(o:Vec<(String,u32)>,n:i32,i:String){
    let db:Vec<String>=o.iter()
    .take(n.try_into().unwrap())
    .map(|(s,_)|s.clone())
    .collect();
    let predicted=db.join(" ");
    let result_final=format!("{} {}",i.trim_end(),predicted);
    println!("{}",result_final);

}

fn main() {
    let file="src/test.pdf";
    let mut texts: Vec<String>=Vec::new();
    match Document::load(file){
        Ok(document)=>{
            let pages=document.get_pages();
            for(i,_) in pages.iter().enumerate(){
                let page_number=    (i+1)as u32;
                let text=document.extract_text(&[page_number]);
                texts.push(text.unwrap_or_default());
            }
            
            }
Err(err) =>println!("There was a error {}",err),
    }
    let combined=texts.join(" ").to_lowercase();
    let words:Vec<&str>=combined.split_whitespace().collect();
    let store=frequency(words);
    let largest=to_get_largest(store.clone());
    let ordered=ordered(store);
    // println!("The largest is---{}",largest);
    let mut input=String::new();
    println!("Please enter a line you want to filled by our prediction_frequency(crime and punsihment is being used for this):");
    io::stdin().read_line(&mut input).expect("failed to take the user input");
    println!("Please enter the number of words you want us to predict:");
    let mut numbers=String::new();
    io::stdin().read_line( &mut numbers).expect("failed to load the line for taking length input");
    let number:i32=numbers.trim().parse().unwrap();
    prediction(ordered,number,input);
    // let input_answer=format!("{} {}",input.trim_end(),largest);
    // println!("Our answer by prediction frequency-{}",input_answer);
}
