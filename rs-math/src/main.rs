use nalgebra::{self, DMatrix};

//// TODO: at runtime, find how many linear equations we have
const NROWS: usize = 3;
//// TODO: at runtime, find how many distinct gas parameters we have
const NCOLS: usize = 3;

fn add_gas_formula_to_coefficient_matrix(
    idx: &mut usize,
    formula: &[f64],
    coeff_matrix: &mut DMatrix<f64>,
) {
    let mut j = 0;
    while j < NCOLS {
        coeff_matrix[(*idx, j)] = formula[j];
        j = j + 1;
    }

    *idx = *idx + 1;
}

fn main() {
    //// Construct coefficient matrix based on input size
    let mut coeff_row_idx = 0;
    let mut coeff_matrix = DMatrix::<f64>::zeros(NROWS, NCOLS);

    //// coefficient matrix example
    println!("og: {}", coeff_matrix);
    let eq1 = vec![2.0, 1.0, -2.0];
    let eq2 = vec![1.0, -1.0, -1.0];
    let eq3 = vec![1.0, 1.0, 3.0];
    add_gas_formula_to_coefficient_matrix(&mut coeff_row_idx, &eq1, &mut coeff_matrix);
    add_gas_formula_to_coefficient_matrix(&mut coeff_row_idx, &eq2, &mut coeff_matrix);
    add_gas_formula_to_coefficient_matrix(&mut coeff_row_idx, &eq3, &mut coeff_matrix);
    println!("after: {} {}", coeff_matrix, coeff_row_idx);

    //// Construct constant matrix based on nrows
    //let const_matrix = DMatrix::<f64>::zeros(nrows, ncols);
}
