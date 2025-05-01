use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Sha512, Digest};
#[derive(Debug)]
struct Transaction{
    sender:String,
    receiver:String,
    amount:u64,
}
#[derive(Debug)]
struct Block{
    index:u64,
    timestamp:String,
    transaction:Transaction,
    previous_block_hash:String,
    current_block_hash:String,
}
struct App{
    blocks:Vec<Block>,
}
impl App{
    fn new()->Self{
        Self { blocks: vec![] }
    }
    fn genesis_block(&mut self){
        let transaction=Transaction{
            sender:"Sarvil_genesis".to_string(),
            receiver: "network".to_string(),
            amount: 0,
        };
        let hash=calculate_hash(0,SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),"genesis",&transaction);
        let genesis_block=Block{
            index: 0,
            timestamp:SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string(),
            transaction,
            previous_block_hash: String::from("genesis"),
            current_block_hash:hash,
        };
        self.blocks.push(genesis_block);
    }
    fn add_block(&mut self,sender:String,receiver:String,amount:u64){
            let previous_block=self.blocks.last().expect("there is nothing here");
            let index=previous_block.index+1;
            let timestamp=SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            let previous_block_hash=previous_block.current_block_hash.clone();
            let transaction=Transaction{
                sender,
                receiver,
                amount,
            };
            let hash=calculate_hash(index, timestamp, &previous_block_hash, &transaction);
            let new_block=Block{
                index,
                timestamp:SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string(),
                transaction,
                previous_block_hash,
                current_block_hash:hash,
            };
            self.blocks.push(new_block);
    }
    fn print_chain(&self){
        for block in &self.blocks{
            println!("{:#?}",block);
        }
    }
}
fn calculate_hash(id:u64,timestamp:u64,previous_hash:&str,transaction:&Transaction)->String{
   let input=format!(
    "{}{}{}{}{}{}",
    id,
    timestamp,
    previous_hash,
    transaction.amount,
    transaction.receiver,
    transaction.sender,
   );
    let mut hasher = Sha256::new();
     hasher.update(input);
    let result = hasher.finalize();
    format!("{:x}",result)
}
fn main() {
 let mut app=App::new();
 app.genesis_block();
 app.add_block("Sarvil".to_string(), "God".to_string(), 800);
 app.add_block("God".to_string(), "Sarvil".to_string(), 1000);
 app.print_chain();
}
