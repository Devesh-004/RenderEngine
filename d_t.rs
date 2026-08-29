use piston_window::*;
fn main(){
    let mut window: PistonWindow= WindowSettings::new("Text", [512, 512])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut glyphs= window
        .load_font("assets/FreeSerif.ttf")
        .unwrap();

    while let Some(e)= window.next(){
        window.draw_2d(&e, |ctx, g, device|{
            clear([0.2; 4], g);

            text::Text::new_color([1.0,1.0,1.0,1.0], 30)
                .draw(
                    "Hello Rust!", 
                    &mut glyphs, 
                    &ctx.draw_state,
                    ctx.transform.trans(100.0, 100.0), 
                    g,
                )
                .unwrap();

            glyphs.factory.encoder.flush(device);
        });
    }
}