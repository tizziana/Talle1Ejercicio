use std::fs;
use std::fs::File;

///Recibe un string de un path de un archivo, lo abrimos, y luevo vamos a leer ese file, para poder poner las lineas que tiene en un vector de strings.
///En el caso de no poder abrir el archivo, o no poder leerlo nos va a aparecer un error.
pub fn read_file_lines(ruta: &String) -> Vec<String> {
    let mut lineas: Vec<String> = Vec::new();

    let _archivo = match File::open(ruta) {
        Err(error_abrir_ar) => panic!("No se pudo abrir el archivo {}", error_abrir_ar),
        Ok(_archivo) => lineas,
    };

    let lines_read = fs::read_to_string(ruta);
    match lines_read {
        Err(error_leer_arc) => panic!("No se pudo leer: {}", error_leer_arc),
        Ok(leido) => lineas = leido.split('\n').map(String::from).collect(),
    };
    lineas
}
