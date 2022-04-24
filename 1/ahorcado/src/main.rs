use ahorcado::read_file_lines;
use ahorcado::ahorcado::run_ahorcado;



fn main() {

    let dic = read_file_lines("diccionario.txt");

    match dic {
        Ok(dic) => {
            run_ahorcado(dic);
        },
        Err(error) => println!("Problem opening file: {}", error),
    }

}
