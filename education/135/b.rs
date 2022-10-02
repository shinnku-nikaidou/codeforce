fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lines();
    let t = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    for _ in 0..t {
        let n = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
        if n % 2 == 0 {
            for i in (1..n - 1).rev() {
                print!("{} ", i);
            }
            println!("{} {}", n - 1, n);
        } else {
            for i in (4..n - 1).rev() {
                print!("{} ", i);
            }
            println!("1 2 3 {} {}", n - 1, n);
        }
        
    }
}
