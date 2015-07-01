use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Create a path to the desired file
    let path = Path::new("names.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => (),
    }


    let cleaned = s.replace("\"", "");

    let mut names: Vec<&str> = cleaned.split(",").collect();

    names.sort();

    let mut totscore = 0;

    let nameslen = names.len();

    for i in 0..nameslen {
        println!("{:?}", names[i]);
        totscore += namescore(names[i]) * (i as u32 + 1);
    }

    println!("{:?}", totscore);

    //println!("{:?}", names[984]);

    fn namescore(name: &str) -> u32 {
        let mut score = 0;
        for c in name.chars() {
            score += c as u32 - 64
        }
        score
    }

}