use wasm_bindgen::JsCast;
use web_sys::{
    Document, HtmlCanvasElement, OesVertexArrayObject, WebGlBuffer, WebGlProgram,
    WebGlRenderingContext as Gl, WebGlShader, Window,
};

pub fn compile_shader(context: &Gl, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, Gl::COMPILE_STATUS)
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

pub fn link_program(
    context: &Gl,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, Gl::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

pub fn window() -> Window {
    web_sys::window().expect("Has window")
}

pub fn document() -> Document {
    window().document().expect("Has document")
}

pub fn canvas(id: &str) -> HtmlCanvasElement {
    document()
        .get_element_by_id(id)
        .expect("canvas element does not exist")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("is HtmlCanvasElement")
}

pub fn get_gl(canvas_id: &str) -> Gl {
    canvas(canvas_id)
        .get_context("webgl")
        .unwrap()
        .unwrap()
        .dyn_into::<Gl>()
        .unwrap()
}

pub fn get_oes_vao(context: &Gl) -> OesVertexArrayObject {
    context
        .get_extension("OES_vertex_array_object")
        .expect("Get OES vao ext")
        .expect("OES vao ext")
        .unchecked_into::<OesVertexArrayObject>()
}

// pub fn render_mesh(context: &Gl, mesh: &Mesh) {
//     let oes_vao = get_oes_vao(context);
//     let vao = oes_vao.create_vertex_array_oes().unwrap();
//     oes_vao.bind_vertex_array_oes(Some(&vao));
//     context.vertex_attrib_pointer_with_i32(0, 3, Gl::FLOAT, false, 0, 0);
//     context.enable_vertex_attrib_array(position_attribute_location as u32);
//     context.draw_arrays(Gl::TRIANGLES, 0);
// }

pub fn bind_array_buffer(gl: &Gl) -> WebGlBuffer {
    let buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&buffer));
    buffer
}
