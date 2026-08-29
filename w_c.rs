use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
fn main(){
    let event_loop= EventLoop::new();
    let _window= WindowBuilder::new()
        .with_title("Hello World")
        .with_inner_size(winit::dpi::LogicalSize::new(800,600))
        .build(&event_loop)
        .expect("Failed to create window");

    event_loop.run(move |event, _,control_flow|{
        *control_flow= ControlFlow::Wait;
        
        match event{
            Event::WindowEvent{event,..}=> match event{
                WindowEvent::CloseRequested=> *control_flow= ControlFlow::Exit,
                _=>{}
            },
            _=>{}
        }
    });
}