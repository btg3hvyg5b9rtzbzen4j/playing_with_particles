use macroquad::{
    prelude::*,
    ui::{
        self, hash, root_ui,
        widgets::{self, Slider},
    },
};

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
    let mut pos = vec3(-130., 0., -50.);
    let mut yaw: f32 = 1.5;
    let mut pitch: f32 = 0.0;
    let mut movement_speed: f32 = 100.;
    let mut go_slow = false;
    let mut time_scale: f32 = 1.;
    let mut body_scale: f32 = 1.;
    let mut grabbed = false;

    set_cursor_grab(grabbed);
    show_mouse(!grabbed);

    let mut bodies: Vec<Body> = vec![
        Body {
            color: YELLOW,
            position: vec3(0., 0., 0.),
            mass: 1.989e30,
            radius: 696_340_000.0,
            velocity: vec3(0., 0., 0.),
        },
        Body {
            color: BLUE,
            position: vec3(149_597_870_000.0, 0., 0.),
            mass: 5.972e24,
            radius: 6_371_000.0,
            velocity: vec3(0., 0., 29_780.),
        },
        Body {
            color: WHITE,
            position: vec3(57_910_000_000.0, 0., 0.),
            mass: 3.301e23,
            radius: 2_439_700.0,
            velocity: vec3(0., 0., 47_360.),
        },
    ];

    loop {
        let dt = get_frame_time();

        clear_background(BLACK);

        let mouse_delta = mouse_delta_position();

        if grabbed {
            yaw += mouse_delta.x * CAMERA_SENSITIVITY * dt;
            pitch += mouse_delta.y * CAMERA_SENSITIVITY * 0.6 * dt;
        }

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
        let new_speed = if go_slow {
            C / SCALE
        } else {
            C * movement_speed / SCALE
        };

        if is_key_down(KeyCode::W) {
            pos += forward * new_speed * dt;
        }
        if is_key_down(KeyCode::A) {
            pos -= right * new_speed * dt;
        }
        if is_key_down(KeyCode::S) {
            pos -= forward * new_speed * dt;
        }
        if is_key_down(KeyCode::D) {
            pos += right * new_speed * dt;
        }

        // SPACE & SHIFT up/down
        if is_key_down(KeyCode::Space) {
            pos.y += new_speed * dt;
        }
        if is_key_down(KeyCode::LeftControl) {
            pos.y -= new_speed * dt;
        }

        // SLOW movement
        if is_key_down(KeyCode::LeftShift) {
            go_slow = true;
        } else {
            go_slow = false;
        }

        // ESC quitting
        if is_key_down(KeyCode::Escape) {
            break;
        }

        // mouse grab
        if is_key_pressed(KeyCode::LeftAlt) {
            grabbed = !grabbed;
            set_cursor_grab(grabbed);
            show_mouse(!grabbed);
        }

        // Scene begin
        set_camera(&Camera3D {
            position: pos,
            up: vec3(0., 1., 0.),
            target: pos + look,
            ..Default::default()
        });

        let n = bodies.len();

        // N-body gravity calculation, we apply the gravity of each body to each body, calculate their accelerations, and later add them
        let mut accelerations: Vec<Vec3> = vec![vec3(0., 0., 0.); bodies.len()];

        for (i, body) in bodies.iter().enumerate() {
            let mut acc = vec3(0., 0., 0.);
            for (j, other) in bodies.iter().enumerate() {
                if i == j {
                    continue;
                }
                let r_vec = other.position - body.position;
                let distance = r_vec.length();
                acc += (r_vec / distance) * G * other.mass / (distance * distance);
            }
            accelerations[i] = acc;
        }

        for (i, body) in bodies.iter_mut().enumerate() {
            let scaled_time = dt * time_scale;
            body.velocity += accelerations[i] * scaled_time;
            body.position += body.velocity * scaled_time;
            body.draw(body_scale);
        }

        // ui rendering
        set_default_camera();

        draw_text(
            format!("Position: {:.2}", pos).as_str(),
            20.0,
            40.0,
            20.0,
            WHITE,
        );
        draw_text(
            format!("Yaw & Pitch: [{:.2}, {:.2}]", yaw, pitch).as_str(),
            20.0,
            70.0,
            20.0,
            WHITE,
        );
        draw_text(
            format!(
                "Movement Speed: [{:.2} m/s] [{:.2}C]",
                new_speed * SCALE,
                if go_slow { 1.0 } else { movement_speed }
            )
            .as_str(),
            20.0,
            100.0,
            20.0,
            WHITE,
        );

        root_ui().window(
            hash!(),
            vec2(screen_width() - 20. - 400., 20.),
            vec2(400., 75.),
            |ui| {
                ui.slider(hash!(), "Time Scale", 1.0..831_536_000., &mut time_scale);
                ui.slider(hash!(), "Body Scale", 1.0..100., &mut body_scale);
                ui.slider(
                    hash!(),
                    "Movement Speed (C)",
                    1.0..100.,
                    &mut movement_speed,
                );
            },
        );

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
    fn draw(&mut self, scale: f32) {
        draw_sphere_wires(
            self.position / SCALE,
            self.radius / SCALE * scale,
            None,
            self.color,
        );
    }
}
