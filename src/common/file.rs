use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

pub async fn print_banner() -> io::Result<()> {

    let mut file_path = PathBuf::from("src");
    file_path.push("resources");
    file_path.push("banner.txt");

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line?);
    }
    println!("{}", "v1.0.0");
    // println!("{}", env::var("PORT").unwrap());
    println!("Server running at http://127.0.0.1:9090");
    Ok(())
}