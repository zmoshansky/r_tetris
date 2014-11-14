#![feature(globs)]

extern crate shader_version;
extern crate sdl2_window;
extern crate event;
extern crate event_loop;
extern crate opengl_graphics;

use std::cell::RefCell;
use std::path;
use std::os::self_exe_path;
use sdl2_window::Sdl2Window;
use event::{ WindowSettings, RenderEvent, UpdateEvent, Input};
use event_loop::Events;
use self::opengl_graphics::Gl;

mod tetris;
mod active;
mod tetromino;

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
        }
    );

    let mut app = tetris::Tetris::new(1.0);

    let exe_path = self_exe_path();
    // let exe_path = match exe_path {
    //     Some(path) => path,
    //     None => return Err(
    //             "Could not get the path to executable".to_string()
    //         ),
    // };
    // Ok(exe_path.join(Path::new("assets")));

    let mut asset_path = exe_path.unwrap().join(Path::new("assets"));
    app.load_assets(&mut asset_path);

    let window = RefCell::new(window);
    let mut gl = Gl::new(shader_version::opengl::OpenGL_3_2);

    for e in Events::new(&window) {
        e.render(|r| app.render(window.borrow_mut().deref_mut(), &mut gl, r));
        e.update(|u| app.update(window.borrow_mut().deref_mut(), u));
        // e.input(|u| app.input(window.borrow_mut().deref_mut(), u));
    }
}
