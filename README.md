# Laser Studio

Laser Studio is an IDE (or, at least, it will be when it's finished) for Tower Unite's laser projector expressions.

## Current Features

- Multithreaded expression interpreter
- Basic text editor
- Variable viewer

## Planned Features

- Documentation viewer
- Full debugger
- Syntax & error highlighting
- Auto completion
- Editor tooltips
- Graphical editor (lets you draw pixel art and export it as an expression)
- Savable projects

## FAQ
### Why run on the CPU? Why not the GPU?
While compute shaders and other GPU compute utilities are fantastic for computing small amounts of data, they *aren't* well suited for return large amounts of data at the same time.
While this would certainly be faster, Laser Studio is a development tool, first and foremost, not a playground for running expressions as fast as possible.
We make quite a few trade-offs in order to maximize the accuracy of both the errors and the resulting values, and one of those is running on the CPU.

### Wasn't there a Javascript version of this?
Yes. It was slow. This is much faster. It's also more accurate, and gives better errors.

### How much faster?
It's been tested to be up to 15x faster. It depends on how many cores your CPU has, since this is multithreaded and the old JS version wasn't.