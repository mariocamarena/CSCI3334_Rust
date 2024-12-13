use crate::monitor::run_monitor;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

fn mock_server(mut stream: TcpStream, status_code: u16){
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let response;
    if status_code == 200 {
        response = "HTTP/1.1 200 OK\r\nContent-Length: 12\r\n\r\nHello World";
    } 
    else {
        response = "HTTP/1.1 500 Internal Server Error\r\nContent-Length: 12\r\n\r\nError World";
    }
    //response = "HTTP/1.1 400 Bad Request\r\nContent-Length: 12\r\n\r\nBad Request";

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

#[test]
fn test_mock_server() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let server_address =listener.local_addr().unwrap();

    thread::spawn(move ||{
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            mock_server(stream, 200); }});

    let urls = vec![format!("http://{}", server_address)];
    let results = run_monitor(urls,Duration::from_secs(5), 1, 1);

    let result = &results[0];
    assert!(result.status.is_ok());
    assert_eq!(result.status.as_ref().unwrap(), &200);
}

#[test]
fn test_failed_requests() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let server_address = listener.local_addr().unwrap();

    thread::spawn(move || {
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            mock_server(stream, 500); }});

    let urls = vec![format!("http://{}", server_address)];
    let results = run_monitor(urls, Duration::from_secs(5), 1, 2);

    let result = &results[0];
    assert!(result.status.is_err());
}

#[test]
fn test_concurrent_requests() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    thread::spawn(move || {
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            mock_server(stream, 200); 
        }});


    let mut urls = vec![];
    for i in 1..=10 {
        urls.push(format!("http://127.0.0.1:8080/test{}", i));
    }

    let results = run_monitor(urls, Duration::from_secs(5), 2, 1);

    for res in results {
        assert!(res.status.is_ok());
    }
}
