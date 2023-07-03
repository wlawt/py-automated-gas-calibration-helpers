## Automated Gas Calibration helpers

A collection of mathematical helper functions written in Python. 

### Usage

```python
python3 main.py
```

### Example

Consider the linear system shown in the code test:

2x	+	y	−	2z	=	3

x	−	y	−	z	=	0

x	+	y	+	3z	=	12

The Coefficient Matrix would be represented as:

[
    [1,  1,  3]
    [1, -1, -1]
    [2,  1, -2]
]

The Constant Vector would be represented as (a Matrix):

[
    [12]
    [0]
    [3]
]

The Augmented Matrix would be the matrix: Coefficient | Constant

Performing RREF, we would get the following solutions:

[7/2, 1, 5/2]

And to detect for outliers, if we plug the computed values against the RHS
(the benchmarked running time), we can check and account for a margin of error.

Note: it is obvious that with a small sample size and integer values, plugging 
the values back into what we just solved gives us no margin of error. However, 
for larger systems of equations and floating points, this should offer some room
of flexibility, in which we can start filtering values.