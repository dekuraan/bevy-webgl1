attribute vec4 a_position;

uniform vec4 u_color;
uniform mat4 u_camera;
uniform mat4 u_model;

varying vec4 v_color;

void main() {
  // Multiply the position by the matrix.
  gl_Position = u_camera * (u_model * a_position);
  // Pass the color to the fragment shader.
  v_color = u_color;
}