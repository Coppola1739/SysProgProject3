use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

mod lib;

fn main() {
    let data_folder = "data";
    if !Path::new(data_folder).exists() {
        fs::create_dir(data_folder).expect("Failed to create data folder");
    }

    start_listening("127.0.0.1:8080").unwrap();
}

fn start_listening(bind_address: &str) -> io::Result<()> {
    let listener = TcpListener::bind(bind_address)?;
    let shared_data_folder = Arc::new(Mutex::new("data"));

    for stream in listener.incoming() {
        let shared_data_folder = Arc::clone(&shared_data_folder);

        thread::spawn(move || {
            if let Ok(mut stream) = stream {
                let client_address = stream.peer_addr().unwrap();
                println!("Connected to client: {}", client_address);

                handle_client(&mut stream, &shared_data_folder).unwrap();
            }
        });
    }

    Ok(())
}

fn handle_client(stream: &mut TcpStream, data_folder: &Arc<Mutex<&str>>) -> io::Result<()> {
    let mut branch_code = String::new();
    stream.read_to_string(&mut branch_code)?;
    println!("Received branch code: {}", branch_code.trim());


    let branch_folder_path = format!("{}/{}", data_folder.lock().unwrap(), branch_code.trim());
    fs::create_dir_all(&branch_folder_path).expect("Failed to create branch folder");

    stream.write_all(b"OK")?;

    let mut file_content = String::new();
    stream.read_to_string(&mut file_content)?;
    println!("Received Base64 content: {}", file_content);
    stream.write_all(b"OK")?;

    let cleaned_content = file_content.trim_start_matches('~').trim_end_matches('~');
    let decoded_content = lib::decode_from_base64(cleaned_content);
    let file_path = format!("{}/branch_weekly_sales.txt", branch_folder_path);
    fs::write(&file_path, decoded_content).expect("Failed to write file");

    println!("Saved decoded content to: {}", file_path);

    Ok(())
}
