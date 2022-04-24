use std::io;

pub fn run_ahorcado(palabra: String) {

    let mut fin_juego = false;

    let mut intento = String::new();

    while !fin_juego {
        io::stdin()
        .read_line(&mut intento)
        .expect("Error leyendo la linea.");

        validar_intento(intento);

        fin_juego = true;

    }

    
}