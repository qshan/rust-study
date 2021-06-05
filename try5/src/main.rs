fn main() {
  println!("Hello, try5!");
  println!("--------------------------");
  //let file_test_try5: String = String::from("/home/fshan/data/work/rustspace/rust_test.txt");
  let file_test_try5: String = String::from("/home/fshan/data/work/rustspace/rust-study/try5/rust_test.txt");
 //case 43
  println!("[Case 43]");
  {
    use std::fs;
    let contents_case43 = fs::read("/home/fshan/data/work/rustspace/rust-study/try5/rust_test.txt").unwrap();
    println!("{:?}", contents_case43);
  }

  //case 44
  println!("[Case 44]");
  {
    use std::io::prelude::*;
    use std::fs;
    let mut buffer_case44 = [0u8; 5];
    let mut file_case44 = fs::File::open("/home/fshan/data/work/rustspace/rust-study/try5/rust_test.txt").unwrap();
    file_case44.read(&mut buffer_case44).unwrap();
    println!("{:?}", buffer_case44);
    println!("--------------------------");
    file_case44.read(&mut buffer_case44).unwrap();
    println!("{:?}", buffer_case44);
    println!("--------------------------");
  }

  //case 45
  println!("[Case 45]");
  {
    use std::io::prelude::*;
    use std::fs;

    let file_test_case45: String = String::from("/home/fshan/data/work/rustspace/rust-study/try5/rust_test.txt");
    //println!("{}", file_test_case45);
    let mut buffer_case45 = [0u8; 5];
    //let mut file_case45 = fs::File::open("/home/fshan/data/work/rustspace/rust-study/try5/rust_test.txt").unwrap();
    let mut file_case45 = fs::File::open(&file_test_case45).unwrap();
    file_case45.read(&mut buffer_case45).unwrap();
    println!("{:?}", buffer_case45);
    println!("--------------------------");
    file_case45.read(&mut buffer_case45).unwrap();
    println!("{:?}", buffer_case45);
    println!("--------------------------");
  }

  //case 46
  println!("[Case 46]");
  {
    use std::fs;

    let file_test_case46: String = String::from("/home/fshan/data/work/rustspace/rust-study/try5/rust_test.txt");
    fs::write(&file_test_case46, "try write from case 46").unwrap();
  }

  //case 47
  println!("[Case 47]");
  {
    use std::io::prelude::*;
    use std::fs::File;

    let file_test_case47: String = String::from("/home/fshan/data/work/rustspace/try5/rust_test_case47.txt");
    let mut file_case47 = File::create(&file_test_case47).unwrap();
    file_case47.write(b"try write from case 47").unwrap();
  }

  //case xx
  println!("[Case xx]");
  {
  }

  //static file_test_try5: String = String::from("/home/fshan/data/work/rustspace/rust-study/try5/rust_test.txt");
  //worked//let file_test_try5: String = String::from("/home/fshan/data/work/rustspace/rust-study/try5/rust_test.txt");
  println!("file_test_try5 is {}", file_test_try5);
  println!("Bye, try5!");
  println!("--------------------------");
} /* End of main() */
