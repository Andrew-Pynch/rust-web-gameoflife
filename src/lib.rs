mod world;
use std::cell::RefCell;
use std::rc::Rc;

use crate::world::World;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;
use web_sys::WebGlProgram;
use web_sys::WebGlShader;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
pub struct WorldWrapper(Rc<RefCell<World>>);

// lib.rs

#[wasm_bindgen]
impl WorldWrapper {
    pub fn new(width: usize, height: usize) -> WorldWrapper {
        let mut world = World::new(width, height);
        world.randomize();
        WorldWrapper(Rc::new(RefCell::new(world)))
    }

    pub fn update_and_render(&self, canvas: HtmlCanvasElement) {
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let width = 64;
        let height = 64;
        let cell_size = 10.0; // Size of each cell in pixels

        // Clone the Rc<RefCell<World>> to use inside the closure
        let world_clone = self.0.clone();

        // Update the world state
        self.0.borrow_mut().update();

        // Render the world
        context.clear_rect(
            0.0,
            0.0,
            width as f64 * cell_size,
            height as f64 * cell_size,
        );

        let world = self.0.borrow();
        for y in 0..height {
            for x in 0..width {
                let index = world.get_index(x, y);
                let cell = world.cells[index];

                // Set the fill style based on the cell state (alive or dead)
                context.set_fill_style(&JsValue::from_str(if cell { "black" } else { "white" }));

                // Draw the rectangle for the cell
                context.fill_rect(
                    x as f64 * cell_size,
                    y as f64 * cell_size,
                    cell_size,
                    cell_size,
                );
            }
        }

        // Schedule the next frame
        let closure = Closure::wrap(Box::new(move || {
            // Use the cloned value inside the closure
            world_clone.borrow_mut().update();
            // Render the world using the existing code
            context.clear_rect(
                0.0,
                0.0,
                width as f64 * cell_size,
                height as f64 * cell_size,
            );
            let world = world_clone.borrow();
            for y in 0..height {
                for x in 0..width {
                    let index = world.get_index(x, y);
                    let cell = world.cells[index];
                    context.set_fill_style(&JsValue::from_str(if cell {
                        "black"
                    } else {
                        "white"
                    }));
                    context.fill_rect(
                        x as f64 * cell_size,
                        y as f64 * cell_size,
                        cell_size,
                        cell_size,
                    );
                }
            }
        }) as Box<dyn FnMut()>);

        window()
            .expect("Failed to get window object") // Unwrap the Option<Window> here
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .unwrap();

        closure.forget();
    }
}

fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

// Link a program
fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create program object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program")))
    }
}
