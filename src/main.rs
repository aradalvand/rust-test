fn main() {
    let mut n = 0;

    loop {
        n += 1;
        if n % 2 != 0 {
            continue;
        }
        if n > 20 {
            break;
        }
        println!("{n}");
    }
}
