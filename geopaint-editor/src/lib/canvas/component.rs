// Plabayo — geopaint.app
// Copyright (C) 2022 Glen Henri J. De Cauwsemaecker
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use gloo_render::{request_animation_frame, AnimationFrame};
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, MouseEvent, WebGl2RenderingContext as GL};
use yew::html::Scope;
use yew::{html, Component, Context, Html, NodeRef};

pub enum Msg {
    Render(f64),
    MouseMove(i32, i32),
    MouseClick(i32, i32),
}

pub struct Canvas {
    gl: Option<GL>,
    node_ref: NodeRef,
    _render_loop: Option<AnimationFrame>,
}

impl Component for Canvas {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let node_ref = NodeRef::default();
        Self {
            gl: None,
            node_ref,
            _render_loop: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Render(timestamp) => {
                // Render functions are likely to get quite large, so it is good practice to split
                // it into it's own function rather than keeping it inline in the update match
                // case. This also allows for updating other UI elements that may be rendered in
                // the DOM like a framerate counter, or other overlaid textual elements.
                self.render_gl(timestamp, ctx.link());
                false
            }
            Msg::MouseMove(x, y) => {
                self.on_mouse_move(x, y);
                false
            }
            Msg::MouseClick(x, y) => {
                self.on_mouse_click(x, y);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onmousemove = ctx.link().batch_callback(|event: MouseEvent| {
            Some(Msg::MouseMove(event.offset_x(), event.offset_y()))
        });
        let onclick = ctx.link().batch_callback(|event: MouseEvent| {
            Some(Msg::MouseClick(event.offset_x(), event.offset_y()))
        });
        html! {
            <canvas
                ref={self.node_ref.clone()}
                {onclick}
                {onmousemove}
            />
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        // Once rendered, store references for the canvas and GL context. These can be used for
        // resizing the rendering area when the window or canvas element are resized, as well as
        // for making GL calls.

        let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();

        let gl: GL = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        self.gl = Some(gl);

        // In a more complex use-case, there will be additional WebGL initialization that should be
        // done here, such as enabling or disabling depth testing, depth functions, face
        // culling etc.

        if first_render {
            // The callback to request animation frame is passed a time value which can be used for
            // rendering motion independent of the framerate which may vary.
            let handle = {
                let link = ctx.link().clone();
                request_animation_frame(move |time| link.send_message(Msg::Render(time)))
            };

            // A reference to the handle must be stored, otherwise it is dropped and the render
            // won't occur.
            self._render_loop = Some(handle);
        }
    }
}

impl Canvas {
    fn on_mouse_move(&mut self, x: i32, y: i32) {
        log::info!("mouse move: ({}, {})", x, y);
    }

    fn on_mouse_click(&mut self, x: i32, y: i32) {
        log::info!("mouse click: ({}, {})", x, y);
    }

    fn render_gl(&mut self, timestamp: f64, link: &Scope<Self>) {
        let gl = self.gl.as_ref().expect("GL Context not initialized!");

        let vert_code = include_str!("basic.vert");
        let frag_code = include_str!("basic.frag");

        // This list of vertices will draw two triangles to cover the entire canvas.
        let vertices: Vec<f32> = vec![
            -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
        ];
        let vertex_buffer = gl.create_buffer().unwrap();
        let verts = js_sys::Float32Array::from(vertices.as_slice());

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);

        let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
        gl.shader_source(&vert_shader, vert_code);
        gl.compile_shader(&vert_shader);

        let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
        gl.shader_source(&frag_shader, frag_code);
        gl.compile_shader(&frag_shader);

        let shader_program = gl.create_program().unwrap();
        gl.attach_shader(&shader_program, &vert_shader);
        gl.attach_shader(&shader_program, &frag_shader);
        gl.link_program(&shader_program);

        gl.use_program(Some(&shader_program));

        // Attach the position vector as an attribute for the GL context.
        let position = gl.get_attrib_location(&shader_program, "a_position") as u32;
        gl.vertex_attrib_pointer_with_i32(position, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(position);

        // Attach the time as a uniform for the GL context.
        let time = gl.get_uniform_location(&shader_program, "u_time");
        gl.uniform1f(time.as_ref(), timestamp as f32);

        gl.draw_arrays(GL::TRIANGLES, 0, 6);

        let handle = {
            let link = link.clone();
            request_animation_frame(move |time| link.send_message(Msg::Render(time)))
        };

        // A reference to the new handle must be retained for the next render to run.
        self._render_loop = Some(handle);
    }
}
