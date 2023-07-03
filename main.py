import numpy as np

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

coeff_matrix = np.array([])
const_matrix = np.array([])


"""
@param formula: vec<u64>
@param coefficient_matrix: vec<vec<u64>>
@returns: vec<vec<u64>> 
"""
def add_gas_formula_to_coefficient_matrix(formula, coefficient_matrix):
    if coefficient_matrix.size == 0:
        return np.hstack((coefficient_matrix, formula))
    return np.vstack((coefficient_matrix, formula))

"""
@param running_time: u64
@param constant_matrix: vec<u64>
@returns: vec<u64>
"""
def add_running_time_to_constant_matrix(running_time, constant_matrix):
    return np.append(constant_matrix, running_time)


"""
TESTS
"""
coeff_matrix = add_gas_formula_to_coefficient_matrix([1,2], coeff_matrix)
coeff_matrix = add_gas_formula_to_coefficient_matrix([4,5], coeff_matrix)
print(coeff_matrix)

const_matrix = add_running_time_to_constant_matrix(3, const_matrix)
const_matrix = add_running_time_to_constant_matrix(6, const_matrix)
print(const_matrix)
