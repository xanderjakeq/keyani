use device_query::DeviceQuery;
use device_query::{DeviceState, Keycode as QueryKeycode};
use notan::draw::*;
use notan::prelude::*;

use std::fs::{self, read, DirEntry};
use std::path::PathBuf;

mod utils;

use utils::*;

#[derive(AppState)]
pub struct State {
    device_state: DeviceState,
    x: f32,
    y: f32,
    last_key: Option<QueryKeycode>,
    images: Vec<Texture>,
    index: usize,
    allow_rerender: bool,
    window_width: i32,
    window_height: i32,
}

#[notan_main]
fn main() -> Result<(), String> {
    let window_config = WindowConfig::new()
        .title("keyani")
        .size(1026, 600) // window's size
        .vsync(true) // enable vsync
        .resizable(false) // window can be resized
        .min_size(600, 400); // Set a minimum window size
                             //.window_icon(Some(PathBuf::from("./examples/assets/rust.ico")))
                             //.taskbar_icon(Some(PathBuf::from("./examples/assets/rust.ico")));

    notan::init_with(setup)
        .add_config(DrawConfig)
        .add_config(window_config)
        .update(update)
        .draw(draw)
        .build()
}

fn setup(gfx: &mut Graphics) -> State {
    let files = match fs::read_dir("../images") {
        Ok(paths) => {
            let mut dir_entries: Vec<DirEntry> = paths.map(|r| r.unwrap()).collect();
            dir_entries.sort_by_key(|dir| dir.path());

            let mut path_vec: Vec<PathBuf> = vec![];

            for path in dir_entries {
                path_vec.push(path.path());
            }

            path_vec
        }
        Err(err) => {
            println!("{:?}", err);
            vec![]
        }
    };

    println!("{:?}", files);

    let textures: Vec<Texture> = files
        .into_iter()
        .map(|path| {
            let image = read(path).unwrap();

            return gfx
                .create_texture()
                .from_image(image.as_slice())
                .build()
                .unwrap();
        })
        .collect();

    let d_state = DeviceState::new();

    State {
        device_state: d_state,
        images: textures,
        x: 0.0,
        y: 0.0,
        last_key: None,
        index: 0,
        allow_rerender: true,
        window_width: 1026,
        window_height: 600,
    }
}

fn update(app: &mut App, state: &mut State) {
    let keys: Vec<QueryKeycode> = state.device_state.get_keys();

    let last_key = keys.last().copied();

    if keys.len() > 0 && state.last_key.is_none() && state.last_key != last_key {
        state.index = rand(1, state.images.len());
    } else {
        state.index = 0;
    }

    if should_rerender(app, state, &last_key) {
        state.allow_rerender = true;
        state.window_width = app.window().width();
        state.window_height = app.window().height();
    }

    state.last_key = last_key;
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    if state.allow_rerender {
        let img_size = state.window_width as f32 / 2.0;
        let mut draw = gfx.create_draw();
        draw.clear(Color::GREEN);

        //TODO:
        //- center the thing
        //- add configuration options
        //- async asset loading?
        draw.image(&state.images[state.index])
            .position(
                (state.window_width as f32 / 2.0) - (img_size / 2.0),
                (state.window_height as f32 / 2.0) - (img_size / 2.0),
            )
            .size(img_size, img_size);
        gfx.render(&draw);
        state.allow_rerender = false;
    }
}
