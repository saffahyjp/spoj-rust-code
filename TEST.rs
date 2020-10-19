use std::io::stdin;

fn main() {
    loop {
        let mut s = String::new();
        stdin().read_line(&mut s).expect("GG");
        if s.trim() == "42" {
            break;
        }
        print!("{}", s);
    }
}
