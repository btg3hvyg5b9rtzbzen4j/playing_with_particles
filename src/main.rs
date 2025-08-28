use macroquad::prelude::*;

const G: f32 = 6.67430e-11;
const C: f32 = 299_792_458.0;
const CAMERA_SENSITIVITY: f32 = 100.0;
const SCALE: f32 = 50_000_000.;

fn conf() -> Conf {
    Conf {
        window_title: String::from("playing_with_particles"),
        window_width: 1260,
        window_height: 768,
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let dt = get_frame_time();

    let mut pos = vec3(0., 100., 0.);
    let mut yaw: f32 = 0.0;
    let mut pitch: f32 = 0.0;

    set_cursor_grab(true);
    show_mouse(false);

    let mut sun = Body {
        color: YELLOW,
        position: vec3(0., 0., 0.),
        mass: 1.989e30,
        radius: 696_340_000.0,
        velocity: vec3(0., 0., 0.),
    };

    let mut earth = Body {
        color: BLUE,
        position: vec3(149_597_870_000.0, 0., 0.),
        mass: 5.972e24,
        radius: 6_371_000.0,
        velocity: vec3(0., 0., 0.),
    };

    let mut movement_speed: f32 = C * 20. / SCALE;

    loop {
        let dt = get_frame_time();

        clear_background(BLACK);

        let mouse_delta = mouse_delta_position();

        yaw += mouse_delta.x * CAMERA_SENSITIVITY * dt;
        pitch += mouse_delta.y * CAMERA_SENSITIVITY * 0.6 * dt;

        // Convert yaw & pitch to a direction vector
        let look = vec3(
            yaw.sin() * pitch.cos(),
            pitch.sin(),
            yaw.cos() * pitch.cos(),
        );

        // Get the right and forward vector relative to the world up vector (no idea what this means i need a visualization)
        let world_up = vec3(0., 1., 0.);
        let right = look.cross(world_up).normalize();
        let forward = vec3(look.x, 0., look.z).normalize();

        // WASD movement
        if is_key_down(KeyCode::W) {
            pos += forward * movement_speed * dt;
        }
        if is_key_down(KeyCode::A) {
            pos -= right * movement_speed * dt;
        }
        if is_key_down(KeyCode::S) {
            pos -= forward * movement_speed * dt;
        }
        if is_key_down(KeyCode::D) {
            pos += right * movement_speed * dt;
        }

        // SPACE & SHIFT up/down
        if is_key_down(KeyCode::Space) {
            pos.y += movement_speed * dt;
        }
        if is_key_down(KeyCode::LeftControl) {
            pos.y -= movement_speed * dt;
        }

        // SLOW movement
        if is_key_down(KeyCode::LeftShift) {
            movement_speed = C / SCALE;
        } else {
            movement_speed = C * 20. / SCALE;
        }

        // ESC quitting
        if is_key_down(KeyCode::Escape) {
            break;
        }

        set_camera(&Camera3D {
            position: pos,
            up: vec3(0., 1., 0.),
            target: pos + look,
            ..Default::default()
        });

        sun.update(dt);
        sun.draw();
        earth.draw();

        next_frame().await
    }
}

struct Body {
    position: Vec3,
    velocity: Vec3,
    mass: f32,
    radius: f32,
    color: Color,
}

impl Body {
    fn draw(&mut self) {
        draw_sphere_wires(self.position / SCALE, self.radius / SCALE, None, self.color);
    }

    fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
    }
}
