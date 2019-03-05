# skribbliobot
drawing bot for skribblio written in rust

```
skribbl.io bot 
A skribbl.io drawing bot, reading an image from the clipboard and drawing it into skribbl.io

USAGE:
    skribbliobot [FLAGS] [OPTIONS] --color-area <color_area> --draw-area <draw_area>

FLAGS:
        --checkerboard       Draw in two stages using a checkerboard pattern
        --dither             Improves image quality, reduces speed
    -h, --help               Prints help information
        --no-batch-colors    Disables drawing colors in a batch
    -V, --version            Prints version information

OPTIONS:
    -c, --color-area <color_area>    The position of the skribbl.io color area in format
                                     x[XWHITE]y[YWHITE]w[WHITE_WIDTH]h[WHITE_HEIGHT]
    -s, --delay <delay>              Drawing delay in ms, too low values may slow down browser [default: 7]
    -d, --draw-area <draw_area>      The position of the skribbl.io drawing area in format
                                     x[XOFFSET]y[YOFFSET]w[WIDTH]h[HEIGHT]
    -t, --timeout <timeout>          Timeout in seconds after which to quit drawing [default: 55]
  ```
  
