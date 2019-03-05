use clap::{App, Arg};
use std::process;

pub struct Box {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub fn get_cli() -> (Box, Box, bool, bool, bool, u64, u64) {
    let clap = App::new("skribbl.io bot")
        .about("A skribbl.io drawing bot, reading an image from the clipboard and drawing it into skribbl.io")
        .arg(
            Arg::with_name("draw_area")
                .long("draw-area")
                .short("d")
                .required(true)
                .help("The position of the skribbl.io drawing area in format x[XOFFSET]y[YOFFSET]w[WIDTH]h[HEIGHT]")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("color_area")
                .long("color-area")
                .short("c")
                .required(true)
                .help("The position of the skribbl.io color area in format x[XWHITE]y[YWHITE]w[WHITE_WIDTH]h[WHITE_HEIGHT]")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("dither")
                .long("dither")
                .takes_value(false)
                .help("Improves image quality, reduces speed"),
        )
        .arg(
            Arg::with_name("checkerboard")
                .long("checkerboard")
                .takes_value(false)
                .help("Draw in two stages using a checkerboard pattern"),
        )
        .arg(
            Arg::with_name("no_batch_colors")
                .long("no-batch-colors")
                .takes_value(false)
                .help("Disables drawing colors in a batch"),
        )
        .arg(
            Arg::with_name("delay")
                .long("delay")
                .short("s")
                .default_value("7")
                .help("Drawing delay in ms, too low values may slow down browser"),
        )
        .arg(
            Arg::with_name("timeout")
                .long("timeout")
                .short("t")
                .default_value("55")
                .help("Timeout in seconds after which to quit drawing"),
        )
        .get_matches();

    let drawing_area = {
        let values: Vec<_> = clap
            .value_of("draw_area")
            .unwrap()
            .split(|c| "xywh".contains(c))
            .filter(|&c| c != "")
            .map(|num| match num.parse::<u32>() {
                Ok(num) => num,
                Err(err) => {
                    println!("Failed to parse drawing area: {}", err);
                    process::exit(1);
                }
            })
            .collect();

        Box {
            x: values[0],
            y: values[1],
            width: values[2],
            height: values[3],
        }
    };
    let color_box = {
        let values: Vec<_> = clap
            .value_of("color_area")
            .unwrap()
            .split(|c| "xywh".contains(c))
            .filter(|&c| c != "")
            .map(|num| match num.parse::<u32>() {
                Ok(num) => num,
                Err(err) => {
                    println!("Failed to parse color area: {}", err);
                    process::exit(1);
                }
            })
            .collect();

        Box {
            x: values[0],
            y: values[1],
            width: values[2],
            height: values[3],
        }
    };

    let dither = clap.is_present("dither");
    let checkerboard = clap.is_present("checkerboard");
    let batch_colors = !clap.is_present("no_batch_colors");
    let delay: u64 = match clap.value_of("delay").unwrap().parse() {
        Ok(delay) => delay,
        Err(err) => {
            println!("Failed to parse delay: {}", err);
            process::exit(1);
        }
    };
    let timeout: u64 = match clap.value_of("timeout").unwrap().parse() {
        Ok(timeout) => timeout,
        Err(err) => {
            println!("Failed to parse timeout: {}", err);
            process::exit(1);
        }
    };

    (
        drawing_area,
        color_box,
        dither,
        checkerboard,
        batch_colors,
        delay,
        timeout,
    )
}
