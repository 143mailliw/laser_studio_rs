# Inputs
Inputs are special variables that are assigned their values by the expression runtime.
The values of input variables cannot be changed in your expression. Attempting to do so will result in a compilation error in Tower Unite, and a runtime error in Laser Studio.

## `x`
The `x` input specifies the x coordinate that the index belongs to in the original shape the expression is based off of (eg. a rectangular grid).

On Laser Studio (which only supports rectangular grids), this variable is equivalent to the following expression:
```
-100 + (findex % x_size)*(200/(x_size - 1))
```

## `y`
The `y` input specifies the y coordinate that the index belogns to in the original shape the expression is based off of (eg. a rectangular grid).

On Laser Studio (which only supports rectangular grids), this variable is equivalent to the following expression:
```
100 - (200/(y_size - 1)) * floor(index / x_size)
```
