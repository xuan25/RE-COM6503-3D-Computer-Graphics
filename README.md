# RE: COM6503 3D Computer Graphics: Assignment - The Museum (ported from JOGL to Rust)

## Features

1. Basic room with two walls and a floor. All of them are textured.
2. A window in the wall. Outside the window is a dynamic landscape, including fluttering snowflakes and day-night cycles.
3. There is a robot (hierarchical model) in the scene. Its pose can be controlled through the control panel. There will be smooth animations between poses. The initial pose of the robot is `pose 1`.
4. There is a mobile phone in the scene with texture mapped to its surface.
5. There is a swinging spotlight in the scene, with its light shining on the floor.
6. There is a large egg in the scene with a specular texture.
7. The scene contains a skylight source, four room-light sources and a spotlight source. The intensity of each light source can be controlled through the control panel, as well as the intensity and colour of the skylight source will also be affected by the day-night cycle.
8. Well-structured control panel.
9. Smooth animations for everything, including the robot poses switching as mentioned above.
10. Two optional skies. One is a box-shaped sky (Skybox), and the other is a spherical sky (Skysphere). The spherical sky is animated. All the skies have a day-night cycle effect.
11. Multisample anti-aliasing (MSAA), Gamma correction, HDR render included.

## How to run

1. Make sure you have the Rust toolchain installed.
2. Make sure GLFW's native development library and an OpenGL-capable display environment are available.
3. Go to the code repository and run the project with `cargo run`.
4. A window showing the museum should appear.
5. It will load all the resources before the scene starts to render. The loading states will be shown in the terminal. (Note: The loading may take a while. 37 textures in total)

## How to run (Alternative)

- Run `cargo run --release` for an optimised build.

## How to use

The left side of the window is the control panel, and the right side is the scene display.

### Control panel

There are five option groups: Camera, Robot pose, Lighting, Sky and Misc.

- Camera: Camera presets
- Robot pose: Control the pose of the robot (Note: The button will have no effect when the robot is moving).
- Lighting: Control the intensity of different light sources in the scene.
- Sky: Used to switch between two kinds of skies, where B is a dynamic sky.
- Misc: Miscellaneous control. Switch between two keymaps for camera movement.

### Scene display

Use the keyboard to control the camera movement, and use the mouse to control the camera rotation.

## Directories

- `legacy` - The original Java/JOGL project retained as the migration reference
- `src/gmaths` - A simplified linear algebra library for 3D computer graphics
- `src/graphics` - General 3D graphics rendering
  - `basic` - Metadata of basic graphics objects
  - `camera` - Camera control
  - `lighting` - Lighting resources management
  - `material` - Material and Texture resources management
  - `node` - Scene-graph related
  - `offscreen` - Off-screen render related (MSAA, Gamma correction, HDR render)
  - `shader` - Shader resources management
- `meshes` - Contains mesh files, which are mainly used to record some complex texture mapping
- `src/scene` - Scene construction and animation
  - `animator` - Animation related
  - `component` - Scene components
- `shaders` - Contains GLSL source code of shaders (`.vert` are vertex shaders; `.frag` are fragment shaders; they are all plain text files)
- `textures` - Texture image resources
