trait Describe{
    fn describe(&self)->String;
}
struct Car{
    model:String,
    mileage:u32,
}
impl Describe for Car{
  fn describe(&self)->String {
   format!("{},{}",self.model,self.mileage)
  }
}
struct Laptop{
  brand:String,
  battery:u8,
}
impl Describe for Laptop{
  fn describe(&self)->String {
         format!("{},{}",self.brand,self.battery)  
  }
}
fn print<T:Describe>(items:&Vec<T>){
  for item in items{
    println!("{}",item.describe());
  }
}
fn main() {
    let cars = vec![
        Car {
            model: String::from("Tesla"),
            mileage: 15000,
        },
        Car {
            model: String::from("Toyota"),
            mileage: 30000,
        },
    ];

    let laptops = vec![
        Laptop {
            brand: String::from("Dell"),
            battery: 87,
        },
        Laptop {
            brand: String::from("Apple"),
            battery: 42,
        },
    ];
    print(&cars);
    print(&laptops);
}
