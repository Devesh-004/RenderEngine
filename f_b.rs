use minifb::{Key, Window, WindowOptions};
const WIDTH: usize= 640;
const HEIGHT: usize= 360;

fn main(){
    let mut buffer: Vec<u32>= vec![0; WIDTH*HEIGHT];

    let mut window= Window::new(
        "Test- Esc to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )

    .unwrap_or_else(|e| panic!("{}", e));

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape){
        /*
        for pixel in buffer.iter_mut(){
            *pixel= 0xFF0000;
        }
        */
        /**/
        for i in buffer.iter_mut(){
            *i= 0;
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}