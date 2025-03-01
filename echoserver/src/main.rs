use std::error::Error;
use std::io::{Read,Write};
use std::net::TcpListener;
use std::{env,str,thread};

fn main()->Result<(),Box<dyn Error>>{
    let args:Vec<String>=env::args().collect();
    let addr = &args[1];
    echo_server(addr)?;
    Ok(())
}

fn echo_server(addr:&str)->Result<(),Box<dyn Error>>{
    let listener = TcpListener::bind(addr)?; //1 bind to the address
    loop{
        let (mut stream,_) = listener.accept()?; //2 accept the connection
        thread::spawn(move ||{  //3 spawn a thread to handle the connection
            let mut buffer = [0u8;1024];
            loop{
                let nbytes = stream.read(&mut buffer).unwrap(); //4 read from the stream
                if nbytes == 0{  //6 if no bytes are read, the connection is closed
                    return;
                }
                print!("{}",str::from_utf8(&buffer[..nbytes]).unwrap());
                stream.write_all(&buffer[..nbytes]).unwrap(); //5 write to the stream
            }
        });
    }
}