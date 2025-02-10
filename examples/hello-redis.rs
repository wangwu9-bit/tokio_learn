use std::thread;

use tokio::time::{sleep, Duration};
use mini_redis::{client, Result};

#[allow(dead_code)]
async fn say_hello() {
    println!("Starting hello");
    sleep(Duration::from_secs(2)).await;  // Simulates some async work
    println!("Finished hello");
}
#[allow(dead_code)]
async fn diff_thread(){
    for i in 0..3{
        println!("Before sleep {} on thread:{:?}",i,thread::current().id());
        thread::sleep(Duration::from_nanos(2));
        println!("After sleep {} on thread:{:?}",i,thread::current().id());
    }
}
#[tokio::main]
async fn main()->Result<()> {
    // connect to the redis 
    let mut  client = client::connect("127.0.0.1:6379").await?;
    // set the hello key and the value
    client.set("hello","hello world".into()).await?;
    // get the value and display the value format
    let value = client.get("hello").await?;
    // diff_thread().await;
    println!("the value of the redis get is :{:?}",value);
    // let op = say_hello();
    // println!("before hello");
    // op.await;
    // println!("hello1:{:?}",thread::current().name());
    Ok(())
}
