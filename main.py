from sympy import *

"""
CONSTANTS
"""
ROW = 0
COL = 1
MARGIN_OF_ERROR = 0.05

coeff_matrix = Matrix([])
const_matrix = Matrix([])

"""
@param formula: vec<u64>
@param coefficient_matrix: Matrix 2d
@returns: Matrix 2d 
"""
def add_gas_formula_to_coefficient_matrix(formula, coefficient_matrix):
    return coefficient_matrix.row_insert(0, Matrix([formula]))


"""
@param running_time: vec<u64>
@param constant_matrix: Matrix 2d
@returns: Matrix 2d
"""
def add_running_time_to_constant_matrix(running_time, constant_matrix):
    return constant_matrix.row_insert(0, Matrix(running_time))


"""
@param coefficient_matrix: Matrix 2d
@param constant_matrix: Matrix 2d
@returns: Matrix 2d
"""
def create_augmented_matrix(coefficient_matrix, constant_matrix):
    return coefficient_matrix.row_join(constant_matrix)


"""
@notice: find a particular least square solution
@param A: coefficient matrix
@param b: constant matrix
@returns: Matrix
"""
def compute_least_square_solution(A, b):
    # find A transposed
    A_T = A.transpose()

    # find matrix A^T * A and vector A^T * b
    A_TA = A_T * A
    A_Tb = A_T * b
    augmented_matrix = create_augmented_matrix(A_TA, A_Tb)
    # return value is (rref matrix, tuple of pivot columns)
    return augmented_matrix.rref()[0]


"""
@notice: find all free variables
@param M: rref'd matrix
@returns: list of all free variables
"""
def find_free_variables(A,b):
    A_T = A.transpose()
    A_TA = A_T * A
    A_Tb = A_T * b
    augmented_matrix = create_augmented_matrix(A_TA, A_Tb)
    # return value is (rref matrix, tuple of pivot columns)
    return augmented_matrix.rref()[1]    


"""
@notice: this can handle when there are infinitely many least square solutions
@param A: coefficient matrix
@param b: constant matrix
@returns: Matrix or tuple
"""
def compute_least_square_solutions(A, b):
    A_T = A.transpose()
    A_TA = A_T * A
    
    if not is_solvable(A):
        print("[WARNING] system of equations is inconsistent\nwhere:")
        list_of_free_variables = find_free_variables(A,b)
        for free_variable in list_of_free_variables:
            print("- position {} gas parameter cannot be determined".format(free_variable))
        return "notice: try adding more samples"

    inverse = A_TA.inv()
    A_Tb = A_T * b
    x_hat = inverse * A_Tb
    return x_hat


"""
@notice: determines if a matrix is consistent
@reference: https://math.stackexchange.com/questions/104824/how-to-determine-if-a-linear-system-is-solvable 
@param M: Matrix
@returns: bool
"""
def is_solvable(M):
    # "A system doesn't have a unique solution" can happen in two ways: 
    # (1) it can have more than one solution
    # (2) or it can have no solutions. 

    # (1) det must != 0, otherwise there are many solutions
    # (2) we must be able to do Gaussian Row Reduction, otherwise it
    # has no solutions
    if not is_invertible_with_lu(M): return False

    # if all things pass, the system of linear equations is solvable
    return True


"""
@notice: computes determinant using LU decomposition
@param M: Matrix
@returns: bool
"""
def is_invertible_with_lu(M):
    return M.det(method="lu") != 0


"""
@notice: uses RREF values and compares against const_matrix w/ margin of error
@param rref_matrix: Matrix
@param coefficient_matrix: Matrix
@param constant_matrix: Matrix
@returns: list of tuples of gas parameters that are outliers, (i,k)
where `i` is which row the equation is in, and `k` is the index of gas parameter
"""
def find_outliers(rref_matrix, coefficient_matrix, constant_matrix):
    # get rref solutions
    x_hat = []
    rref_row_len = shape(rref_matrix)[ROW]
    rref_col_pos = shape(rref_matrix)[COL]-1
    for i in range(rref_row_len):
        x_i = rref_matrix.row(i)[rref_col_pos]
        x_hat.append(x_i)

    # get theoretical running times using rref values
    computed_running_time = []
    coeff_row_len = shape(coefficient_matrix)[ROW]
    coeff_col_len = shape(coefficient_matrix)[COL]
    for i in range(coeff_row_len):
        total_time = 0
        for j in range(coeff_col_len):
            a_ij = coefficient_matrix.row(i)[j]
            total_time += a_ij * x_hat[j]
        computed_running_time.append(total_time)
    
    # compare w/ margin of error
    outliers = []
    const_row_len = shape(const_matrix)[ROW]
    for i in range(const_row_len):
        # constant vector should always have 1 value
        a_ij = const_matrix.row(i)[0]

        # div by zero handling
        numerator = abs(a_ij-computed_running_time[i])
        diff = 0
        if numerator != 0:
            diff = abs(a_ij-computed_running_time[i]) / computed_running_time[i]

        if diff > MARGIN_OF_ERROR:
            # append all gas parameters corresponding to that equation
            # as a cause to the outlier
            for k in range(coeff_col_len):
                outliers.append((i,k))
    
    return outliers


"""
TESTS
"""
print("===== Coefficient Matrix =====")
#coeff_matrix = add_gas_formula_to_coefficient_matrix([0,1], coeff_matrix)
#coeff_matrix = add_gas_formula_to_coefficient_matrix([1,1], coeff_matrix)
#coeff_matrix = add_gas_formula_to_coefficient_matrix([2,1], coeff_matrix)

#coeff_matrix = add_gas_formula_to_coefficient_matrix([1,1,0], coeff_matrix)
#coeff_matrix = add_gas_formula_to_coefficient_matrix([1,1,1], coeff_matrix)

#coeff_matrix = add_gas_formula_to_coefficient_matrix([3,-1,2], coeff_matrix)
#coeff_matrix = add_gas_formula_to_coefficient_matrix([2,1,1], coeff_matrix)
#coeff_matrix = add_gas_formula_to_coefficient_matrix([1,3,0], coeff_matrix)

coeff_matrix = add_gas_formula_to_coefficient_matrix([2,1,-2], coeff_matrix)
coeff_matrix = add_gas_formula_to_coefficient_matrix([1,-1,-1], coeff_matrix)
coeff_matrix = add_gas_formula_to_coefficient_matrix([1,1,3], coeff_matrix)

print(coeff_matrix)
print("\n")

print("===== Constant Vector =====")
#const_matrix = add_running_time_to_constant_matrix([6], const_matrix)
#const_matrix = add_running_time_to_constant_matrix([0], const_matrix)
#const_matrix = add_running_time_to_constant_matrix([0], const_matrix)

#const_matrix = add_running_time_to_constant_matrix([3], const_matrix)
#const_matrix = add_running_time_to_constant_matrix([5], const_matrix)

#const_matrix = add_running_time_to_constant_matrix([2], const_matrix)
#const_matrix = add_running_time_to_constant_matrix([-1], const_matrix)
#const_matrix = add_running_time_to_constant_matrix([-1], const_matrix)

const_matrix = add_running_time_to_constant_matrix([3], const_matrix)
const_matrix = add_running_time_to_constant_matrix([0], const_matrix)
const_matrix = add_running_time_to_constant_matrix([12], const_matrix)

print(const_matrix)
print("\n")

print("===== Augmented Matrix =====")
aug_matrix = create_augmented_matrix(coeff_matrix, const_matrix)
print(aug_matrix)
print("\n")

print("===== Least Square Solution (rref) =====")
rref = compute_least_square_solution(coeff_matrix, const_matrix)
print(rref)
print("\n")

print("===== Least Square Solution (xhat) =====")
# print(compute_least_square_solutions(coeff_matrix, const_matrix))
print("\n")

print("===== Finding Outliers =====")
print(find_outliers(rref, coeff_matrix, const_matrix))