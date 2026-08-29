use winit::event::{ElementState, Event, MouseButton, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main(){
    let event_loop= EventLoop::new();
    let _window= WindowBuilder::new().with_title("Mouse Input").build(&event_loop).unwrap();

    event_loop.run(move| event, _,control_flow|{
        *control_flow= ControlFlow::Wait;

        match event{
            Event::WindowEvent { event,.. }=> match event{
                WindowEvent::MouseInput { state, button, .. }=> {
                    let action= match state{
                        ElementState::Pressed=> "Pressed",
                        ElementState::Released=>"Released",
                    };
                    match button{
                        MouseButton::Left=>{
                            println!("Left Mouse Button {}", action);   
                        }
                        MouseButton::Right=>{
                            println!("Right Mouse Button {}", action);
                        }
                        MouseButton::Middle=>{
                            println!("Middle Mouse Button {}", action);
                        }
                        _=> {}
                    }
                }
                WindowEvent::CloseRequested=>{
                    *control_flow= ControlFlow::Exit;
                }
                _=>{}
            },
            _=>{}
        }
    });
}