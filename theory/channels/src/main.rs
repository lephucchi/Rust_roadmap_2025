use crossbeam_channel::unbounded;
use std::thread;

enum Message{
    PrintMsg(String),
    Sum(i32,i32),
    Quit(),
}

enum MainMsg{
    
}


fn main(){

    let(send , receive) = unbounded();

    let a = thread::spawn(move|| loop{
        match receive.recv(){
            Ok(msg) => match msg{
                Message::PrintMsg(data) => println!("{}" , data),
                Message::Sum(a,b) => println!("{}+{}={}", a , b , a+b),
                Message::Quit() => {
                    println!("Program ended");
                    break;
                }
            }
            Err(e) => {
                println!("Erro data: {:?}" , e );
                break;
            }
        }
    });

    send.send(Message::PrintMsg("Hello Chible".to_string()));
    send.send(Message::Sum(5,3));
    send.send(Message::Quit());

    a.join();
}