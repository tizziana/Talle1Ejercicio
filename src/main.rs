// Guia 2: Ejercicio 2 -> diff

mod lcs_f;
mod print_diff;
mod read_file;
use lcs_f::lcs as diff;
use print_diff::print_diffs as print_d;
use read_file::read_files as rf;
use std::env;

///Funcion principal -> main
///Primero guarda las lineas de los archivos llamando a read_file_lines devolviendonos un vector con las mismas.
///Sengundo se llama a diff_btw_files, pasandole los vectores que nos fueron devueltos anteriormente. Aqui se genera una matriz en la que se indican si las lineas son iguales o no.
///Tercer se llama a print_diff, donde se imprimen las lineas entre ambos archivos de la siguiente forma:
/// -si son iguales las lineas se imprime la linea normalmente.
/// -si la linea del primer archivo es diferente se imprime con un sigo '<'
/// -si la linea del segudo archivo es diferente se imprime con un sigo '>'

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename1 = &args[1];
    let filename2 = &args[2];

    let _archivo1 = rf::read_file_lines(filename1);
    let _archivo2 = rf::read_file_lines(filename2);

    let mut matriz_diff = diff::diff_btw_files(&_archivo1, &_archivo2);
    print_d::print_diff(
        &mut matriz_diff,
        &_archivo1,
        &_archivo2,
        _archivo1.len(),
        _archivo2.len(),
    );
}
