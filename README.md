# Laser Studio

Laser Studio is an IDE (or, at least, it will be when it's finished) for Tower Unite's laser projector expressions.

## Current Features

- Multithreaded expression interpreter
- Basic text editor
- Variable viewer

## Planned Features

- Documentation viewer
- Full debugger
- Redo, find & replace
- Syntax & error highlighting
- Auto completion
- Editor tooltips
- Graphical editor (lets you draw pixel art and export it as an expression)

## FAQ
### Why run on the CPU? Why not the GPU?
While compute shaders and other GPU compute utilities are fantastic for computing small amounts of data, they *aren't* well suited for return large amounts of data at the same time.
While this would certainly be faster, Laser Studio is a development tool, first and foremost, not a playground for running expressions as fast as possible.
We make quite a few trade-offs in order to maximize the accuracy of both the errors and the resulting values, and one of those is running on the CPU.

### Wasn't there a Javascript version of this?
Yes. It was slow. This is much faster. It's also more accurate, and gives better errors.

### How much faster?
It's been tested to be up to 15x faster. It depends on how many cores your CPU has, since this is multithreaded and the old JS version wasn't.

### Why doesn't my expression work? It runs fine in Tower, but doesn't run at all in Laser Studio!
Laser Studio is signifigantly more strict about what you can get away with. Check the errors, which can be shown by clicking the Errors button in the Render tab.
Laser Studio tries to prevent undefined behavior occuring, so things you can get away with in Tower Unite like missing arguments, parentheses, and semi-colons don't work in in Laser Studio.

Here's a list of some of the problems you might run into from Laser Studio's stricter parser:
- Variable assignments must end in a semi-colon.
- Functions cannot be run with more or less arguments than expected.
- Un-matched parentheses will always result in an error.
- You cannot use a variable before it has been assigned to.

### Why doesn't my expression work? I'm not getting any errors, but everything's in the wrong place!
If you found errors you had to correct before your expression ran in Laser Studio, your expression may have relied in quirks in Tower Unite's laser projection.
Try running the corrected expression in Tower Unite, to see if it behaves the same way. If it does, you'll have to fix your expression.

However, if your *error-free* code causes inaccuracies between Laser Studio and Tower Unite, please submit a Github issue - that shouldn't happen.
