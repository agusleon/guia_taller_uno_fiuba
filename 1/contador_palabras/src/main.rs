use std::collections::HashMap;

use contador_palabras::read_line::read_file_lines;


fn main() {

    let mut hashmap = HashMap::new();

    match read_file_lines("archivo_uno.txt", &mut hashmap) {
        Ok(()) => (),
        Err(error) => println!("Error leyendo archivo: {}", error),
    }
    
    let sorted_vector = ordenar_por_frecuencia(&mut hashmap);

    imprimir_palabras_ordenadas(sorted_vector);
}

pub fn ordenar_por_frecuencia(hashmap: &mut HashMap<String,u8>) -> Vec<(&String, &u8)>{

    let mut sorted_vector: Vec<(&String, &u8)> = hashmap.iter().collect();
    sorted_vector.sort_by(|a, b| b.1.cmp(a.1));
    return sorted_vector;

}

pub fn imprimir_palabras_ordenadas(vector: Vec<(&String, &u8)>){
    for (key, value) in vector {
        println!("{} -> {}",key, value);
    }
}