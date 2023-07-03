#import numpy as np
from sympy import *

"""
1a + 2b = 3
4a + 5b = 6

vec<u64> u64 -> (vec<vec<u64>>, vec<u64>)
LHS RHS -> (coefficient matrix, constant matrix)
[1,2] 3
[4,5] 6

coefficient matrix:
[
    [1,2]
    [4,5]
]

constant matrix:
[3, 6]
"""

coeff_matrix = Matrix([])
const_matrix = Matrix([])

"""
@param formula: vec<u64>
@param coefficient_matrix: vec<vec<u64>>
@returns: vec<vec<u64>> 
"""
def add_gas_formula_to_coefficient_matrix(formula, coefficient_matrix):
    return coefficient_matrix.row_insert(0, Matrix([formula]))


"""
@param running_time: vec<u64>
@param constant_matrix: vec<u64>
@returns: vec<u64>
"""
def add_running_time_to_constant_matrix(running_time, constant_matrix):
    return constant_matrix.row_insert(0, Matrix(running_time))


"""
@param coefficient_matrix: vec<vec<u64>>
@param constant_matrix: vec<u64>
@returns: vec<vec<u64>>
"""
def create_augmented_matrix(coefficient_matrix, constant_matrix):
    return coefficient_matrix.row_join(constant_matrix)


"""
@param A: coefficient matrix
@param b: constant matrix
@returns: Matrix
"""
def solve_linear_equations(A, b):
    # find A transposed
    A_T = A.transpose()

    # find matrix A^T * A and vector A^T * b
    A_TA = A_T * A
    A_Tb = A_T * b
    augmented_matrix = create_augmented_matrix(A_TA, A_Tb)
    answer = augmented_matrix.rref()
    print(answer)
    



"""
TESTS
"""
print("===== Coefficient Matrix =====")
coeff_matrix = add_gas_formula_to_coefficient_matrix([1,2], coeff_matrix)
coeff_matrix = add_gas_formula_to_coefficient_matrix([4,5], coeff_matrix)
print(coeff_matrix)
print("\n")

print("===== Constant Matrix =====")
const_matrix = add_running_time_to_constant_matrix([3], const_matrix)
const_matrix = add_running_time_to_constant_matrix([6], const_matrix)
print(const_matrix)
print("\n")

print("===== Augmented Matrix =====")
aug_matrix = create_augmented_matrix(coeff_matrix, const_matrix)
print(aug_matrix)
print("\n")

print("===== Solving Linear Equations =====")
solve_linear_equations(coeff_matrix, const_matrix)