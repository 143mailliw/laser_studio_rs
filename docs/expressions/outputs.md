# Outputs
Outputs are special variables that are used to declare the position and color of the laser.
Unlike inputs, outputs can be both read and written to, just like a normal variable.

## x'
The `x'` output determines the final x coordinate that the laser will be drawn at. The default value of this output is `0`.

## y'
The `y'` output determines the final y coordinate that the laser will be drawn at. The default value of this output is '0'.

## h
The `h` output determines the hue of the laser. The default value of this output is based off of the color the user has picked in Tower Unite, but it is `0` in Laser Studio.

## s
The `s` output determines the saturation of the laser. The default value of this output is based off of the color the user has picked in Tower Unite, but it is `0` in Laser Studio.

## v
The 'v' output determines the value (brightness) of the laser. The default value of this output is '1'.
`0` results in an invisible beam on Tower Unite, and a black dot in Laser Studio. `1` results in the beam being fully visible on Tower Unite, and a full brightness dot on Laser Studio.
