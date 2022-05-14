use bevy::{prelude::*, render::options::WgpuOptions};
use js_sys::{Float32Array, Uint16Array, Uint8Array};
use util::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlCanvasElement, WebGlBuffer, WebGlProgram, WebGlRenderingContext as Gl};
pub mod util;

pub struct Webgl1RenderingPlugin;
impl Plugin for Webgl1RenderingPlugin {
    fn build(&self, app: &mut App) {
        let gl = get_gl("bevy");
        let vert_shader = compile_shader(&gl, Gl::VERTEX_SHADER, include_str!("shaders/mesh.vert"))
            .expect("got vert shader");
        let frag_shader =
            compile_shader(&gl, Gl::FRAGMENT_SHADER, include_str!("shaders/mesh.frag"))
                .expect("got frag shader");
        let program =
            link_program(&gl, &vert_shader, &frag_shader).expect("could create webgl program");
        gl.use_program(Some(&program));
        gl.enable(Gl::CULL_FACE);
        gl.enable(Gl::DEPTH_TEST);
        let position_buffer = gl.create_buffer().unwrap();
        let uv_buffer = gl.create_buffer().unwrap();
        let index_buffer = gl.create_buffer().unwrap();
        app.insert_non_send_resource(Buffers {
            position_buffer,
            uv_buffer,
            index_buffer,
        });
        app.insert_resource(WindowDescriptor {
            canvas: Some("#bevy".to_string()),
            ..Default::default()
        })
        .insert_resource(WgpuOptions {
            backends: None,
            ..Default::default()
        })
        .insert_non_send_resource(program)
        .add_system(draw_meshes_system);
    }
}
struct Buffers {
    position_buffer: WebGlBuffer,
    uv_buffer: WebGlBuffer,
    index_buffer: WebGlBuffer,
}

fn draw_meshes_system(
    meshes: Query<(&GlobalTransform, &Handle<Mesh>, &Handle<StandardMaterial>)>,
    perspective_cameras: Query<(&GlobalTransform, &PerspectiveProjection)>,
    mesh_storage: Res<Assets<Mesh>>,
    material_storage: Res<Assets<StandardMaterial>>,
    image_storage: Res<Assets<Image>>,
    program: NonSend<WebGlProgram>,
    buffers: NonSend<Buffers>,
) {
    let gl = get_gl("bevy");
    //Get locations
    let position_location = gl.get_attrib_location(&program, "a_position");
    let uv_location = gl.get_attrib_location(&program, "a_texcoord");
    let camera_location = gl.get_uniform_location(&program, "u_camera").unwrap();
    let model_location = gl.get_uniform_location(&program, "u_model").unwrap();
    //draw scene
    let canvas = gl.canvas().unwrap().unchecked_into::<HtmlCanvasElement>();
    let width = canvas.width();
    let height = canvas.height();
    gl.viewport(0, 0, width as i32, height as i32);
    //clear canvas
    gl.clear(Gl::COLOR_BUFFER_BIT | Gl::DEPTH_BUFFER_BIT);

    gl.use_program(Some(&program));
    // set up camera uniform
    {
        let (transform, perspective) = perspective_cameras.single();
        let aspect = (width as f32) / (height as f32);
        let z_near = perspective.near;
        let z_far = perspective.far;
        let matrix = Mat4::perspective_rh_gl(perspective.fov, aspect, z_near, z_far);
        let matrix = matrix * transform.compute_matrix().inverse();
        gl.uniform_matrix4fv_with_f32_array(Some(&camera_location), false, &matrix.to_cols_array());
    }
    //draw_meshes
    //TODO (perf) draw meshes instanced

    for (mesh_gtf, mesh, material) in meshes.iter() {
        //set up texture
        {
            let image = if let Some(material) = material_storage.get(material) {
                if let Some(image) =
                    image_storage.get(material.base_color_texture.as_ref().unwrap())
                {
                    image
                } else {
                    continue;
                }
            } else {
                continue;
            };
            let width = image.texture_descriptor.size.width;
            let height = image.texture_descriptor.size.height;
            let texture = gl.create_texture().unwrap();
            gl.bind_texture(Gl::TEXTURE_2D, Some(&texture));
            let array = Uint8Array::new_with_length(image.data.len() as u32);
            array.copy_from(&image.data);
            gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                Gl::TEXTURE_2D,
                0,
                Gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                Gl::RGBA,
                Gl::UNSIGNED_BYTE,
                Some(&array),
            )
            .expect("did da texture");
            gl.generate_mipmap(Gl::TEXTURE_2D);
        }
        //Do mesh
        let indices_len = {
            let mesh = if let Some(mesh) = mesh_storage.get(mesh) {
                mesh
            } else {
                continue;
            };
            //positions
            {
                let position_buffer = &buffers.position_buffer;
                gl.enable_vertex_attrib_array(position_location as u32);
                gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&position_buffer));
                gl.vertex_attrib_pointer_with_i32(
                    position_location as u32,
                    3,
                    Gl::FLOAT,
                    false,
                    0,
                    0,
                );

                //TODO (perf) use unsafe array copy
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
            //UVs
            {
                let uv_buffer = &buffers.uv_buffer;
                gl.enable_vertex_attrib_array(uv_location as u32);
                gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&uv_buffer));
                gl.vertex_attrib_pointer_with_i32(uv_location as u32, 2, Gl::FLOAT, false, 0, 0);
                let uvs = mesh.attribute(Mesh::ATTRIBUTE_UV_0).unwrap();
                let uvs = match uvs {
                    bevy::render::mesh::VertexAttributeValues::Float32x2(ps) => ps,
                    _ => panic!("uvs not f32x2"),
                };
                let uvs = uvs
                    .iter()
                    .map(|xyz| xyz.iter().map(|f| *f))
                    .flatten()
                    .collect::<Vec<f32>>();
                let array = Float32Array::new_with_length(uvs.len() as u32);
                array.copy_from(&uvs);

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
                let index_buffer = &buffers.index_buffer;
                let array = Uint16Array::new_with_length(indices.len() as u32);
                array.copy_from(&indices);
                gl.bind_buffer(Gl::ELEMENT_ARRAY_BUFFER, Some(index_buffer));
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
            let transform = mesh_gtf;
            let matrix = transform.compute_matrix();
            gl.uniform_matrix4fv_with_f32_array(
                Some(&model_location),
                false,
                &matrix.to_cols_array(),
            );
        }
        //draw
        gl.draw_elements_with_i32(Gl::TRIANGLES, indices_len as i32, Gl::UNSIGNED_SHORT, 0);
    }
}
