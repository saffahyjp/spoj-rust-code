fn read_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn read_int_line() -> Vec<i32> {
    read_line()
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect()
}

fn is_prime(x : i32) -> bool {
    if x < 2 {
        return false;
    }
    let mut i = 2;
    loop {
        if i * i > x {
            break;
        }
        if x % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn main() {
    let t = read_int_line()[0];
    for _ in 0 .. t {
        let line = read_int_line();
        let (l, r) = (line[0], line[1]);
        for i in l ..= r {
            if is_prime(i) {
                println!("{}", i);
            }
        }
        println!();
    }
}
