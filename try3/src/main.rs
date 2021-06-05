fn main() {
  println!("Hello, try3!");

  //case 32
  println!("[Case 32]");
  {
    struct PointCase32<T>
    {
      x: T,
      y: T,
    }

    impl<T> PointCase32<T>
    {
      fn x(&self) -> &T
      {
        &self.x
      }
      fn y(&self) -> &T
      {
        &self.y
      }
    }

    let p1 = PointCase32 {x: 20, y: 30};
    println!("p1.x is {}", p1.x());
    println!("p1.y is {}", p1.y());
    println!("--------------------------");
    let p2 = PointCase32 {x: 20.1, y: 30.2};
    println!("p2.x is {}", p2.x());
    println!("p2.y is {}", p2.y());

  }


  //case 33
  println!("[Case 33]");
  {
//    struct PointCase33<T, U>
//    {
//      x: <T>,
//      y: <U>,
//    }
//
//    impl <T, U> PointCase33<T, U>
//    {
//        fn mixup<V, W>(self, other: PointCase33<V, W>) -> PointCase33<T, W>
//        {
//            PointCase33
//            {
//                x: self.x,
//                y: others.y,
//            }
//        }
//    }
//
//    let p1 = PointCase33 {x: 20, y: 30};
  }

  //case 35
  println!("[Case 35]");
  {
    trait Descriptive
    {
      fn descriptive(&self) -> String
      {String::from("[Object]")}
    }

    struct PersonCase35
    {
      name: String,
      age:  u8
    }

    impl Descriptive for PersonCase35
    {
      fn descriptive(&self) -> String
      {
          format!("{} is {}", &self.name, &self.age)
      }
    }

    let xiaowang  = PersonCase35{name: "XiaoWang".to_string(), age: 20};
    println!("{} years old", xiaowang.descriptive() );
    println!("--------------------------");
    let xiaozhang = PersonCase35{name: String::from("XiaoZhang"), age: 20};
    println!("{} years old", xiaozhang.descriptive() );
  }

  //case 36
  println!("[Case 36]");
  {
      trait ComparableCase36
      {
          fn compare_case36(&self, object: &Self) -> i8;
      }

      fn max_case36<T: ComparableCase36>(array: &[T]) -> &T
      {
          let mut max_index = 0;
          let mut i = 1;
          while i < array.len()
          {
              if array[i].compare_case36(&array[max_index]) > 0
              {
                  max_index = i;
              }
              i += 1;
          }
          //worked/*ToCheck*/&array[max_index]
          return &array[max_index];
      }

      impl ComparableCase36 for f64
      {
          fn compare_case36(&self, object: &f64) -> i8
          {
              if &self > &object { 1 }
              else if &self == &object { 0 }
              else { -1 }
          }
      }

      let arr = [1.0, 3.0, 5.0, 4.0, 2.0, 8.0, 6.0];
      println!("maximum of arr is {}", max_case36(&arr));
  }

  //case 37
  println!("[Case 37]");
  {
      //fn longer_case37(s1: &str, s2: &str) -> &str
      fn longer_case37<'a>(s1: &'a str, s2: &'a str) -> &'a str
      {
          if s2.len() > s1.len()
          {
              s2
          }
          else
          {
              s1
          }
      }

      let r;
      {
          let s1 = "rust";
          let s2 = "emacsenv";
          r = longer_case37(s1, s2);
          //just one line here//r = longer_case37(&s1, &s2);
      }
      println!("{} is longer", r)
  }

  //case 38
  println!("[Case 38]");
  {
    struct StrCase38<'a>
    {
      contents: &'a str
    }

    let s = StrCase38{contents: "string_slice"};
    println!("s.contents is {}", s.contents);
    println!("--------------------------");

    impl <'a> StrCase38<'a>
    {
      fn get_content_case38(&self) -> &str
      {
        self.contents
      }
    }

    let s1 = StrCase38{contents: "string_slice_update"};
    println!("s1.contents is {}", s1.contents);
    println!("s1.contents is {}", s1.get_content_case38());
  }

  //case 39
  println!("[Case 39]");
  {
    //ToCheck//use std::fmt::Display;
    //ToCheck//fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T: Display
    //ToCheck//{
    //ToCheck//  println!("Announcement is {}", ann);
    //ToCheck//  if x.len() > y.len()
    //ToCheck//  { x }
    //ToCheck//  else
    //ToCheck//  { y }
    //ToCheck//}
  }

  //case xx
  println!("[Case xx]");
  {
  }

  println!("Bye, try3!");
} /* End of main() */
