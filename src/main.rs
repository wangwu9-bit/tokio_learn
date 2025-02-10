

use mini_redis::{Connection, Frame};
use tokio::{io::AsyncReadExt, net::{TcpListener, TcpStream}};




#[tokio::main]
async fn main(){
    // first connect to the local redis use the TCP
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    // read the frame
    loop{
        // accept the connect
        let (socket,_) = listener.accept().await.unwrap();

        // process the socket 
        process(socket).await;
    }
}

async fn process(socket:TcpStream){
    let mut connect = Connection::new(socket);
    if let Some(frame) = connect.read_frame().await.unwrap(){
        println!("GOT {:?}",frame);

        let response = Frame::Error("unimplemented".to_string());
        connect.write_frame(&response).await.unwrap();
    }
}