fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lines();
    let t = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    for _ in 0..t {
        let n = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
        // let cnt = Vec::with_capacity(n);
        let line = lines.next().unwrap().unwrap();
        let mut cnt = line
            .split(" ")
            .map(|s: &str| s.parse().unwrap())
            .collect::<Vec<i32>>();
        assert_eq!(n, cnt.len());
        for i in 0..n {
            for j in i + 1..n {
                if cnt[i] == 0 {
                    break;
                }
                if cnt[j] == 0 {
                    continue;
                }
                let l = std::cmp::min(cnt[i], cnt[j]);
                cnt[i] -= l;
                cnt[j] -= l;
            }
        }
        for i in 0..n {
            if cnt[i] != 0 {
                println!("{}", i+1);
            }
        }
    }
}
