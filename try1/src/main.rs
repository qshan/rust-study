fn main() {

    println!("Hello, try1!");

    //case 1
    println!("[Case 1]");
    let a = 12;
    let b = 12.5;
    println!("Hello, world!");
    println!("a is {}", a);
    println!("b is {}", b);
    println!("a and b is {} and {}", a, b);

    //case 2
    println!("[Case 2]");
    let mut c = 567;
    println!("c is {} before update", c);
    c = 123;
    println!("c is {} after update", c);

    //case 3
    println!("[Case 3]");
    let mut d: u32 = 901;
    println!("d is {} in current status", d);
    d = 9011;
    println!("d is {} before update", d);
    let d: u64 = 9014;
    println!("d is {} after update", d);

    //case 4
    println!("[Case 4]");
    let mut s = "abc";
    println!("s is {} in current status", s);
    s = "cde";
    println!("s is {} in current status", s);
    let mut s = 123;
    println!("s is {} before update", s);
    s = 9014;
    println!("s is {} after update", s);

    //case 5
    println!("[Case 5]");
    let mut e: i128 = 123;
    println!("e is {} before update", e);
    e = 9014;
    println!("e is {} after update", e);
    //
    let mut e: u128 = 123;
    println!("e is {} before update", e);
    e = 9014;
    println!("e is {} after update", e);
    //
    let mut e: u64 = 123_456;
    println!("e is {} before update", e);
    e = 9014_234;
    println!("e is {} after update", e);
    //
    let mut e: u64 = 0xff;
    println!("e is {} before update", e);
    e = 0xee;
    println!("e is {} after update", e);
    //
    let mut e: u64 = 0o77;
    println!("e is {} before update", e);
    e = 0o66;
    println!("e is {} after update", e);
    //
    let mut e: u64 = 0b1111_0000;
    println!("e is {} before update", e);
    e = 0b1010_1010;
    println!("e is {} after update", e);
    //
    let mut e: f64 = 123.0;
    println!("e is {} before update", e);
    e = 9014.0;
    println!("e is {} after update", e);
    //
    //case 6
    println!("[Case 6]");
    let data_tup: (f64, u32, i32) = (12.0, 32, 32);
    println!("data_tup.0 is {} before update", data_tup.0);
    println!("data_tup.1 is {} before update", data_tup.1);
    println!("data_tup.2 is {} before update", data_tup.2);
    let (x, y, z) = data_tup;
    println!("x is {} after update", x);
    println!("y is {} after update", y);
    println!("z is {} after update", z);
    //
    //case 7
    println!("[Case 7]");
    let f = [1,2,3,4,5];
    println!("f[0] is {}", f[0]);
    let f = [3; 5];
    println!("f[3] is {}", f[3]);
    let f: [u32; 5] = [1, 2, 3, 4, 5];
    println!("f[3] is {}", f[3]);
    //
    let mut g = [1, 2, 3, 4, 5];
    println!("g[2] is {} before update", g[2]);
    g[2] = 5;
    println!("g[2] is {} after update", g[2]);
    //
    //case 7
    println!("[Case 7]");
    println!("try comment code in rust");
    //example of mask a line
    /*example of mask a piece of code*/
    /*example of mask multi-lines here
      example of mask multi-lines here
     */
    //
    //case 8
    println!("[Case 8]");
    //fn example_function_case8()
    //{
    //    println!("Here is the a example of a function");
    //}
    example_function_case8();
    ////
    //fn example_function2_case8(x: i32, y: u32)
    //{
    //    println!("Here is the a example of a function2");
    //    println!("The argument x is {}, and y is {}", x, y);
    //}
    let mut h: u64;
    h = example_function2_case8(123, 456);
    println!("h is {}", h);
    h = 0;
    println!("h is {}", h);
    //
    //case 9
    println!("[Case 9]");
    let number  = 10;
    let number1 = 6;
    if number < 8
    {
      println!("The condition is True");
    }
    else if number1 > 2
      {
            println!("The condition2 is True");
      }
      else
      {
            println!("The condition2 is False");
      }
    //
    //case 10
    println!("[Case 10]");
    let mut number_while: u32 = 1;
    while number_while < 5
    {
      println!("Current number_while is {}", number_while);
      number_while += 1;
    }
    println!("End of while example here");
    //
    //case 11
    println!("[Case 11]");
    let j = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for i in j.iter()
    {
        println!("i is {}", i)
    }
    //
    for i in 0..5
    {
      println!("i is {}", i)
    }
    println!("End of for example here");
    //
    //case 12
    println!("[Case 12]");
    let j = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut i = 0;
    loop
    {
      println!("i is {}", i);
      println!("j[{}] is {}", i, j[i]);
      if i > 5
      {
        break;
      }
      i += 1;
    }
    println!("End of loop example here");

    println!("Bye, try1!");
  /* -------------------------------------------------- */
} /* End of main() */

  fn example_function_case8()
  {
    println!("Here is the a example of a function");
    println!("----------End-of-example_function_case8");
  }
  //
  fn example_function2_case8(x: u32, y: i32) -> u64
  {
    println!("Here is the a example of a function2");
    println!("The argument x is {}, and y is {}", x, y);
    println!("----------End-of-example_function2_case8");
    return x as u64 + y as u64;
  }
