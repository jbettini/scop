use winit::{
    application::ApplicationHandler, 
    event::{WindowEvent,ElementState, KeyEvent}, 
    event_loop::{ActiveEventLoop, EventLoop, ControlFlow},
    window::{Window, WindowId},
    keyboard::{KeyCode, PhysicalKey}
};

use glium::{
    glutin::surface::WindowSurface,
    Display
};

use super::{
    ctx::Ctx,
    teapot::{NORMALS, VERTICES, INDICES},
    object::Object
};

pub struct App {
    pub window: Option<Window>,
    pub display: Option<Display<WindowSurface>>,
    pub event_loop: Option<EventLoop<()>>,
    pub form: Option<Object>,
    pub ctx: Ctx
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            display: None,
            event_loop: Some(EventLoop::new().unwrap()),
            form: None,
            ctx: Ctx::default()
        }
    }
    pub fn init_display(&mut self) {
        let event_loop = self.event_loop.as_ref().expect("EventLoop should be initialized");
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.set_control_flow(ControlFlow::Wait);
        let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .with_inner_size(self.ctx.width, self.ctx.height)
            .with_title("Super Scop :O")
            .build(event_loop);
        self.form = Some(Object::new(&display, &VERTICES[..], &NORMALS[..], &INDICES[..]));
        self.display = Some(display);
        self.window = Some(_window);
    }
    pub fn run(mut self) {
        if let Some(event_loop) = self.event_loop.take() {
            let _ = event_loop.run_app(&mut self);
        } else {
            panic!("EventLoop not initialized");
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        if self.display.is_none() {
            self.init_display();
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.window.as_ref().expect("Error: Window is not initialized !").request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Window close requested by the user.");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                match self.form {
                    Some(ref mut object) => {
                        object.draw_obj(self.display.as_ref().expect("Error: Display Inexisting!"), & mut self.ctx);
                    },
                    None => {
                        println!("Aucun objet n'est présent");
                    }
                }
                self.window.as_ref().unwrap().request_redraw();
            },
            WindowEvent::KeyboardInput { device_id: _device_id, event, is_synthetic } => {
                if is_synthetic {
                    return ;
                }
                if let KeyEvent { 
                    physical_key: PhysicalKey::Code(key_code),
                    state: ElementState::Pressed,
                    ..
                } = event {
                    match key_code {
                        KeyCode::Escape => {
                            println!("Escape key pressed - closing the application.");
                            event_loop.exit();
                        },
                        KeyCode::Space => {
                            self.ctx.rotation = !self.ctx.rotation;
                        },
                        KeyCode::KeyA => {
                            self.ctx.x_factor += 0.05;
                        },
                        KeyCode::KeyD => {
                            self.ctx.x_factor -= 0.05;
                        },
                        KeyCode::KeyS => {
                            self.ctx.z_factor += 0.05;
                        },
                        KeyCode::KeyW => {
                            self.ctx.z_factor -= 0.05;
                        },
                        KeyCode::KeyL => {
                            if let Some(form) = &mut self.form {
                                form.shaders_switch(& mut self.ctx);
                            }                   
                        },
                        KeyCode::KeyB => {
                            self.ctx.backface = !self.ctx.backface;
                        }
                        KeyCode::KeyV => {
                            self.ctx.polmode = !self.ctx.polmode;
                        }
                        _ => {
                            // TODO: Autre input clavier
                        }
                    }
                }
            },
            WindowEvent::Resized(window_size) => {
                self.display.as_ref().expect("Error: Display not initialized.").resize(window_size.into());
                (self.ctx.width, self.ctx.height) = self.display.as_ref().unwrap().get_framebuffer_dimensions();
                // TODO: Gerer les changement de taille de fenetre
            },
            _ => {}
        }
    }
    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        println!("Application is shutting down...");
        self.display.as_ref().unwrap().finish();
        println!("Display resources cleaned up.");
        if let Some(window) = self.window.take() {
            drop(window);
            println!("Window resources cleaned up.");
        }
        self.event_loop = None;
    }
}


impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}