///Recibe una matriz llena de numeros, dos vectores de strings y las longitudes de los mismos.
///Se va fijando si estos vectores, linea a linea, son = o no, imprimiento la linea de ambos (en el caso de ser =), o la linea diferente entre ellos con un simbolo de < o > según esa linea pertenezca al primer archivo o al segundo.

pub fn print_diff(c: &mut Vec<Vec<i32>>, x: &Vec<String>, y: &Vec<String>, i: usize, j: usize) {
    if i > 0 && j > 0 && x[i - 1] == y[j - 1] {
        print_diff(c, x, y, i - 1, j - 1);
        println!("  {}", x[i - 1]);
    } else if j > 0 && (i == 0 || c[i][j - 1] >= c[i - 1][j]) {
        print_diff(c, x, y, i, j - 1);
        println!("> {}", y[j - 1]);
    } else if i > 0 && (j == 0 || c[i][j - 1] < c[i - 1][j]) {
        print_diff(c, x, y, i - 1, j);
        println!("< {}", x[i - 1]);
    } else {
        println!(" ");
    }
}
