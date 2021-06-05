fn main() {

	println!("Hello, try2!");

	//case 13
	println!("[Case 13]");
	//ownership of variable
	{
		let j = 9;
		println!("here j is {}", j);
	}
		println!("here j is invisible");

	//case 14
	println!("[Case 14]");
	//ownership of variable
	{
		let s1 = String::from("hello");
		println!("here s1 is {}", s1);
		let s2 = s1;
		//println!("here s1 is {}", s1);
		println!("here s1 invisible");
		println!("here s2 is {}", s2);
	}

	//case 15
	println!("[Case 15]");
	//ownership of variable
	{
		let s1 = String::from("hello");
		println!("here s1 is {}", s1);
		take_ownership_case15(s1);
		//println!("here s1 is {}", s1);
		println!("here s1 is invisible");

		let k = 5;
		println!("here k is {}", k);
		make_copy_case15(k);
		println!("here k is {}", k);
	}

	//case 16
	println!("[Case 16]");
	//ownership of variable with function
	{
		let s1 = give_ownership_case16();
		println!("here s1 is {}", s1);

		let s2 = String::from("hello_s2");
		println!("here s2 is {}", s2);

		let s3 = take_and_give_ownership_case16(s2);
		//let s3: String = take_and_give_ownership_case16(s2);
		println!("here s1 is {}", s1);
		//println!("here s1 is invisible");
		//println!("here s2 is {}", s2);
		println!("here s2 is invisible");
		println!("here s3 is {}", s3);
		//println!("here s3 is invisible");
	}

	//case 17
	println!("[Case 17]");
	{
		let s1 = String::from("hello_case17");
		println!("here s1 is {}", s1);
		let s2 = &s1;
		println!("here s1 is {}", s1);
		println!("here s2 is {}", s2);
	}

	//case 18
	println!("[Case 18]");
	{
		let mut s1 = String::from("hello_case18_s1");
		println!("here s1 is {} before borrow", s1);
		println!("--------------------------");
		let mut s2 = &s1;
		println!("here s1 is {}", s1);
		println!("here s2 is {}", s2);
		//mask//s1 = String::from("hello_case18_updated");
		println!("here s1 could not be update after borrow");
		println!("--------------------------");
		let s3 = String::from("hello_case18_s3");
		s2 = &s3;
		println!("here s1 is {}", s1);
		println!("here s1 could be update after s2 borrow s3");
		println!("here s2 is {}", s2);
		println!("here s3 is {}", s3);
		println!("--------------------------");
		s1 = String::from("hello_case18_s1_updated");
		println!("here s1 is {}", s1);
		println!("here s2 is {}", s2);
		println!("here s3 is {}", s3);
	}

	//case 19
	println!("[Case 19]");
	{
		let s1 = String::from("hello_case19_s1");
		println!("here s1 is {}", s1);
		println!("--------------------------");
		//s2 borrow ownership from s1
		let mut s2 = &s1;
		println!("here s1 is {}", s1);
		println!("here s2 is {}", s2);
		println!("--------------------------");
		//TODO//let s3 = s1;
		let s3 = s2;
		//s3 get the borrow ownership from s2
		println!("here s1 is {}", s1);
		println!("here s2 is {}", s2);//TODO//why s2 is here and accessable after
		//println!("here s2 is invisible now");
		println!("here s3 is {}", s3);
		println!("--------------------------");
		//s2 re-borrow ownership from s3
		s2 = &s3;
		println!("here s1 is {}", s1);
		println!("here s2 is {}", s2);
		println!("here s3 is {}", s3);
	}

	//case 20
	println!("[Case 20]");
	{
		let s1 = String::from("0123456789abcdef_case20_s1");
		let part1 = &s1[0..4];
		let part2 = &s1[5..9];
		println!("part1 is {}", part1);
		println!("part2 is {}", part2);
		println!("--------------------------");
		let part3 = &s1[..4];
		let part4 = &s1[5..];
		println!("part3 is {}", part3);
		println!("part2 is {}", part4);
	}

	//case 21
	println!("[Case 21]");
	{
		let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
		let part1 = &array[0..4];
		println!("part1 is :");
		for i in part1.iter()
		{
			println!("{}", i);
		}
		println!("--------------------------");
		//let part2 = &array[..4];
		let part3 = &array[5..];
		println!("part3 is :");
		for i in part3.iter()
		{
			println!("{}", i);
		}
	}

	//case 22
	println!("[Case 22]");
	{
		struct StudentStruct
		{
			stu_name: String,
			//age: u32
			stu_age:  u32
		}//Here

		let zhangsan = StudentStruct
		{
			stu_name:  "Zhang San".to_string(),
			stu_age:   19
		};

		println!("Zhang San's Name is {}", zhangsan.stu_name);
		println!("Zhang San's age is {}", zhangsan.stu_age);
	}

	//case 23
	println!("[Case 23]");
	{
		struct Color(u32, u32, u32, u32, u32, u32);//Here
		let color1 = Color (0, 1, 2, 3, 4, 5);

		println!("color1is( {}, {}, {}, {}, {}, {})", color1.0, color1.1, color1.2, color1.3, color1.4, color1.5);
	}

	//case 24
	println!("[Case 24]");
	{
		#[derive(Debug)]
		struct StudentStruct1
		{
			stu_time: u32,
			//age: u32
			stu_age:  u32,
		}//Here

		let zhangsan1 = StudentStruct1
		{
			stu_time:  2015,
			stu_age:   19
		};
		println!("Zhang San is {:#?}", zhangsan1);
	}

	//case 25
	println!("[Case 25]");
	{

		let zhangsan_case25 = StudentStructCase25
		{
			stu_name:  "Zhang San".to_string(),
			stu_enter_year:  2015,
			stu_exit_year:  2019
		};

		println!("Zhang San's Name is {}", zhangsan_case25.stu_name);
		println!("Zhang San's age is {}", zhangsan_case25.area());
	}

	//case 26
	println!("[Case 26]");
	{
		#[derive(Debug)]
		struct RectangleCase26
		{
			width:   u32,
			height:  u32,
		}//Here

		impl RectangleCase26
		{
			//count on #[derive(Debug)]
			fn create(width: u32, height: u32) -> RectangleCase26
			{
				RectangleCase26 {width, height}
			}

			fn area(width: u32, height: u32) -> u32
			{
				width * height
		}
	}

		let rect_case26 = RectangleCase26::area(30, 50);
		println!("{:?}", rect_case26);
		let rect_create_case26 = RectangleCase26::create(30, 50);
		//count on #[derive(Debug)]
		println!("{:?}", rect_create_case26);
	}

	//case 27
	println!("[Case 27]");
	{
		#[derive(Debug)]
		enum BookCase27 {Type1, Type2, Type3}
		let book1 = BookCase27::Type1;
		let book2 = BookCase27::Type2;
		let book3 = BookCase27::Type3;
		println!("{:?}", book1);
		println!("{:?}", book2);
		println!("{:?}", book3);
	}

	//case 28
	//match
	println!("[Case 28]");
	{
		//#[derive(Debug)]
		enum BookCase28
			{
					Papery {index: u32},
					Electronic {url: String},
			}

		let book  = BookCase28::Papery {index: 1001};
		let ebook = BookCase28::Electronic {url: String::from("url...")};

		match ebook
		{
			BookCase28::Papery {index} =>
			{
				println!("book Papery {}", index);
			}

			BookCase28::Electronic {url} =>
			{
				println!("ebook Electronic {}", url);
			}
		}

		match book
		{
			BookCase28::Papery {index} =>
			{
				println!("book Papery {}", index);
			}

			BookCase28::Electronic {url} =>
			{
				println!("ebook Electronic {}", url);
			}
		}
	}

	//case 29
	//match
	println!("[Case 29]");
	{
		let t = "abc";
			match t
			{
				"abc" => println!("Yes, it is abc"),
				_     => {println!("no matched item")},
			}
	}

	//case 30
	//match
	println!("[Case 30]");
	{
		enum Option<T>
		{
				Some(T),
				None,
		}

		let opt1 = Option::Some("hello_case30");
		match opt1
		{
			Option::Some(something) =>
			{
				println!("it is {}", something);
			},
			Option::None =>
			{
				println!("it is Nothing");
			}
		}

		let opt2: Option<&str> = Option::None;
		match opt2
		{
			Option::Some(something) =>
			{
				println!("it is {}", something);
			},
			Option::None =>
			{
				println!("it is Nothing");
			}
		}
	}

	//case 31
	//if let
	println!("[Case 31]");
	{
		let i = 0;
		if let 0 = i //here, attention to the formate,if let Value = VariableName
		{
			println!("i is 0 here");
		}

		enum BookCase31
		{
			Papery(u32),
			Electronic(String)
		}

		let book = BookCase31::Electronic(String::from("url"));
		if let BookCase31::Papery(index) = book
		{
			println!("it Papery {}", index);
		}else
		{
			println!("No Papery book");
		}

		let book1 = BookCase31::Papery(1001);
		if let BookCase31::Papery(index) = book1
		{
			println!("it Papery {}", index);
		}else
		{
			println!("No Papery book");
		}
	}

	println!("Bye, try2!");
} /* End of main() */
fn take_ownership_case15(var_str: String)
{
		println!("here var_str is {}", var_str);
}

fn make_copy_case15(var_int: i32)
{
		println!("here var_int is {}", var_int);
}

fn give_ownership_case16() -> String
{
	let s1 = String::from("hello_case16");
	return s1;
}

fn take_and_give_ownership_case16(var_str: String) -> String
{
		return var_str;
}

struct StudentStructCase25
{
	stu_name: String,
	stu_enter_year:  u32,
	stu_exit_year:  u32
}//Here

impl StudentStructCase25
{
		fn area(&self) -> u32 {self.stu_exit_year - self.stu_enter_year}
}
