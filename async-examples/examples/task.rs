use async_std::fs::File;
use async_std::io;
use async_std::prelude::*;
use async_std::task;

async fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;

    Ok(contents)
}

fn main() {
    let reader_task = task::spawn(async {
        let result = read_file("Cargo.toml").await;
        match result {
            Ok(s) => println!("{}", s),
            Err(e) => println!("Error reading file: {:?}", e),
        }
    });
    println!("Started task!");
    task::block_on(reader_task);
    println!("Stopped task!");
}
