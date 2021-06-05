fn main() {
  println!("Hello, try4!");
  println!("--------------------------");

  //case 40
  println!("[Case 40]");
  {
    let args = std::env::args();
    println!("args info: {:?}", args);
  }

  //case 41
  println!("[Case 41]");
  {
    let args = std::env::args();
    println!("args info: {:?}", args);
    println!("--------------------------");
    println!("args info:");
    for i in args
    {
        println!("{}", i)
    }
    println!("args end");
  }

  //case 42
  println!("[Case 42]");
  {
    use std::io::stdin;
    let mut str_buff = String::new();
    stdin().read_line(&mut str_buff)
      .expect("Failed to read line.");
    println!("Input is \n{}\n", str_buff);
    println!("--------------------------");
  }
  //case xx
  println!("[Case xx]");
  {
  }

  println!("Bye, try4!");
  println!("--------------------------");
} /* End of main() */
