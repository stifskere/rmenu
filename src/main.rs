use std::process::{Command, Stdio};

use completions::path::get_path_programs;
use components::match_selector::pager::Pager;
use components::text_input::TextInput;
use config::loader::Config;
use flexi_logger::{Logger, colored_default_format};
use log::info;
use sdl2::event::Event;
use sdl2::init as sdl2_init;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::rwops::RWops;
use sdl2::ttf::init as ttf_init;
use sdl2::version::version as sdl2_version;
use utils::errors::handle_app_error;
use utils::vector_matrix::{Vector2I, Vector2U};

mod completions;
mod components;
mod config;
mod utils;

fn main() {
    Logger::try_with_str("DEBUG")
        .expect("Couldn't start logger")
        .format(colored_default_format)
        .start()
        .unwrap();

    info!("Staring r-menu version {}", env!("CARGO_PKG_VERSION"));

    let sdl_context = handle_app_error!(sdl2_init());
    let ttf_context = handle_app_error!(ttf_init());
    let video_subsystem = handle_app_error!(sdl_context.video());
    let display_bounds = handle_app_error!(video_subsystem.display_bounds(0));

    info!("Initialized SDL2 {}", sdl2_version());

    Config::load().unwrap();

    let window_rect =
        Rect::new(0, -(display_bounds.height() as i32 / 2), display_bounds.width(), 20);

    let window = handle_app_error!(
        video_subsystem
            .window("r-menu", window_rect.width(), window_rect.height())
            .position(window_rect.x(), window_rect.y())
            .borderless()
            .build()
    );

    info!("Started window, requested: {window_rect:?}");

    let mut canvas = handle_app_error!(
        window
            .into_canvas()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())
    );

    let texture_creator = canvas.texture_creator();

    let font = handle_app_error!(ttf_context.load_font_from_rwops(
        handle_app_error!(RWops::from_bytes(include_bytes!("../assets/OpenSans-Regular.ttf"))),
        14
    ));

    let mut input = TextInput::new(&font);
    input.set_color(Color::WHITE);
    input.set_position(Vector2I::new(0, 0));

    let minus_a_quarter_window = (window_rect.width() / 2) / 2;

    let mut pager = Pager::new(
        handle_app_error!(get_path_programs())
            .into_iter()
            .collect(),
        &font,
    );
    pager.set_position(Vector2I::new(minus_a_quarter_window as i32, 0));
    pager.set_size(Vector2U::new(
        window_rect.width() - minus_a_quarter_window,
        window_rect.height(),
    ));
    pager.set_text_color(Color::WHITE);
    pager.set_select_color(Color::RGB(255, 165, 0));
    handle_app_error!(pager.compute_text(""));

    let mut shift_pressed = false;
    let mut in_args = false;

    let mut event_pump = handle_app_error!(sdl_context.event_pump());
    'event_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'event_loop,

                Event::KeyDown { keycode: Some(keycode), .. } => match keycode {
                    Keycode::Escape => {
                        info!("Cheerio.");
                        break 'event_loop;
                    },

                    Keycode::LShift | Keycode::RShift => {
                        shift_pressed = true;
                    },

                    Keycode::Tab => {
                        if !in_args {
                            if let Some(selected) = pager.get_selected_entry() {
                                input.set_text(
                                    selected
                                        .item()
                                        .get_original_text(),
                                );
                            }
                        }
                    },

                    Keycode::Return => {
                        let input_args = input.get_args();

                        let mut command = match pager.get_selected_entry() {
                            Some(selected) if input_args.len() <= 1 => {
                                info!(
                                    "Requesting to start '{}'",
                                    selected
                                        .item()
                                        .get_original_text()
                                );
                                Command::new(
                                    selected
                                        .item()
                                        .get_original_text(),
                                )
                            },

                            _ => {
                                if input_args.is_empty() {
                                    continue;
                                }

                                let mut command = Command::new(&input_args[0]);

                                if input_args.len() > 1 {
                                    command.args(&input_args[1..]);
                                }

                                info!("Requesting to start '{}'", input_args.join(" "));

                                command
                            },
                        };

                        command.stdout(Stdio::null());
                        command.stderr(Stdio::null());
                        command.stdin(Stdio::null());

                        #[cfg(unix)]
                        unsafe {
                            use std::os::unix::process::CommandExt;

                            command.pre_exec(|| {
                                sdl2::libc::setsid();
                                Ok(())
                            });
                        }

                        #[cfg(windows)]
                        {
                            use std::os::windows::process::CommandExt;

                            command.creation_flags(0x00000008);
                        }

                        handle_app_error!(command.spawn());

                        info!("Started gracefully... Have a jolly good day!");

                        break 'event_loop;
                    },

                    keycode => {
                        if input.is_caret_at_end() {
                            pager.keycode_interaction(keycode);
                        }

                        if pager.is_caret_at_start() {
                            input.keycode_interaction(keycode);
                        }

                        input.act_char_at_caret(keycode, shift_pressed);

                        let input_args = input.get_args();

                        if let Some(program_name) = input_args.get(0) {
                            handle_app_error!(pager.compute_text(program_name));
                        }

                        in_args = input_args.len() > 1;
                    },
                },

                Event::KeyUp { keycode: Some(keycode), .. } => match keycode {
                    Keycode::LShift | Keycode::RShift => {
                        shift_pressed = false;
                    },

                    _ => {},
                },

                _ => {},
            }
        }

        canvas.set_draw_color(Color::RGB(20, 20, 20));
        canvas.clear();

        handle_app_error!(input.draw(&mut canvas, &texture_creator));

        if !in_args {
            handle_app_error!(pager.draw(&mut canvas));
        }

        canvas.present();
    }
}
