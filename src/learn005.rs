// learn005.rs
// 2.3 数组与切片
// @user: sy.wang
// @date: 2019-12-12
// 

use std::mem;

fn analyze_slice(slice : &[i32]) {
	println!("first element of the slice: {}", slice[0]);
	println!("the length of slice: {}", slice.len());
}

fn main() {
	
	let xs :[i32; 5] = [1,2,3,4,5];

	let ys :[i32; 500]=[0;500];

	println!("first element of the slice: {}", xs[0]);
	println!("second element of the slice: {}", xs[1]);

	// 数组在栈中分配
	println!("array occupies {} bytes", mem::size_of_val(&xs));

	println!("数组可以自动被借用成为slice");
	analyze_slice(&xs);

	println!("silce可以指向数组的一部分");
	analyze_slice(&ys[1..4]);

	// 越界
	// println!("{}", xs[5]);

}
