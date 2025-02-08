use std::thread;

use mini_redis::{client, Result};


async fn say_hello(){
    println!("hello:{:?}",thread::current().name());
}
#[tokio::main]
async fn main()->Result<()> {
    // connect to the redis 
    // let mut  client = client::connect("127.0.0.1:6379").await?;
    // set the hello key and the value
    // client.set("hello","hello world".into()).await?;
    // get the value and display the value format
    // let value = client.get("hello").await?;
    
    // println!("the value of the redis get is :{:?}",value);
    let op = say_hello();
    println!("hello:{:?}",thread::current().name());
    op.await;
    Ok(())
}
