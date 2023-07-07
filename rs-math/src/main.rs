use anyhow::Result;
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

/*
 * @notice: Add the running time corresponding to the gas formula to
 * constant matrix.
 * @param idx: Keeps track of which row to edit
 * @param running_time: The running time w.r.t the gas formula
 * @param constant_matrix: The Matrix we want to edit
 */
fn add_running_time_to_constant_matrix(
    idx: &mut usize,
    running_time: f64,
    constant_matrix: &mut DMatrix<f64>,
) {
    constant_matrix[(*idx, 0)] = running_time;
    *idx = *idx + 1;
}

/*
 * @notice: Join coefficient and constant matrix to make augmented
 * @param augmented_matrix: Matrix to join coefficient and constant
 * @param coefficient_matrix: Matrix for gas formula
 * @param constant_matrix: Matrix for running time w.r.t gas formula
 */
fn create_augmented_matrix(
    augmented_matrix: &mut DMatrix<f64>,
    coefficient_matrix: &mut DMatrix<f64>,
    constant_matrix: &mut DMatrix<f64>,
) {
    let mut i = 0;
    let mut j = 0;
    while i < NROWS {
        while j < NCOLS {
            augmented_matrix[(i, j)] = coefficient_matrix[(i, j)];
            j = j + 1;
        }
        i = i + 1;
        j = 0;
    }

    i = 0;
    while i < NROWS {
        augmented_matrix[(i, NCOLS)] = constant_matrix[(i, 0)];
        i = i + 1;
    }
}

/*
 * @notice: Compute least squares
 * @param A: Coefficient matrix
 * @param b: Constant matrix
 * @return x_hat: A matrix of the solution
 */
#[allow(non_snake_case)]
fn compute_least_square_solutions(
    A: &mut DMatrix<f64>,
    b: &mut DMatrix<f64>,
) -> Result<DMatrix<f64>, String> {
    let A_T = A.transpose();
    let A_TA = A_T.ad_mul(A);

    if !A_TA.is_invertible() {
        return Err("cannot invert A matrix".to_string());
    }

    let inverse = A_TA.try_inverse().expect("inverse should work");
    let A_Tb = A_T.ad_mul(b);
    let x_hat = inverse.ad_mul(&A_Tb);
    Ok(x_hat)
}

fn main() {
    //// Construct coefficient matrix based on input size at runtime
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
    println!("coefficient matrix after: {}", coeff_matrix);

    //// Construct constant matrix based on nrows at runtime
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

    //// Construct augmented matrix at runtime
    let mut augmented_matrix = DMatrix::<f64>::zeros(NROWS, NCOLS + 1);
    create_augmented_matrix(&mut augmented_matrix, &mut coeff_matrix, &mut const_matrix);
    println!("augmented matrix: {}", augmented_matrix);

    //// least square solutions example
    let lss = compute_least_square_solutions(&mut coeff_matrix, &mut const_matrix);
    if lss.is_ok() {
        println!("least sq: {}", lss.unwrap());
    }
}
