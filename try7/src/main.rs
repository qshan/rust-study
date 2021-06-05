use std::io::prelude::*;
use std::fs::OpenOptions;

fn main() -> std::io::Result<()> {
  println!("Hello, try7!");
  println!("--------------------------");

  //case 48
  println!("[Case 49]");

    let file_test_case49: String = String::from("/home/fshan/data/work/rustspace/try7/rust_test_case48.txt");
    //let mut file_case49 = std::fs::OpenOptions::new()
    let mut file_case49 = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(&file_test_case49)
        ?;
    //let mut file_case49 = OpenOptions::create().append(true).open(&file_test_case48)?;

    file_case49.write(b"00 try write from case 49\n")?;
    file_case49.write(b"01 try write from case 49\n")?;

    Ok(())

  //println!("Bye, try7!");
  //println!("--------------------------");
}
