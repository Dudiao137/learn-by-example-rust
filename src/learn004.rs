// learn004.rs
// 2.2 元组 
// @user: sy.wang
// @date: 2019-12-12
// 

fn reverse(pair: (i32, bool)) -> (bool, i32) {
	let (integer, boolean) = pair;

	(boolean, integer)
}

fn main() {
	
	let long_tuple = (1u8, 2u16, 3u32, 4u64,
					-1i8, -2i16, -3i32, -4i64,
					0.1f32, 0.2f64,
					'a', true);

	println!("long tuple first value: {}", long_tuple.0);
	println!("long tuple second value: {}", long_tuple.1);

	let tuple_of_tuples = ((1u8, 2u16, 3u32),(4u64, -1i8), -2i16);
	println!("tuple of tuple : {:?}", tuple_of_tuples);

	let too_long_tuple = (1,2,3,4,5,6,7,8,9,0/*,1,2,3,4,5*/);
	let too_long_tuple = (1,2,3,4,5,6,7,8,9,0,1,2,3,4,5);
	println!("tuple of too long : {:?}", too_long_tuple);
	 
	let pair = (1, true);
	println!("pair is {:?}", pair);

	println!("reversed is : {:?}", reverse(pair));

	println!("one element : {:?}", (5u32,));
	println!("one element, int  {:?}", (5u32));

	let tuple = (1, "hello", true);
	let (a, b, c) = tuple;

	println!("{},{},{}", a, b, c);

}

