#![allow(non_snake_case)]

use futures::executor::block_on;
use futures::task::Poll::Ready;


fn main() {
    println!("\nstarted..");
    
    let futu = generateString();
    block_on(futu);
    
    match futu {
        Pending => println!("<--> pending"),
        Ready(()) => println!("<--> ready: {}", "<>"),
    }

    let a = "<>";
    println!("test output: {}", a);

    println!("..ended");
}


async fn generateString() { //-> String {
    println!("[inside]");
    //"gen str".to_string()
}
