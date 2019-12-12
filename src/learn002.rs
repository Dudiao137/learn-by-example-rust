fn main () {
	
	println!("{} days", 31);
	println!("{} days", 31i64);

	println!("{0}, this is {1}, {1} this is {0}", "alice", "bob");

	println!("{subject} {verb} {object}",
				object = "the lazy dog",
				subject = "the quick brown fox",
				verb = "jumps over");

	println!("{0} of {0:b}", 2);

	println!("{number:>width$}", number = 2, width = 6);
	println!("{number:>0width$}", number = 2, width = 6);

	#[derive(Debug)]
	#[allow(dead_code)]
	struct Structure(i32);

	println!("this struct `{:#?}` ", Structure(3));

}



