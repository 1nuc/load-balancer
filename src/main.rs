use std::{
    error::Error, io::{BufRead, BufReader, BufWriter, Read, Write}, net::{TcpListener, TcpStream},
};

fn main() {
    //preparing the server
    let server = TcpListener::bind("localhost:8808").expect("Error while creating the server");
    for stream in server.incoming() {
        match stream{
            Ok(str) =>{
                handle_request(str);},
            Err(err) => eprint!("{err:?}"),
        }
    }
    println!("Hello, world!");
}

fn handle_request(stream: TcpStream) -> Vec<String>{
    let buffer = BufReader::new(stream);
    buffer
        .lines()
        .map(|x| x.unwrap())
        .take_while(|x| x.is_empty())
        .collect::<Vec<_>>()
}
