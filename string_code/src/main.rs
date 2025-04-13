

struct stringManiuplator;
impl stringManiuplator{
    fn add(&self,s:&str)->String{
        let mut  m:String=String::from(s);
        m.push_str("the reality is so big");
        m
    }
}

fn main() {
    let data="to string ";
    let z=stringManiuplator.add(data);
    println!("{}",z);
}
