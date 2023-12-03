use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::fs::File;
use std::path::Path;

mod lib;

fn main() {
    start_data_transfer("127.0.0.1:8080", "ALBNM", "data/ALBNM/branch_weekly_sales.txt").unwrap();
}

fn start_data_transfer(server_address: &str, branch_code: &str, file_path: &str) -> io::Result<()> {
    let file_content = read_file_content(file_path)?;
    let encoded_content = lib::encode_to_base64(&file_content);
    let mut stream = TcpStream::connect(server_address)?;
    let branch_code_msg = format!("bcode~{}\n", branch_code);
    stream.write_all(branch_code_msg.as_bytes())?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    println!("{}", response.trim());

    if response.trim() == "OK" {
        let file_msg = format!("~{}~\n", encoded_content);
        stream.write_all(file_msg.as_bytes())?;

        response.clear();
        stream.read_to_string(&mut response)?;
        println!("{}", response.trim());

        drop(stream);
    }

    Ok(())
}

fn read_file_content(file_path: &str) -> io::Result<String> {
    let path = Path::new(file_path);
    let mut file = File::open(&path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}
