use winit::{
    application::ApplicationHandler, 
    event::{ElementState, KeyEvent, WindowEvent}, 
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, 
    keyboard::{KeyCode, PhysicalKey}, 
    window::{Window, WindowId}
};

use glium::{
    glutin::surface::WindowSurface,
    Display
};

use crate::app::{mesh::Mesh, parser::Obj};

use super::{
    ctx::Ctx,
    rendering::Renderer,
    utils,
    parser,
    parser::Images
};

pub struct App {
    pub window: Window,
    pub display: Display<WindowSurface>,
    pub renderer: Renderer,
    pub ctx: Ctx,
}

impl App {
    pub fn run() {
        utils::print_help();
        match EventLoop::new() {
            Ok(ev) => {
                ev.set_control_flow(ControlFlow::Wait);
                ev.set_control_flow(ControlFlow::Poll);
                let mut ctx = Ctx::new();
                let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
                    .with_inner_size(ctx.width, ctx.height)
                    .with_title("Super Scop :O")
                    .build(&ev);
                let renderer = Renderer::new(&display, & mut ctx);
                let mut app = Self {
                    window: window,
                    display: display,
                    renderer: renderer,
                    ctx: ctx,
                };
                let _ = ev.run_app(&mut app);
            },
            Err(e) => {
                println!("Error: Impossible to init eventloop. {}", e);
                std::process::exit(1);
            }
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        println!("The Application is starting !");
    }
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Window close requested by the user.");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.renderer.draw_obj(&self.display, & mut self.ctx);
                self.window.request_redraw();
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
                            if self.ctx.light_move {
                                self.ctx.light[0] -= 0.25;
                            } else {
                                self.ctx.x_factor += 0.1;
                            }
                        },
                        KeyCode::KeyD => {
                            if self.ctx.light_move {
                                self.ctx.light[0] += 0.25;
                            } else {
                                self.ctx.x_factor -= 0.1;
                            }
                        },
                        KeyCode::KeyS => {
                            if self.ctx.light_move {
                                self.ctx.light[2] -= 0.25;
                            } else {
                                self.ctx.z_factor += 0.5;
                            }
                        },
                        KeyCode::KeyW => {
                            if self.ctx.light_move {
                                self.ctx.light[2] += 0.25;
                            } else {
                                self.ctx.z_factor -= 0.5;
                            }
                        },
                        KeyCode::ArrowDown => {
                            if self.ctx.light_move {
                                self.ctx.light[1] -= 0.25;
                            } else {
                                self.ctx.y_factor += 0.1;
                            }
                        },
                        KeyCode::ArrowUp => {
                            if self.ctx.light_move {
                                self.ctx.light[1] += 0.25;
                            } else {
                                self.ctx.y_factor -= 0.1;
                            }
                        },
                        KeyCode::ArrowLeft => {
                            let speed: f32 = self.ctx.speed_factor;
                            if speed < 0.4 {
                                self.ctx.speed_factor += 0.005;
                            } else {
                                self.ctx.speed_factor = 0.4;
                            }
                        }
                        KeyCode::ArrowRight => {
                            let speed: f32 = self.ctx.speed_factor;
                            if speed > -0.4 {
                                self.ctx.speed_factor -= 0.005;
                            } else {
                                self.ctx.speed_factor = -0.4;
                            }
                        }
                        KeyCode::KeyP => {
                            self.renderer.shaders_switch(& mut self.ctx);             
                        },
                        KeyCode::KeyL => {
                            self.ctx.light_move = !self.ctx.light_move
                        },
                        KeyCode::KeyB => {
                            self.ctx.backface = !self.ctx.backface;
                        }
                        KeyCode::KeyV => {
                            if self.ctx.polmode == 2 {
                                self.ctx.polmode = 0;
                            } else {
                                self.ctx.polmode += 1;
                            }
                        },
                        KeyCode::KeyH => {
                            utils::print_help();
                        }
                        KeyCode::KeyX => {
                            self.ctx.speed_factor *= -1.0;
                        },
                        KeyCode::KeyT => {
                            self.ctx.texture = !self.ctx.texture;
                        },
                        _ => {}
                    }
                }
            },
            WindowEvent::Resized(window_size) => {
                self.display.resize(window_size.into());
                (self.ctx.width, self.ctx.height) = self.display.get_framebuffer_dimensions();
            },
            WindowEvent::DroppedFile(path_buf) => {
                enum ParsedResult {
                    Obj(Obj),
                    Images(Images),
                }
                if let Some(filepath) = path_buf.to_str() {
                    let filepath_lower = filepath.to_lowercase();
                    if !filepath_lower.ends_with(".obj") && !filepath_lower.ends_with(".ppm") {
                        println!("Error: Unsupported file extension.");
                    } else {
                        let ret: Result<ParsedResult, _> = if filepath_lower.ends_with(".obj") {
                            parser::obj_parser(filepath).map(ParsedResult::Obj)
                        } else {
                            Images::new(&self.display, filepath).map(ParsedResult::Images)
                        };
                        match ret {
                            Ok(ParsedResult::Obj(obj)) => {
                                self.ctx.obj = obj;
                                self.renderer.mesh = Mesh::get_mesh_vector(&mut self.ctx);
                            },
                            Ok(ParsedResult::Images(img)) => {
                                self.renderer.img = img;
                                self.renderer.mesh = Mesh::get_mesh_vector(&mut self.ctx);
                            },
                            Err(err) => {
                                println!("Error while parsing: {:?}", err);
                            }
                        }
                    }
                } else {
                    println!("Error: Invalid file path.");
                }
            },
            _ => {}
        }
    }
    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        println!("Application is shutting down...");
        self.display.finish();
        println!("Resources cleaned up.");
    }
}