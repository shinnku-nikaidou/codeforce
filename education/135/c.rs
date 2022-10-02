struct S;
impl S {
    fn f(mut a: i64) -> usize {
        let mut ans = 0;
        while a != 0 {
            a = a / 10;
            ans += 1;
        }
        ans
    }
}

fn main() {
    let _guard = ();
    // use stdio_override::StdinOverride;
    // let file_name = "./input.txt";
    // let _guard = StdinOverride::override_file(file_name);
    let stdin = std::io::stdin();
    let mut lines = stdin.lines();
    let t = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    for _ in 0..t {
        let mut ans = 0;
        let n = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
        let a = nextline2vec::<i64>(&mut lines);
        assert_eq!(n, a.len());
        let b = nextline2vec::<i64>(&mut lines);
        assert_eq!(n, b.len());
        let mut a_large = vec![];
        let mut b_large = vec![];
        let mut a_small = [0; 10];
        let mut b_small = [0; 10];
        for i in a {
            if i >= 10 {
                a_large.push(i)
            } else {
                a_small[i as usize] += 1;
            }
        }
        for i in b {
            if i >= 10 {
                b_large.push(i)
            } else {
                b_small[i as usize] += 1;
            }
        }
        a_large.sort();
        b_large.sort();
        // println!("{:?}, {:?}", a_large, a_small);
        // println!("{:?}, {:?}", b_large, b_small);
        let la = a_large.len();
        let lb = b_large.len();
        let mut i = 0;
        let mut j = 0;
        loop {
            if i == la && j == lb {
                break;
            } else if i == la {
                b_small[S::f(b_large[j])] += 1;
                j += 1;
                ans += 1;
            } else if j == lb {
                a_small[S::f(a_large[i])] += 1;
                i += 1;
                ans += 1;
            } else {
                if a_large[i] == b_large[j] {
                    i += 1;
                    j += 1;
                } else if a_large[i] > b_large[j] {
                    b_small[S::f(b_large[j])] += 1;
                    j += 1;
                    ans += 1;
                } else if a_large[i] < b_large[j] {
                    a_small[S::f(a_large[i])] += 1;
                    i += 1;
                    ans += 1;
                }
            }
        }
        for i in 2..10 as usize{
            ans += (a_small[i] - b_small[i] as i32).abs();
        }
        println!("{}", ans);
        // println!("{:?}, {:?}", a_small, b_small);
    }
    drop(_guard);
}

fn nextline2vec<T>(lines: &mut std::io::Lines<std::io::StdinLock>) -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    lines
        .next()
        .unwrap()
        .unwrap()
        .split(" ")
        .map(|s: &str| s.parse::<T>().unwrap())
        .collect::<Vec<T>>()
}
