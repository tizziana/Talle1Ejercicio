use std::cmp;

///Recibe dos vectores de strings, y se llena una matriz, que es de m(long vec 1) x n (long vec 2),
///y luego se va comparando las linead de cada uno, y se llena con numeros que representan si esas lineas son o no =.
pub fn diff_btw_files(x: &[String], y: &[String]) -> Vec<Vec<i32>> {
    let m = x.len();
    let n = y.len();

    let mut c = vec![vec![0; n + 1]; m + 1];

    for i in 0..m {
        for (j, linea) in y.iter().enumerate().take(n) {
            if x[i] == *linea {
                c[i + 1][j + 1] = c[i][j] + 1;
            } else {
                c[i + 1][j + 1] = cmp::max(c[i + 1][j], c[i][j + 1])
            }
        }
    }
    c
}
