use macroquad::{
    prelude::*,
    ui::{hash, root_ui},
};

const G: f32 = 6.67430e-11;
const C: f32 = 299_792_458.0;
const CAMERA_SENSITIVITY: f32 = 100.0;
const SCALE: f32 = 50_000_000.;
const MAX_PITCH: f32 = std::f32::consts::FRAC_PI_2 - 0.01;
const MAX_TIME: f32 = 31_536_000.;

fn conf() -> Conf {
    Conf {
        window_title: String::from("playing_with_particles"),
        window_width: 1260,
        window_height: 768,
        ..Default::default()
    }
}

fn random_chaotic_bodies(c: u32) -> Vec<Body> {
    let mut bodies = Vec::new();

    for _ in 0..c {
        let mass = rand::gen_range(1.0e30, 2.0e30); // Solar-mass stars
        let radius = rand::gen_range(6.0e8, 7.0e8); // Sun-like radius
        let pos = vec3(
            rand::gen_range(-2.0e11, 2.0e11),
            rand::gen_range(-2.0e11, 2.0e11),
            rand::gen_range(-1.0e10, 1.0e10),
        );
        let vel = vec3(
            rand::gen_range(-3.0e4, 3.0e4), // ~tens of km/s
            rand::gen_range(-3.0e4, 3.0e4),
            rand::gen_range(-3.0e4, 3.0e4),
        );
        let color = Color::from_rgba(
            rand::gen_range(50, 255),
            rand::gen_range(50, 255),
            rand::gen_range(50, 255),
            255,
        );

        bodies.push(Body {
            position: pos,
            velocity: vel,
            mass,
            radius,
            color,
        });
    }

    bodies
}

#[macroquad::main(conf)]
async fn main() {
    let mut pos = vec3(-20000., 0., -50.);
    let mut yaw: f32 = 1.5;
    let mut pitch: f32 = 0.0;
    let mut movement_speed: f32 = 5000.;
    let mut go_slow = false;
    let mut time_scale: f32 = 2000000.;
    let mut body_scale: f32 = 15.;
    let mut grabbed = false;

    set_cursor_grab(grabbed);
    show_mouse(!grabbed);

    let mut bodies = random_chaotic_bodies(3);

    loop {
        let dt = get_frame_time();
        let mouse_delta = mouse_delta_position();

        if grabbed {
            yaw += mouse_delta.x * CAMERA_SENSITIVITY * dt;
            pitch += mouse_delta.y * 0.7 * CAMERA_SENSITIVITY * dt;
            pitch = pitch.clamp(-MAX_PITCH, MAX_PITCH);
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

        // quitting
        if is_key_down(KeyCode::Delete) {
            break;
        }

        // mouse grab
        if is_key_pressed(KeyCode::Escape) {
            grabbed = !grabbed;
            set_cursor_grab(grabbed);
            show_mouse(!grabbed);
        }

        // regenerate
        if is_key_pressed(KeyCode::R) {
            bodies = random_chaotic_bodies(3);
        }

        // Scene begin
        set_camera(&Camera3D {
            position: pos,
            up: vec3(0., 1., 0.),
            target: pos + look,
            z_far: 1_000_000.,
            ..Default::default()
        });

        clear_background(BLACK);

        // TODO: put this into a separate thread
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

            draw_sphere_wires(
                body.position / SCALE,
                body.radius / SCALE * body_scale,
                None,
                body.color,
            );
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
            60.0,
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
            80.0,
            20.0,
            WHITE,
        );

        draw_text(
            "Press [ESCAPE] to lock/unlock mouse!",
            20.0,
            screen_height() - 20.0,
            20.0,
            WHITE,
        );
        draw_text(
            "Press [R] to generate new bodies!",
            20.0,
            screen_height() - 40.0,
            20.0,
            WHITE,
        );

        root_ui().window(
            hash!(),
            vec2(screen_width() - 20. - 400., 20.),
            vec2(400., 75.),
            |ui| {
                ui.slider(hash!(), "Time Scale", 1.0..MAX_TIME, &mut time_scale);
                ui.slider(hash!(), "Body Scale", 1.0..1000., &mut body_scale);
                ui.slider(
                    hash!(),
                    "Movement Speed (C)",
                    1.0..10000.,
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
