#![feature(collections)]

extern crate bit_vec;

use bit_vec::BitVec;


fn opt_dist(bytes: &BitVec, num: usize) -> usize {
	assert!(num <= bytes.len());

	let mut mask = BitVec::new();

	let mut min = std::usize::MAX;

	for start in 0..(bytes.len() - num) {
		mask.truncate(0);
		mask.grow(start, false);
		mask.grow(num, true);
		mask.grow(bytes.len() - start - num, false);

		println!("a {:?}", bytes);
		println!("b {:?}", mask);

		let diff = mask.iter().zip(bytes.iter()).filter(|&(a, b)| a != b).count();

		
		println!("c {:?}", diff);
		if diff < min {
			min = diff;
		}
	}

	min	
}


fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod test{
	#[test]
	fn basic_test() {
		use opt_dist;
		use bit_vec::BitVec;
		assert_eq!(opt_dist(&BitVec::from_bytes(&[0b0010001000]), 5), 3);
		assert_eq!(opt_dist(&BitVec::from_bytes(&[0b0010001000]), 4), 4);
		assert_eq!(opt_dist(&BitVec::from_bytes(&[0b0010001000]), 3), 3);
		assert_eq!(opt_dist(&BitVec::from_bytes(&[0b0010001000]), 2), 2);
		assert_eq!(opt_dist(&BitVec::from_bytes(&[0b0010001000]), 1), 1);
		assert_eq!(opt_dist(&BitVec::from_bytes(&[0b0010001000]), 0), 2);
	}
}