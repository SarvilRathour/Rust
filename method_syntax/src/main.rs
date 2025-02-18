struct Person{
    name:String,
    age:u32,
}
impl Person{
    fn describe(&self)->String{
       format!("Name:{},Age:{}",self.name,self.age)
    }
}
fn main() {
    let person=Person{
        name:"sarvil".to_string(),
        age:19,
    };
    let description=person.describe();
    println!("the details of the person is {}",description);
}
