extern crate piston_window;

pub mod materials;

use materials::sand::{Sand};
use materials::{matter::Matter};

use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", (640, 480))
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });


    let mut board : Vec<Option<Box<dyn Matter>>> = vec![];

    for _ in 0..(640 * 480){
        board.push(None);
    }

    let mut mouse_position = [0usize; 2];

    let mut is_pressed = false;


    while let Some(e) = window.next() {


        if let Some(args) = e.mouse_cursor_args(){
            mouse_position[0] = (args[0].round()) as usize;
            mouse_position[1] = (args[1].round()) as usize;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args(){
            let boxed_matter : Box<dyn Matter> = Box::new(Sand::new(25.0));

            board[mouse_position[0] + (mouse_position[1] * 640)] = Some(boxed_matter);
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args(){
            is_pressed = true;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.release_args(){
            is_pressed = false;
        }

        if(is_pressed){
            let boxed_matter : Box<dyn Matter> = Box::new(Sand::new(25.0));

            board[mouse_position[0] + (mouse_position[1] * 640)] = Some(boxed_matter);
        }


        if let Some(_args) = e.render_args(){
            window.draw_2d(&e, |c, g, _d| {
                clear([0.0, 0.0, 0.0, 0.0], g);

                for n in 0..(640 * 480) {
                    if let Some(matter) =  &board[n]{
                        let x = n % 640;
                        let y = n / 640;
                        rectangle(matter.get_color(), [x as f64, y as f64, 2f64, 2f64], c.transform, g);
                    }
                  
                }

            });
        }

        if let Some(_args) = e.update_args(){
            for i in 0usize..(640 * 479){
                if let Some(matter) = &board[i]{
                    if *(&board[i + 640].is_none()){
                        board[i] = None;
                        board[i + 640] = Some(Box::new(Sand::new(25f32)));
                    }

                    if let Some(_) = &board[i + 640]{
                        if *(&board[i + 639].is_none()){
                            board[i] = None;
                            board[i + 639] = Some(Box::new(Sand::new(25f32)));
                        }
                        
                        // if *(&board[i + 641].is_none()){
                        //     board[i] = None;
                        //     board[i + 641] = Some(Box::new(Sand::new(25f32)));
                        // }
                    }
                }
            }
        }
    }
}