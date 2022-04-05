use bevy::prelude::*;
use bevy_webgl1::util::{bind_array_buffer, compile_shader, get_gl, link_program};
use js_sys::{Float32Array, Uint8Array};
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext as Gl};
fn main() -> Result<(), String> {
    //Setup
    let gl = get_gl("canvas");
    let vert_shader = compile_shader(&gl, Gl::VERTEX_SHADER, include_str!("shaders/mesh.vert"))?;
    let frag_shader = compile_shader(&gl, Gl::FRAGMENT_SHADER, include_str!("shaders/mesh.frag"))?;
    let program = link_program(&gl, &vert_shader, &frag_shader)?;
    gl.use_program(Some(&program));

    //Get locations
    let position_location = gl.get_attrib_location(&program, "a_position");
    // let color_location = gl.get_attrib_location(&program, "a_color");
    let matrix_location = gl.get_uniform_location(&program, "u_matrix").unwrap();

    //fill position buffer
    let position_buffer = bind_array_buffer(&gl);
    set_geometry(&gl);
    //fill color buffer
    // let color_buffer = bind_array_buffer(&gl);
    // set_color(&gl);

    //draw scene
    let canvas = gl.canvas().unwrap().unchecked_into::<HtmlCanvasElement>();
    let width = canvas.width();
    let height = canvas.height();
    gl.viewport(0, 0, width as i32, height as i32);
    gl.clear(Gl::COLOR_BUFFER_BIT | Gl::DEPTH_BUFFER_BIT);
    gl.enable(Gl::CULL_FACE);
    gl.enable(Gl::DEPTH_TEST);
    gl.use_program(Some(&program));
    //Do positions
    gl.enable_vertex_attrib_array(position_location as u32);
    gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&position_buffer));
    gl.vertex_attrib_pointer_with_i32(position_location as u32, 3, Gl::FLOAT, false, 0, 0);
    //do colors
    // gl.enable_vertex_attrib_array(color_location as u32);
    // gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&color_buffer));
    // gl.vertex_attrib_pointer_with_i32(color_location as u32, 3, Gl::UNSIGNED_BYTE, true, 0, 0);
    //do the matrix
    let aspect = (width as f32) / (height as f32);
    let z_near = 1.0;
    let z_far = 2000.0;
    let matrix = Mat4::perspective_rh_gl(60.0_f32.to_radians(), aspect, z_near, z_far);
    let matrix = matrix * Mat4::from_translation([-150.0, 0.0, -360.0].into());
    // let matrix = matrix * Mat4::from_quat()
    gl.uniform_matrix4fv_with_f32_array(Some(&matrix_location), false, &matrix.to_cols_array());
    gl.draw_arrays(Gl::TRIANGLES, 0, 16 * 6);
    Ok(())
}

fn set_geometry(gl: &Gl) {
    let geometry = [
        // left column front
        0.0, 0.0, 0.0, 0.0, 150.0, 0.0, 30.0, 0.0, 0.0, 0.0, 150.0, 0.0, 30.0, 150.0, 0.0, 30.0,
        0.0, 0.0, // top rung front
        30.0, 0.0, 0.0, 30.0, 30.0, 0.0, 100.0, 0.0, 0.0, 30.0, 30.0, 0.0, 100.0, 30.0, 0.0, 100.0,
        0.0, 0.0, // middle rung front
        30.0, 60.0, 0.0, 30.0, 90.0, 0.0, 67.0, 60.0, 0.0, 30.0, 90.0, 0.0, 67.0, 90.0, 0.0, 67.0,
        60.0, 0.0, // left column back
        0.0, 0.0, 30.0, 30.0, 0.0, 30.0, 0.0, 150.0, 30.0, 0.0, 150.0, 30.0, 30.0, 0.0, 30.0, 30.0,
        150.0, 30.0, // top rung back
        30.0, 0.0, 30.0, 100.0, 0.0, 30.0, 30.0, 30.0, 30.0, 30.0, 30.0, 30.0, 100.0, 0.0, 30.0,
        100.0, 30.0, 30.0, // middle rung back
        30.0, 60.0, 30.0, 67.0, 60.0, 30.0, 30.0, 90.0, 30.0, 30.0, 90.0, 30.0, 67.0, 60.0, 30.0,
        67.0, 90.0, 30.0, // top
        0.0, 0.0, 0.0, 100.0, 0.0, 0.0, 100.0, 0.0, 30.0, 0.0, 0.0, 0.0, 100.0, 0.0, 30.0, 0.0,
        0.0, 30.0, // top rung right
        100.0, 0.0, 0.0, 100.0, 30.0, 0.0, 100.0, 30.0, 30.0, 100.0, 0.0, 0.0, 100.0, 30.0, 30.0,
        100.0, 0.0, 30.0, // under top rung
        30.0, 30.0, 0.0, 30.0, 30.0, 30.0, 100.0, 30.0, 30.0, 30.0, 30.0, 0.0, 100.0, 30.0, 30.0,
        100.0, 30.0, 0.0, // between top rung and middle
        30.0, 30.0, 0.0, 30.0, 60.0, 30.0, 30.0, 30.0, 30.0, 30.0, 30.0, 0.0, 30.0, 60.0, 0.0,
        30.0, 60.0, 30.0, // top of middle rung
        30.0, 60.0, 0.0, 67.0, 60.0, 30.0, 30.0, 60.0, 30.0, 30.0, 60.0, 0.0, 67.0, 60.0, 0.0,
        67.0, 60.0, 30.0, // right of middle rung
        67.0, 60.0, 0.0, 67.0, 90.0, 30.0, 67.0, 60.0, 30.0, 67.0, 60.0, 0.0, 67.0, 90.0, 0.0,
        67.0, 90.0, 30.0, // bottom of middle rung.
        30.0, 90.0, 0.0, 30.0, 90.0, 30.0, 67.0, 90.0, 30.0, 30.0, 90.0, 0.0, 67.0, 90.0, 30.0,
        67.0, 90.0, 0.0, // right of bottom
        30.0, 90.0, 0.0, 30.0, 150.0, 30.0, 30.0, 90.0, 30.0, 30.0, 90.0, 0.0, 30.0, 150.0, 0.0,
        30.0, 150.0, 30.0, // bottom
        0.0, 150.0, 0.0, 0.0, 150.0, 30.0, 30.0, 150.0, 30.0, 0.0, 150.0, 0.0, 30.0, 150.0, 30.0,
        30.0, 150.0, 0.0, // left side
        0.0, 0.0, 0.0, 0.0, 0.0, 30.0, 0.0, 150.0, 30.0, 0.0, 0.0, 0.0, 0.0, 150.0, 30.0, 0.0,
        150.0, 0.0,
    ];
    let array = Float32Array::new_with_length(geometry.len() as u32);
    array.copy_from(&geometry);
    gl.buffer_data_with_array_buffer_view(Gl::ARRAY_BUFFER, &array, Gl::STATIC_DRAW);
}

// fn set_color(gl: &Gl) {
//     let color = [
//         // left column front
//         200, 70, 120, 200, 70, 120, 200, 70, 120, 200, 70, 120, 200, 70, 120, 200, 70, 120,
//         // top rung front
//         200, 70, 120, 200, 70, 120, 200, 70, 120, 200, 70, 120, 200, 70, 120, 200, 70, 120,
//         // middle rung front
//         200, 70, 120, 200, 70, 120, 200, 70, 120, 200, 70, 120, 200, 70, 120, 200, 70, 120,
//         // left column back
//         80, 70, 200, 80, 70, 200, 80, 70, 200, 80, 70, 200, 80, 70, 200, 80, 70, 200,
//         // top rung back
//         80, 70, 200, 80, 70, 200, 80, 70, 200, 80, 70, 200, 80, 70, 200, 80, 70, 200,
//         // middle rung back
//         80, 70, 200, 80, 70, 200, 80, 70, 200, 80, 70, 200, 80, 70, 200, 80, 70, 200,
//         // top
//         70, 200, 210, 70, 200, 210, 70, 200, 210, 70, 200, 210, 70, 200, 210, 70, 200, 210,
//         // top rung right
//         200, 200, 70, 200, 200, 70, 200, 200, 70, 200, 200, 70, 200, 200, 70, 200, 200, 70,
//         // under top rung
//         210, 100, 70, 210, 100, 70, 210, 100, 70, 210, 100, 70, 210, 100, 70, 210, 100, 70,
//         // between top rung and middle
//         210, 160, 70, 210, 160, 70, 210, 160, 70, 210, 160, 70, 210, 160, 70, 210, 160, 70,
//         // top of middle rung
//         70, 180, 210, 70, 180, 210, 70, 180, 210, 70, 180, 210, 70, 180, 210, 70, 180, 210,
//         // right of middle rung
//         100, 70, 210, 100, 70, 210, 100, 70, 210, 100, 70, 210, 100, 70, 210, 100, 70, 210,
//         // bottom of middle rung.
//         76, 210, 100, 76, 210, 100, 76, 210, 100, 76, 210, 100, 76, 210, 100, 76, 210, 100,
//         // right of bottom
//         140, 210, 80, 140, 210, 80, 140, 210, 80, 140, 210, 80, 140, 210, 80, 140, 210, 80,
//         // bottom
//         90, 130, 110, 90, 130, 110, 90, 130, 110, 90, 130, 110, 90, 130, 110, 90, 130, 110,
//         // left side
//         160, 160, 220, 160, 160, 220, 160, 160, 220, 160, 160, 220, 160, 160, 220, 160, 160, 220,
//     ];
//     let array = Uint8Array::new_with_length(color.len() as u32);
//     array.copy_from(&color);
//     gl.buffer_data_with_array_buffer_view(Gl::ARRAY_BUFFER, &array, Gl::STATIC_DRAW);
// }
