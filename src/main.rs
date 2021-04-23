extern crate piston_window;

pub mod materials;

use materials::sand::{Sand};
use materials::{matter::Matter};

type Pixel = (Box<dyn Matter>, f64, f64);

use piston_window::*;
fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", (640, 480))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });


    let mut board : Vec<Pixel> = vec![];
    let mut mouse_position = [0f64; 2];


    while let Some(e) = window.next() {


        if let Some(args) = e.mouse_cursor_args(){
            mouse_position = args;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args(){
            let boxed_matter : Box<dyn Matter> = Box::new(Sand::new(25.0));
            let pixel : Pixel = (boxed_matter, mouse_position[0], mouse_position[1]);

            board.push(pixel);
        }

        if let Some(_args) = e.render_args(){
            window.draw_2d(&e, |c, g, _d| {
                clear([0.0, 0.0, 0.0, 0.0], g);

                for n in &board {
                    rectangle(n.0.get_color(), [n.1, n.2, 2.0, 2.0], c.transform, g);
                }

            });
        }
    }
}