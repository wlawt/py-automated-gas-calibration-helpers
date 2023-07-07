use anyhow::Result;
use nalgebra::{self, DMatrix};
use std::env;
use std::ops::Mul;

//// TODO: at runtime, find how many linear equations we have
const NROWS: usize = 3;
//// TODO: at runtime, find how many distinct gas parameters we have
const NCOLS: usize = 2;
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

    let nrows = augmented_matrix.nrows();
    let ncols = coefficient_matrix.ncols();
    while i < nrows {
        while j < ncols {
            augmented_matrix[(i, j)] = coefficient_matrix[(i, j)];
            j = j + 1;
        }
        i = i + 1;
        j = 0;
    }

    i = 0;
    while i < nrows {
        augmented_matrix[(i, ncols)] = constant_matrix[(i, 0)];
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
    let A_TA = A_T.clone().mul(A.clone());
    let A_Tb = A_T.clone().mul(b.clone());

    if !A_TA.is_invertible() {
        return Err("cannot invert A_TA matrix".to_string());
    }

    let inverse = A_TA.try_inverse().expect("inverse should work");
    let x_hat = inverse.mul(&A_Tb);
    Ok(x_hat)
}

/*
 * @notice: Find all free variables which is the pivot columns
 * @param A: Coefficient matrix
 * @param b: Constant matrix
 * @return pivot_columns: A vector containing the pivot column indices
 */
#[allow(non_snake_case)]
fn find_free_variables(A: &mut DMatrix<f64>, b: &mut DMatrix<f64>) -> Vec<usize> {
    let A_T = A.transpose();
    let mut A_TA = A_T.clone().mul(A.clone());
    let mut A_Tb = A_T.clone().mul(b.clone());

    let nrows_a_ta = A_TA.nrows();
    let ncols_a_ta = A_TA.ncols();
    let mut aug_matrix = DMatrix::<f64>::zeros(nrows_a_ta, ncols_a_ta + 1);
    create_augmented_matrix(&mut aug_matrix, &mut A_TA, &mut A_Tb);
    rref(&mut aug_matrix);
    println!("RREF'd matrix: {}", aug_matrix);

    let pivot_columns = find_pivot_columns(&mut aug_matrix);
    pivot_columns
}

/*
 * @notice: Find pivot columns if system of linear eq can't be solved
 * @param matrix: An input matrix to solve, typically the RREF'd matrix
 * @return pivot_columns: A vector containing the pivot column indices
 */
fn find_pivot_columns(matrix: &mut DMatrix<f64>) -> Vec<usize> {
    let mut pivot_columns = Vec::new();
    let ncols = matrix.ncols() - 1;

    for j in 0..ncols {
        let mut has_pivot = false;
        for i in 0..matrix.nrows() {
            if matrix[(i, j)] != 0.0 {
                has_pivot = true;
                break;
            }
        }
        if has_pivot {
            pivot_columns.push(j);
        }
    }

    pivot_columns
}

/*
 * @notice: Reduced row echelon form (RREF)
 * @param matrix: A matrix to perform RREF
 */
fn rref(matrix: &mut DMatrix<f64>) {
    let (nrows, ncols) = matrix.shape();
    let mut lead = 0;

    for r in 0..nrows {
        if ncols <= lead {
            break;
        }

        let mut i = r;

        while matrix[(i, lead)] == 0.0 {
            i += 1;

            if nrows == i {
                i = r;
                lead += 1;

                if ncols == lead {
                    return;
                }
            }
        }

        if i != r {
            matrix.swap_rows(i, r);
        }

        let pivot = matrix[(r, lead)];

        for j in 0..ncols {
            matrix[(r, j)] /= pivot;
        }

        for i in 0..nrows {
            if i != r {
                let factor = matrix[(i, lead)];
                for j in 0..ncols {
                    matrix[(i, j)] -= factor * matrix[(r, j)];
                }
            }
        }

        lead += 1;
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    //// Construct coefficient matrix based on input size at runtime
    let mut coeff_row_idx = 0;
    let mut coeff_matrix = DMatrix::<f64>::zeros(NROWS, NCOLS);

    //// coefficient matrix example
    println!("coefficient matrix before: {}", coeff_matrix);

    // working example
    let eq1 = vec![1.0, 1.0];
    let eq2 = vec![2.0, 2.0];
    let eq3 = vec![2.0, 3.0];
    let r1 = 10.0;
    let r2 = 20.0;
    let r3 = 25.0;

    // prints free variables
    /*let eq1 = vec![1.0, 1.0];
    let eq2 = vec![2.0, 2.0];
    let eq3 = vec![3.0, 3.0];
    let r1 = 10.0;
    let r2 = 30.0;
    let r3 = 30.0;*/

    add_gas_formula_to_coefficient_matrix(&mut coeff_row_idx, &eq1, &mut coeff_matrix);
    add_gas_formula_to_coefficient_matrix(&mut coeff_row_idx, &eq2, &mut coeff_matrix);
    add_gas_formula_to_coefficient_matrix(&mut coeff_row_idx, &eq3, &mut coeff_matrix);
    println!("coefficient matrix after: {}", coeff_matrix);

    //// Construct constant matrix based on nrows at runtime
    let mut const_row_idx = 0;
    let mut const_matrix = DMatrix::<f64>::zeros(NROWS, VEC_COL);

    //// constant matrix example
    println!("constant matrix before: {}", const_matrix);

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
        println!("least square solution: {}", lss.unwrap());
    } else {
        let free_variables = find_free_variables(&mut coeff_matrix, &mut const_matrix);
        println!("free variables are: {:?}", free_variables);
    }
}
