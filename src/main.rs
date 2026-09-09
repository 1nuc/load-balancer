use std::{
    error::Error, fmt::format, io::{BufRead, BufReader, BufWriter, Read, Write}, net::{TcpListener, TcpStream},
};

fn main() {
    //preparing the server
    let server = TcpListener::bind("localhost:8808").expect("Error while creating the server");
    for stream in server.incoming() {
        match stream{
            Ok(str) =>{
                let req_body=handle_request(&str);
                handle_response(str);
                println!("{req_body:#?}")
            },
            Err(err) => eprint!("{err:?}"),
        }
    }
    println!("Hello, world!");
}

fn handle_request(stream: &TcpStream) -> Vec<String>{
    let buffer = BufReader::new(stream);
    buffer
        .lines()
        .map(|x| x.unwrap())
        .take_while(|x| !x.is_empty())
        .collect::<Vec<_>>()
}
fn handle_response(mut stream: TcpStream){
    //this is the singature for the get request response HTTP/1.1 200 Ok \r\n\r\n
    let res="HTTP/1.1 200 Ok \r\n\r\n the message is received now";
    stream.write_all(res.as_bytes()).expect("error in right the response");
}
