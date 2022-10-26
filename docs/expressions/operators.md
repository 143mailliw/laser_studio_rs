# Operators
Operators allow you to perform simple operations, such as addition, multiplication, and exponential multiplication, as well as logical operations like OR, NOT and AND.

Operators are listed in the order they are performed.

## ^ (Exponent)
Returns the left hand side (used as the base) to the power of the right hand side (used as the exponent).

## * (Multiplication)
Returns the left hand side multiplied by the right hand side.

## / (Division)
Returns the left hand side divided by the right hand side.

## % (Modulo)
Returns the remainder of the left hand side divided by the right hand side.

## + (Addition)
Returns the sum of the left and right hand sides.

## - (Subtraction)
Returns the difference of the left and right hand sides.

## <= (Less Than or Equal To)
Returns 1 if the left hand side is less than or equal to the right hand side. If it isn't, returns 0.

## >= (Greater Than or Equal To)
Returns 1 if the left hand side is greater than or equal to the right hand side. If it isn't, returns 0.

## == (Equal To)
Returns 1 if the left hand side is equal to the right hand side. If it isn't, returns 0.

## < (Less Than)
Returns 1 if the left hand side is less than the right hand side. If it isn't, returns 0.

## > (Greater Than)
Returns 1 if the left hand side is greater than the right hand side. If it isn't, returns 0.

## & (AND)
Returns 1 if the left hand side AND right hand side are both 1 or greater than 1. If they aren't, returns 0.

## | (OR)
Returns 1 if the left hand side OR right hand side are 1 or greater than 1. If neither are, returns 0.

# Special Operators

## - (Negate)
Negates the number, group, variable reference, or function call after it.

## ! (NOT)
Returns 1 if the number, group, variable reference, or function call after it is less than 1. If it isn't, returns 0.

## # (Comment)
Useful for taking notes. Anything written after a # will be ignored, until the start of the next line.

## Shorthand Multiplication
As long as the left hand side is a number, multiplication can be shortened. For example:
- `2 * 3` is the same as `2(3)`.
- `2 * variable` is the same as `2variable`.
- `2 * (30 + 10)` is the same as `2(30+10)`.
- `2 * sin(30)` is the same as `2sin(30)`.
