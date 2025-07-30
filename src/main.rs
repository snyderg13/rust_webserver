use std::io::{BufRead, Write};

fn main() {
    let debug = true;
    let hello_test = false;

    println!("Hello, world!");

    let listener = std::net::TcpListener::bind("127.0.0.1:9999").unwrap();

    for mut stream in listener.incoming().flatten() {
        let mut reader = std::io::BufReader::new(&mut stream);

        let mut line = String::new();
        reader.read_line(&mut line).unwrap();

        match line.trim().split(' ').collect::<Vec<_>>().as_slice() {
            ["GET", resource, "HTTP/1.1"] => {
                loop {
                    let mut line = String::new();
                    reader.read_line(&mut line).unwrap();
                    if line.trim().is_empty() {
                        break;
                    }
                    print!("line = {line}");
                }

                let mut res_path = std::path::PathBuf::new();
                res_path.push("/var/www/html");

                if debug {
                    let test_path = res_path.display();
                    println!("res_path = {test_path}");
                }

                res_path.push(resource.trim_start_matches("/"));

                if debug {                
                    let test_path = res_path.display();
                    println!("res_path = {test_path}");
                }

                if resource.ends_with('/') {
                    res_path.push("index.html");
                }

                if hello_test {
                    stream.write_all(b"HTTP/1.1 200 OK\r\n\r\nHello World!").unwrap();
                } else {
                    stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
                    stream.write_all(&std::fs::read(res_path).unwrap()).unwrap();
                }
            },
            _ => todo!()
        }

    }
}
