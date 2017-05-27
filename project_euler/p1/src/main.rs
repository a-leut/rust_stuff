fn main() {
    let mut n = 0;
	for i in 1..1001 {
		match i {
			i if i % 3 == 0 => n += i,
			i if i % 5 == 0 => n += i,
			_ => ()
		}
	}
	println!("{}", n);
}
