use std::collections::HashMap;
use lopdf::Document;
use regex::Regex;
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
            println!("{k}-----{v}");
            if v>max_count{
                max_count=v;
                max_key=k;
            }
            
            println!("The max count is {}",max_count);
        }
        println!("The max count is {}",max_count);
       max_key.to_string()
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
    let largest=to_get_largest(store);
    println!("The largest is---{}",largest);
}
