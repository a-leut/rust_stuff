fn main() {
    let mut i = 0;
	let mut j = 1;
	let mut acc = 0;
	loop {
		if j > 4000000 {
			break;
		}
		if j % 2 == 0 {
			acc += j;
		}
		let k = i;
		i = j;
		j = k + j;
	}
	println!("{}", acc);
}
