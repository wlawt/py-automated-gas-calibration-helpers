use nalgebra::{self, DMatrix};

//// TODO: at runtime, find how many linear equations we have
const NROWS: usize = 3;
//// TODO: at runtime, find how many distinct gas parameters we have
const NCOLS: usize = 3;
const VEC_COL: usize = 1;

/*
 * @notice: Add a gas formula to the coefficient matrix
 * @param idx: Keeps track of which row to edit
 * @param formula: The gas formula to add
 * @param coefficient_matrix: The Matrix we want to edit
 */
fn add_gas_formula_to_coefficient_matrix(
    idx: &mut usize,
    formula: &[f64],
    coefficient_matrix: &mut DMatrix<f64>,
) {
    let mut j = 0;
    while j < NCOLS {
        coefficient_matrix[(*idx, j)] = formula[j];
        j = j + 1;
    }

    *idx = *idx + 1;
}

fn add_running_time_to_constant_matrix(
    idx: &mut usize,
    running_time: f64,
    constant_matrix: &mut DMatrix<f64>,
) {
    constant_matrix[(*idx, 0)] = running_time;
    *idx = *idx + 1;
}

fn main() {
    //// Construct coefficient matrix based on input size
    let mut coeff_row_idx = 0;
    let mut coeff_matrix = DMatrix::<f64>::zeros(NROWS, NCOLS);

    //// coefficient matrix example
    println!("coefficient matrix before: {}", coeff_matrix);
    let eq1 = vec![2.0, 1.0, -2.0];
    let eq2 = vec![1.0, -1.0, -1.0];
    let eq3 = vec![1.0, 1.0, 3.0];
    add_gas_formula_to_coefficient_matrix(&mut coeff_row_idx, &eq1, &mut coeff_matrix);
    add_gas_formula_to_coefficient_matrix(&mut coeff_row_idx, &eq2, &mut coeff_matrix);
    add_gas_formula_to_coefficient_matrix(&mut coeff_row_idx, &eq3, &mut coeff_matrix);
    println!(
        "coefficient matrix after: {} {}",
        coeff_matrix, coeff_row_idx
    );

    //// Construct constant matrix based on nrows
    let mut const_row_idx = 0;
    let mut const_matrix = DMatrix::<f64>::zeros(NROWS, VEC_COL);

    //// constant matrix example
    println!("constant matrix before: {}", const_matrix);
    let r1 = 3.0;
    let r2 = 0.0;
    let r3 = 12.0;
    add_running_time_to_constant_matrix(&mut const_row_idx, r1, &mut const_matrix);
    add_running_time_to_constant_matrix(&mut const_row_idx, r2, &mut const_matrix);
    add_running_time_to_constant_matrix(&mut const_row_idx, r3, &mut const_matrix);
    println!("constant matrix after: {}", const_matrix);
}
