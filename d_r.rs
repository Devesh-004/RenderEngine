/*
use minifb::{Key, Window, WindowOptions};
const WIDTH: usize= 640;
const HEIGHT: usize= 480;

fn main(){
    let mut buffer: Vec<u32>= vec![0; WIDTH * HEIGHT];

    let rect_x= 100;
    let rect_y= 50;
    let rect_w= 200;
    let rect_h= 100;
    
    let rect_color: u32= 0xFF0000FF;

    for y in rect_y..(rect_y + rect_h){
        for x in rect_x..(rect_x + rect_w){
            if x < WIDTH && y < HEIGHT{
                let index = y * WIDTH + x;
                buffer[index]= rect_color;
            }
        }
    }

    let mut window= Window::new(
        "Rectangle",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e|{
        panic!("{}", e);
    });
    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape){
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
*/

use macroquad::prelude::*;
#[macroquad::main("Rectangle")]
async fn main(){
    loop{
        clear_background(LIGHTGRAY);
        draw_rectangle(100.0, 100.0, 120.0, 60.0, GREEN);
        next_frame().await;
    }
}