attribute vec4 a_position;
attribute vec2 a_texcoord;

uniform mat4 u_camera;
uniform mat4 u_model;

varying vec2 v_texcoord;

void main() {
  vec4 world_position = u_model * a_position;
  gl_Position = u_camera * world_position;
  // set UV
  v_texcoord = a_texcoord;
}