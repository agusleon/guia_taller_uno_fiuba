use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn read_file_lines(path: &str, hashmap: &mut HashMap<String,u8>) -> Result<(),io::Error> {

    let file = File::open(path);

    match file {
        Ok(file) =>
        {
            let reader = BufReader::new(file);

            let lines = reader.lines();

            for l in lines {
                
                let linea = l?; 
                agregar_a_hashmap(&linea, hashmap);
            }
            
            Ok(())

        },
        Err(error) => Err(error),
    }

    
}

pub fn agregar_a_hashmap(linea: &String, hashmap: &mut HashMap<String,u8>){

    let linea_lower = linea.to_lowercase();
    
    let split = linea_lower.split_whitespace();
    
    for word in split {
        
        let mut key = String::from(word);

        key.retain(|c| !r#"(),".;:'?¿!¡"#.contains(c));

        if hashmap.contains_key(&key) {
            let previous_value = hashmap[&key];
            hashmap.insert(key, 1 + previous_value);
        }
        else {
            hashmap.insert(key, 1);
        }

    }
}