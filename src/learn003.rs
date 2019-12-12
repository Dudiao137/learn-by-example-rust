// learn003.rs
// 2.1 字面量与运算符 
// @user: sy.wang
// @date: 2019-12-12
// 

fn main() {
	
	println!("1+2={}", 1u32+2);
	println!("4-2={}", 4u32-2);
	println!("1-2={}", 1i32-2);

	println!("true and flase is {}", true&&false);
	println!("true or flase is {}", true||false);
	println!("not true is {}", !true);


	// bit 
	println!("0011 and 0101 is {:04b}", 0b0011u32 & 0b0101);
	println!("0011 and 0101 is {}", 0b0011u32 & 0b0101);
	println!("0011 or 0101 is {:04b}", 0b0011u32 | 0b0101);
	println!("0011 or 0101 is {}", 0b0011u32 | 0b0101);
	println!("0011 xor 0101 is {:04b}", 0b0011u32 ^ 0b0101);
	println!("0011 xor 0101 is {}", 0b0011u32 ^ 0b0101);

	println!("1<<5 is {}", 1u32<<5);
	println!("1<<5 is {:08b}", 1u32<<5);
	println!("0x80>>2 is 0x {:x}", 0x80u32>>2);

	// readable
	println!("one million is {}", 1_000_000u32);

}


