#![feature(globs)]

extern crate shader_version;
extern crate sdl2_window;
extern crate piston;
extern crate event;

use sdl2_window::Sdl2Window;
use piston::AssetStore;
use event::WindowSettings;


fn main() {
    let (width, height) = (400, 800);
    let window = Sdl2Window::new(
        shader_version::opengl::OpenGL_3_2,
        WindowSettings {
            title: "R Tetris".to_string(),
            size: [width, height],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
            // background_color: [0.2, 0.2, 0.2, 0.2],
        }
    );

    let mut assets = AssetStore::from_folder("assets");
    // let mut app = tetris::Tetris::new(1.0);
    // app.run(&mut window, &mut assets);
}
