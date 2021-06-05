use std::io::prelude::*;
use std::fs::OpenOptions;

fn main() -> std::io::Result<()> {
  println!("Hello, try6!");
  println!("--------------------------");

  //case 48
  println!("[Case 48]");

    let file_test_case48: String = String::from("/home/fshan/data/work/rustspace/try6/rust_test_case48.txt");
    //let mut file_case48 = std::fs::OpenOptions::new()
    let mut file_case48 = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_test_case48)
        ?;

    //let mut file_case48 = OpenOptions::create().append(true).open(&file_test_case48)?;

    file_case48.write(b"00 try write from case 48\n")?;
    file_case48.write(b"01 try write from case 48\n")?;

    Ok(())


  //println!("Bye, try6!");
  //println!("--------------------------");
}
