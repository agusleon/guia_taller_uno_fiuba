use std::fs::File;
use rand::prelude::*;
use std::io::{self, prelude::*, BufReader};

const FILE_LEN : usize = 466550;

pub fn read_file_lines(path: &str) -> Result<String,io::Error> {

    let file = File::open(path);

    match file {
        Ok(file) =>
        {
            let reader = BufReader::new(file);

            let mut lines = reader.lines();
            let mut rng = thread_rng();
            let rand_line = rng.gen_range(0..FILE_LEN);

            lines.nth(rand_line).expect("Could not read word.") 
           

        },
        Err(error) => Err(error),
    }

    
}
