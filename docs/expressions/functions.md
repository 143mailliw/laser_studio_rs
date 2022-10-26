# Functions
The expression language includes several built in functions that allow you to perform specific actions.
Functions can be called by typing the name of the function, an opening parenthesis, the parameters (seperated by commas), and then a closing parenthesis.
The resulting complete function call will look like this: `functionName(parameter1, parameter2)`.

## sin
`sin(x)`

Returns the sine of 'x'.

## cos
`cos(x)`

Returns the cosine of 'x'.

## tan
`tan(x)`

Returns the tangent of `x`.

## asin
`asin(x)`

Returns the inverse (arc) sine of `x`.

## acos
`acos(x)`

Returns the inverse (arc) cosine of 'x'.

## atan
`atan(x)`

Returns the inverse (arc) tangent of `x`.

## atan2
`atan2(a, b)`

Returns the inverse (arc) tangent of `b / a`.

## sqrt
`sqrt(x)`

Returns the square root of the input value.

## min
`min(a, b)`

Returns the smallest number of the two inputs, `a` and `b`.

## max
`max(a, b)`

Returns the biggest number of the two inputs, `a` and `b`.

## floor
`floor(x)`

Returns the nearest whole number less than or equal to the input value.

## ceil
`ceil(x)`

Returns the nearest whole number greater than or equal to the input value.

## round
`round(x)`

Returns the nearest whole number to the input value.

## abs
`abs(x)`

Returns the absolute value of the input value.

## rand
`rand()`

Returns a random number between 0 and 1.
This can be manipulated to get a random number within any range, using the following formula:
`minimum + rand() * (maximum - minimum)`.

## lerp
`lerp(fraction, a, b)`

Interpolates between `a` and `b` using `fraction`, returning the interpolated value.

Contrary to the official Tower Unite documentation, this function is equivalent to the following expression:
`(a * (1.0 - fraction) + b * fraction)`.

## if
`if(float, a, b)`

Returns `a` if `float` is greater than or equal to 1, otherwise returns `b`.
