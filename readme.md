# R-MENU

This is a **D-MENU RIIR**, rewritten in the **SDL2** rust bindings, with possibly some bugs.

The project is usable but definitively not finished, and maintenance is not my priority
at this moment, I will be committing to the project as soon as I can.

**This project is only tested in X11 and BSPWM, If you do submit a PR I will be aware of it.**

## Installation

This project for now must be built from source, as I cannot upload a project I damn incomplete.

Since the project consists of a single binary, `cargo build --release` should do it, and then
you can simply move the binary somewhere such as `/usr/bin` if root or somewhere else, and configure
it on **SXHKD** if on **X11**, or whatever you may use in wayland.

## Configuration

This project has a few configuration options, by default it behaves like a white version of **D-MENU**.

The configuration path can be set with an environment variable which you **SHOULD SET BEFORE STARTING
THE PROGRAM FOR THE FIRST TIME**. That variable is `RMENU_CONFIG_PATH` and that path must resolve
to a `.toml` file that contains the following top level definitions.

- `window_position`: The valid values for this are `"top"` and `"bottom"`, relative to the screen.
- `window_padding`: The padding is a vector represented as an array matrix of size 2 `[x, y]`.
- `window_height`: The height is a single integer with the window height.

- `window_background_color`: The background color of the window.
- `text_color`: The color of all the rendered text which is not highlighted.
- `highlight_color`: The selection color of the program selector and more things in the future.
- `highlighted_text_color`: The text under selection, this option is to avoid color overlapping.

- `font_path`: A path containing a **true-text** file, if the option is not provided, open sans
will be loaded instead.
- `font_size`: Depending on the **window_height** this will be the size of all the rendered fonts.

If you configure **SXHKD** or any similar program the `./` path is usually `$HOME` or
from wherever you started your window manager, so the program will create a file there
if when you start the program a configuration file is not present there and the `RMENU_CONFIG_PATH`
variable is not present.

The default values for all the definitions are the ones that may be found in the configuration file
itself, deleting one won't make the program fail, only replace the value with a default one.

## Error debugging

If a configuration error occurs, a fallback window will open showing the error message,
if the window somehow does not open, start the program from a terminal, and logs should
tell you why the error happened, open an issue pasting the error and I'll see what I can
do.

Happy hacking, or whatever JS devs say.
