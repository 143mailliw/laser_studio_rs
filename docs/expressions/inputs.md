# Inputs
Inputs are special variables that are assigned their values by the expression runtime.
The values of input variables cannot be changed in your expression. Attempting to do so will result in a compilation error in Tower Unite, and a runtime error in Laser Studio.

## `x`
The `x` input specifies the x coordinate that the index belongs to in the original shape the expression is based off of (eg. a rectangular grid).

On a rectangular grid, this variable is equivalent to the following expression:
```
-100 + (findex % x_size)*(200/(x_size - 1))
```

## `y`
The `y` input specifies the y coordinate that the index belogns to in the original shape the expression is based off of (eg. a rectangular grid).

On a rectangular grid, this variable is equivalent to the following expression:
```
100 - (200/(y_size - 1)) * floor(index / x_size)
```

## `index`
The `index` input specifies which index is currently being calculated.

On a rectangular grid, this value will be within an exclusive range of `0` through `(x_size * y_size)`.
For example, if you have a grid of size 20 x 20, the top left corner will be 0 and the bottom right corner will be 399.

## `count`
The `count` input specifies the amount of lasers being rendered.

On a rectangluar grid, this variable is equivalent to the following expression:
```
x_size * y_size - 1
```

## `fraction`
The `fraction` input provides a fraction representing the ratio between the current index and the maximum index.

This variable is equivalent to the following expression:
```
index / count
```

## `time`
The `time` input specifies the current time on your computer, in seconds.

In Tower Unite, this is your local time. On Laser Studio, this is currently UTC.

## `projectionStartTime`
The `projectionStartTime` input specifies the time that the expression started running, in seconds.

As with the `time` input, this is based on your local time in Tower Unite, and UTC on Laser Studio.

## `projectionTime`
The `projectionTime` input specifies the time since the expression start running, in seconds.

As with the `time` input, this is based on your local time in Tower Unite, and UTC on Laser Studio.

This variable is equivalent to the following expression:
```
time - projectionStartTime
```

## `pi`
Pi. Equivalent to typing out 3.1415926535898...

## `tau`
Double pi. Equivalent to typing out `2 * pi` or 6.2831853071796...
