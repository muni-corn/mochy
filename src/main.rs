use std::{io::{stdin, stdout, Write, Read}, cmp::Ordering, fs::File};

fn main() {
    let file_name = std::env::args().nth(1).expect("need a file name to sort");

    let mut v: Vec<String> = {
        let mut file = File::open(file_name).expect("couldn't open file to sort :(");
        let mut s = String::new();
        file.read_to_string(&mut s).expect("couldn't read content of file");
        s.lines().map(String::from).collect()
    };

    v.sort_unstable_by(|a, b| {
        println!("\nis '{a}' more important than '{b}'?");
        print!("[Y/n] ");
        loop {
            let mut response = String::new();
            stdout().flush().expect("couldn't flush stdout :(");
            stdin().read_line(&mut response).expect("couldn't read from stdin :(");
            response = response.to_lowercase();
            if response.starts_with('y') || response.trim().is_empty() {
                return Ordering::Less
            } else if response.starts_with('n') {
                return Ordering::Greater
            }
            print!("hm? [Y/n] ");
        };
    });

    for l in v {
        println!("{l}");
    }
}
