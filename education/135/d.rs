struct S;
impl S {
    fn anti_lexicographically(a: &String, b: &String) -> i32 {
        let a_rev: String = a.chars().rev().collect();
        let b_rev: String = b.chars().rev().collect();
        if a_rev > b_rev {
            1
        } else if a_rev == b_rev {
            0
        } else {
            -1
        }
    }

    fn alice_must_win(alice: &String, bob: &String, s: &[u8]) -> bool {
        let mut alice1 = alice.clone();
        let mut alice2 = alice.clone();
        alice1.push(*s.first().unwrap() as char);
        alice2.push(*s.last().unwrap() as char);
        Self::bob_must_lose(&alice1, bob, &s[1..])
            || Self::bob_must_lose(&alice2, bob, &s[..s.len() - 1])
    }

    fn bob_must_win(alice: &String, bob: &String, s: &[u8]) -> bool {
        if s.len() == 1 {
            let mut bob = bob.clone();
            bob.push(*s.first().unwrap() as char);
            let t = Self::anti_lexicographically(alice, &bob);
            if t == 1 {
                return true;
            } else {
                return false;
            }
        }
        let mut bob1 = bob.clone();
        let mut bob2 = bob.clone();
        bob1.push(*s.first().unwrap() as char);
        bob2.push(*s.last().unwrap() as char);
        Self::alice_must_lose(&alice, &bob1, &s[1..])
            || Self::alice_must_lose(&alice, &bob2, &s[..s.len() - 1])
    }

    fn alice_must_lose(alice: &String, bob: &String, s: &[u8]) -> bool {
        let mut alice1 = alice.clone();
        let mut alice2 = alice.clone();
        alice1.push(*s.first().unwrap() as char);
        alice2.push(*s.last().unwrap() as char);
        Self::bob_must_win(&alice1, bob, &s[1..])
            && Self::bob_must_win(&alice2, bob, &s[..s.len() - 1])
    }

    fn bob_must_lose(alice: &String, bob: &String, s: &[u8]) -> bool {
        if s.len() == 1 {
            let mut bob = bob.clone();
            bob.push(*s.first().unwrap() as char);
            let t = Self::anti_lexicographically(alice, &bob);
            if t == -1 {
                return true;
            } else {
                return false;
            }
        }
        let mut bob1 = bob.clone();
        let mut bob2 = bob.clone();
        bob1.push(*s.first().unwrap() as char);
        bob2.push(*s.last().unwrap() as char);
        Self::alice_must_win(&alice, &bob1, &s[1..])
            && Self::alice_must_win(&alice, &bob2, &s[..s.len() - 1])
    }

    fn can_draw(s: &mut String, lru: &mut std::collections::BTreeMap<String, bool>) -> bool {
        if lru.contains_key(s) {
            return lru[s];
        }
        if s.is_empty() {
            return true;
        } else if s.len() == 2 {
            let s_raw = unsafe { &s.as_mut_vec()[..] };
            if *s_raw.first().unwrap() == *s_raw.last().unwrap() {
                lru.insert(s.clone(), true);
                return true;
            } else {
                lru.insert(s.clone(), false);
                return false;
            }
        }
        let ans;

        let s_raw = unsafe { &s.as_mut_vec()[..] };
        if *s_raw.first().unwrap() == *s_raw.last().unwrap() {
            ans = Self::can_draw(
                &mut String::from_utf8_lossy(&s_raw[1..s_raw.len() - 1]).into_owned(),
                lru,
            );
            lru.insert(s.clone(), ans);
        } else if s_raw[0] == s_raw[1] && s_raw[s_raw.len() - 1] == s_raw[s_raw.len() - 2] {
            let ansl = Self::can_draw(&mut String::from_utf8_lossy(&s_raw[2..]).into_owned(), lru);
            let ansr = Self::can_draw(
                &mut String::from_utf8_lossy(&s_raw[..s_raw.len() - 2]).into_owned(),
                lru,
            );
            ans = ansl && ansr;
            lru.insert(s.clone(), ans);
        } else {
            ans = false;
            lru.insert(s.clone(), ans);
        }
        ans
    }
}

#[derive(Debug, Clone)]
enum Win {
    Alice,
    Bob,
    Draw,
}

fn main() {
    let _guard = ();
    // use stdio_override::StdinOverride;
    // let file_name = "./input.txt";
    // let _guard = StdinOverride::override_file(file_name);
    let stdin = std::io::stdin();
    let mut lines = stdin.lines();
    let t = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    let mut cache = std::collections::BTreeMap::new();
    for _ in 0..t {
        let mut s = lines.next().unwrap().unwrap();
        // println!("{}", s);
        // if true {
        if s.len() > 24  {
            if S::can_draw(&mut s, &mut cache) {
                println!("Draw");
            } else {
                println!("Alice");
            }
            continue;
        }
        let is_alice_win = S::alice_must_win(&"".to_owned(), &"".to_owned(), unsafe {
            &s.as_mut_vec()[..]
        });
        let is_alice_lose = S::alice_must_lose(&"".to_owned(), &"".to_owned(), unsafe {
            &s.as_mut_vec()[..]
        });
        match (is_alice_win, is_alice_lose) {
            (true, true) => panic!("alice can't win or lose at the same time"),
            (true, false) => println!("{:?}", Win::Alice),
            (false, true) => println!("{:?}", Win::Bob),
            (false, false) => println!("{:?}", Win::Draw),
        }
    }
    drop(_guard);
}

fn _nextline2vec<T>(lines: &mut std::io::Lines<std::io::StdinLock>) -> Vec<T>
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
