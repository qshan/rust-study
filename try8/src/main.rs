fn main() {
  println!("Hello, try8!");
  println!("--------------------------");

  //case 50
  println!("[Case 50]");
  {
    let mut vector_case50 = vec![1, 2, 3, 4, 5, 6, 7, 8];
    println!("{:?}", vector_case50);
    println!("--------------------------");
    vector_case50.push(16);
    vector_case50.push(26);
    vector_case50.push(66);
    println!("{:?}", vector_case50);
  }

  //case 51
  println!("[Case 51]");
  {
    let mut v1: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut v2: Vec<i32> = vec![11, 12, 13, 14, 15];
    println!("{:?}", v1);
    println!("{:?}", v2);
    println!("--------------------------");
    v1.append(&mut v2);
    println!("{:?}", v1);

  }

  //case 52
  println!("[Case 52]");
  {
    let v1: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{}",
             match v1.get(2)
             {
                 Some(value) => value.to_string(),
                 None => "None".to_string()
             }
            );
  }

  //case 53
  println!("[Case 53]");
  {
    let v1: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{}",
             v1[2]
            );

  }

  //case 54
  println!("[Case 54]");
  {
    let v1: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("v1 is :");
    for i in &v1
    {
      println!("{}", i);
    }
    println!("--------------------------");
  }

  //case 55
  println!("[Case 55]");
  {
    let mut v1: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("v1 is :");
    for i in &v1
    {
      println!("{}", i);
    }
    println!("--------------------------");
    for i in &mut v1
    {
      *i += 10;/*Here*/
    }
    println!("--------------------------");
    println!("v1 after update is :");
    for i in &v1
    {
      println!("{}", i);
    }
    println!("--------------------------");
  }

  //case 56
  //String
  println!("[Case 56]");
  {
    let str_from_float = 1.1.to_string();
    println!("str_from_float is {}", str_from_float);

    let str_from_double_quote = String::from("This is a string");
    println!("str_from_double_quote is {}", str_from_double_quote);
    println!("--------------------------");
  }

  //case 57
  //String
  println!("[Case 57]");
  {
    let mut s_case57 = String::from("This is a string");
    println!("s_case57 is {}", s_case57);
    println!("--------------------------");
    s_case57.push_str(" this is part added");/*Here*/
    println!("s_case57 is {}", s_case57);
    println!("--------------------------");
    s_case57.push('#');/*Here*/
    s_case57.push('#');/*Here*/
    println!("s_case57 is {}", s_case57);
    println!("--------------------------");
  }

  //case 58
  //String
  println!("[Case 58]");
  {
    let s1_case58 = String::from("00 This is a string");
    let s2_case58 = String::from("01 This is a string");
    println!("s1_case58 is {}", s1_case58);
    println!("s2_case58 is {}", s2_case58);
    println!("--------------------------");
    let s3_case58 = s1_case58 + &s2_case58;/*Here*/
    //println!("s1_case58 is {}", s1_case58);
    println!("s1_case58 is invisible now");/*Here*/
    println!("s2_case58 is {}", s2_case58);
    println!("s3_case58 is {}", s3_case58);
    println!("--------------------------");
  }

  //case 59
  //String
  println!("[Case 59]");
  {
    let s1_case59 = String::from("00 This is a string from Case 59");
    let s2_case59 = String::from("01 This is a string from Case 59");
    println!("s1_case59 is {}", s1_case59);
    println!("s2_case59 is {}", s2_case59);
    println!("--------------------------");
    let s3_case59 = s1_case59 +" +++++ " + &s2_case59;/*Here*/
    //println!("s1_case59 is {}", s1_case59);
    println!("s1_case59 is invisible now");/*Here*/
    println!("s2_case59 is {}", s2_case59);
    println!("s3_case59 is {}", s3_case59);
    println!("--------------------------");
  }


  //case 60
  //String
  println!("[Case 60]");
  {
    let s1_case60 = String::from("00 This is a string from Case 60");
    let s2_case60 = String::from("01 This is a string from Case 60");
    println!("s1_case60 is {}", s1_case60);
    println!("s2_case60 is {}", s2_case60);
    println!("--------------------------");

    let s3_case60 = format!("{} --- {} ---- {}", s1_case60, " +++++ ", s2_case60);/*Here*/
    //let s3_case60 = format!("{} ---  ---- {}", s1_case60, s2_case60);/*Here*/
    println!("s1_case60 is {}", s1_case60);
    println!("s2_case60 is {}", s2_case60);
    println!("s3_case60 is {}", s3_case60);
    println!("--------------------------");
  }

  //case 61
  //String
  println!("[Case 61]");
  {
    let s1_case61 = String::from("00 This is a string from Case 61");
    println!("s1_case61 is {}", s1_case61);
    println!("--------------------------");
    let s1_len = s1_case61.len();
    let s1_chars_count = s1_case61.chars().count();
    println!("s1_case61 is {}", s1_case61);
    println!("s1_case61_len is {}", s1_len);
    println!("s1_case611_chars_count is {}", s1_chars_count);
    println!("--------------------------");
  }

  //case 62
  //String
  println!("[Case 62]");
  {
    let s1_case62 = String::from("Case 62");
    println!("s1_case62 is {}", s1_case62);
    println!("--------------------------");
    for c in s1_case62.chars()
    {
      println!("{}", c);
    }
    println!("--------------------------");
  }

  //case 63
  //String
  println!("[Case 63]");
  {
    let s1_case63 = String::from("00 This is a string from Case 63");
    println!("s1_case63 is {}", s1_case63);
    let a = s1_case63.chars().nth(1);
    println!("a is {:?}", a);
    println!("--------------------------");
  }

  //case 64
  //String
  println!("[Case 64]");
  {
    let s1_case64 = String::from("00 This is a string from Case 64");
    println!("s1_case64 is {}", s1_case64);
    let a = &s1_case64[5..9];/*Here*/
    println!("slice[5..9] is {:?}", a);
    println!("--------------------------");
  }

  //case 65
  //hash map
  println!("[Case 65]");
  {
    use std::collections::HashMap;

    let mut map_case65 = HashMap::new();
    map_case65.insert("color", "red");
    map_case65.insert("size", "5x2");
    println!("color is {}", map_case65.get("color").unwrap());/*Here*/
    println!("size is {}", map_case65.get("size").unwrap());
    println!("--------------------------");
  }

  //case 66
  //hash map
  println!("[Case 66]");
  {
    use std::collections::HashMap;

    let mut map_case66 = HashMap::new();
    map_case66.insert("color", "red");
    map_case66.insert("size", "5x2");
    println!("go through map_case66 now");
    for p in map_case66.iter()
    {
        println!("{:?}", p);
    }
    println!("--------------------------");
  }


  //case 67
  //hash map
  println!("[Case 67]");
  {
    use std::collections::HashMap;

    let mut map_case67 = HashMap::new();
    map_case67.insert(0, "red");
    map_case67.insert(1, "5x2");
    map_case67.insert(2, "200");
    println!("go through map_case67 now");
    for p in map_case67.iter()
    {
        println!("{:?}", p);
    }
    println!("--------------------------");
    if let Some(x) = map_case67.get_mut(&1)
    {
      *x = "b";
    }
    println!("go through map_case67 after update");
    for p in map_case67.iter()
    {
        println!("{:?}", p);
    }
    println!("--------------------------");
  }

  //case xx
  println!("[Case xx]");
  {
  }

  println!("Bye, try8!");
  println!("--------------------------");
} /* End of main() */
