use bevy::prelude::*;
use bevy_webgl1::util::{bind_array_buffer, compile_shader, get_gl, link_program};
use js_sys::{Float32Array, Uint16Array};
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlProgram, WebGlRenderingContext as Gl};
fn main() -> Result<(), String> {
    console_error_panic_hook::set_once();
    //Setup
    let gl = get_gl("canvas");
    let vert_shader = compile_shader(&gl, Gl::VERTEX_SHADER, include_str!("shaders/mesh.vert"))?;
    let frag_shader = compile_shader(&gl, Gl::FRAGMENT_SHADER, include_str!("shaders/mesh.frag"))?;
    let program = link_program(&gl, &vert_shader, &frag_shader)?;
    gl.use_program(Some(&program));

    //Get locations
    let position_location = gl.get_attrib_location(&program, "a_position");
    let camera_location = gl.get_uniform_location(&program, "u_camera").unwrap();
    let model_location = gl.get_uniform_location(&program, "u_model").unwrap();
    //draw scene
    let canvas = gl.canvas().unwrap().unchecked_into::<HtmlCanvasElement>();
    let width = canvas.width();
    let height = canvas.height();

    gl.viewport(0, 0, width as i32, height as i32);
    {
        //clear canvas
        gl.clear(Gl::COLOR_BUFFER_BIT | Gl::DEPTH_BUFFER_BIT);
        gl.enable(Gl::CULL_FACE);
        gl.enable(Gl::DEPTH_TEST);
        gl.use_program(Some(&program));
        //fill position buffer
        {
            let position_buffer = bind_array_buffer(&gl);
            //Do positions
            gl.enable_vertex_attrib_array(position_location as u32);
            gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&position_buffer));
            gl.vertex_attrib_pointer_with_i32(position_location as u32, 3, Gl::FLOAT, false, 0, 0);
            //set geometry
            let shape = shape::Icosphere {
                radius: 150.0,
                subdivisions: 2,
            };
            let mesh = Mesh::from(shape);
            let positions = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
            let positions = match positions {
                bevy::render::mesh::VertexAttributeValues::Float32x3(ps) => ps,
                _ => panic!("positions not f32x3"),
            };
            let positions = positions
                .iter()
                .map(|xyz| xyz.iter().map(|f| *f))
                .flatten()
                .collect::<Vec<f32>>();
            let array = Float32Array::new_with_length(positions.len() as u32);
            array.copy_from(&positions);
            gl.buffer_data_with_array_buffer_view(Gl::ARRAY_BUFFER, &array, Gl::STATIC_DRAW);
            let indices = match mesh.indices().unwrap() {
                bevy::render::mesh::Indices::U16(is) => is.clone(),
                bevy::render::mesh::Indices::U32(is) => {
                    is.iter().map(|long| *long as u16).collect()
                }
            };
            let index_buffer = gl.create_buffer();
            let array = Uint16Array::new_with_length(indices.len() as u32);
            array.copy_from(&indices);
            gl.bind_buffer(Gl::ELEMENT_ARRAY_BUFFER, index_buffer.as_ref());
            gl.buffer_data_with_array_buffer_view(
                Gl::ELEMENT_ARRAY_BUFFER,
                &array,
                Gl::STATIC_DRAW,
            );
            // set model uniform
            let transform = Transform::from_translation([0.0, 0.0, 0.0].into());
            let matrix = transform.compute_matrix();
            gl.uniform_matrix4fv_with_f32_array(
                Some(&model_location),
                false,
                &matrix.to_cols_array(),
            );
            // set up camera uniform
            let aspect = (width as f32) / (height as f32);
            let z_near = 1.0;
            let z_far = 2000.0;
            let matrix = Mat4::perspective_rh_gl(60.0_f32.to_radians(), aspect, z_near, z_far);
            let matrix = matrix * Mat4::from_translation([-150.0, 0.0, -360.0].into());
            gl.uniform_matrix4fv_with_f32_array(
                Some(&camera_location),
                false,
                &matrix.to_cols_array(),
            );
            //set mesh color uniform
            gl.draw_elements_with_i32(Gl::TRIANGLES, indices.len() as i32, Gl::UNSIGNED_SHORT, 0);
        }
    }
    Ok(())
}

fn draw_meshes_system(
    meshes: Query<(&GlobalTransform, &Handle<Mesh>)>,
    mesh_storage: Assets<Mesh>,
    program: NonSend<WebGlProgram>,
) {
    let gl = get_gl("bevy");
    //Get locations
    let position_location = gl.get_attrib_location(&program, "a_position");
    let camera_location = gl.get_uniform_location(&program, "u_camera").unwrap();
    let model_location = gl.get_uniform_location(&program, "u_model").unwrap();
    //draw scene
    let canvas = gl.canvas().unwrap().unchecked_into::<HtmlCanvasElement>();
    let width = canvas.width();
    let height = canvas.height();
    gl.viewport(0, 0, width as i32, height as i32);
    //clear canvas
    gl.clear(Gl::COLOR_BUFFER_BIT | Gl::DEPTH_BUFFER_BIT);
    gl.enable(Gl::CULL_FACE);
    gl.enable(Gl::DEPTH_TEST);
    gl.use_program(Some(&program));
    //draw_meshes
    for (gtf, mesh) in meshes.iter() {
        let position_buffer = bind_array_buffer(&gl);
        //Do positions
        gl.enable_vertex_attrib_array(position_location as u32);
        gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&position_buffer));
        gl.vertex_attrib_pointer_with_i32(position_location as u32, 3, Gl::FLOAT, false, 0, 0);
        //set mesh buffers

        let indices_len = {
            let mesh = mesh_storage.get(mesh).unwrap();
            //positions
            {
                let positions = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
                let positions = match positions {
                    bevy::render::mesh::VertexAttributeValues::Float32x3(ps) => ps,
                    _ => panic!("positions not f32x3"),
                };
                let positions = positions
                    .iter()
                    .map(|xyz| xyz.iter().map(|f| *f))
                    .flatten()
                    .collect::<Vec<f32>>();
                let array = Float32Array::new_with_length(positions.len() as u32);
                array.copy_from(&positions);
                gl.buffer_data_with_array_buffer_view(Gl::ARRAY_BUFFER, &array, Gl::STATIC_DRAW);
            }
            //indices
            {
                let indices = match mesh.indices().unwrap() {
                    bevy::render::mesh::Indices::U16(is) => is.clone(),
                    bevy::render::mesh::Indices::U32(is) => {
                        is.iter().map(|long| *long as u16).collect()
                    }
                };
                let index_buffer = gl.create_buffer();
                let array = Uint16Array::new_with_length(indices.len() as u32);
                array.copy_from(&indices);
                gl.bind_buffer(Gl::ELEMENT_ARRAY_BUFFER, index_buffer.as_ref());
                gl.buffer_data_with_array_buffer_view(
                    Gl::ELEMENT_ARRAY_BUFFER,
                    &array,
                    Gl::STATIC_DRAW,
                );
                indices.len()
            }
        };
        // set model uniform
        {
            let transform = Transform::from_translation([0.0, 0.0, 0.0].into());
            let matrix = transform.compute_matrix();
            gl.uniform_matrix4fv_with_f32_array(
                Some(&model_location),
                false,
                &matrix.to_cols_array(),
            );
        }
        // set up camera uniform
        {
            let aspect = (width as f32) / (height as f32);
            let z_near = 1.0;
            let z_far = 2000.0;
            let matrix = Mat4::perspective_rh_gl(60.0_f32.to_radians(), aspect, z_near, z_far);
            let matrix = matrix * Mat4::from_translation([-150.0, 0.0, -360.0].into());
            gl.uniform_matrix4fv_with_f32_array(
                Some(&camera_location),
                false,
                &matrix.to_cols_array(),
            );
        }
        //set mesh color uniform
        gl.draw_elements_with_i32(Gl::TRIANGLES, indices_len as i32, Gl::UNSIGNED_SHORT, 0);
    }
}
