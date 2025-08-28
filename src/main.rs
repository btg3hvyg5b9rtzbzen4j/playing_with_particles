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

// im funny (what the f8**)
fn radiants_to_deg(x: f32) -> f32 {
    return x * 180.0 / std::f32::consts::PI;
}

// im funny 2 (what the f9**)
fn deg_to_radiants(x: f32) -> f32 {
    return x * std::f32::consts::PI / 180.0;
}

#[macroquad::main(conf)]
async fn main() {
    let mut pos = vec3(-130., 0., -50.);
    let mut yaw: f32 = 1.5;
    let mut pitch: f32 = 0.0;
    let mut movement_speed: f32 = 1000.;
    let mut go_slow = false;
    let mut time_scale: f32 = 1.;
    let mut body_scale: f32 = 1.;
    let mut grabbed = false;

    set_cursor_grab(grabbed);
    show_mouse(!grabbed);

    let mut bodies: Vec<Body> = vec![
        // Sun
        Body {
            color: Color::from_rgba(0xFF, 0xD7, 0x00, 255), // gold
            position: vec3(0., 0., 0.),
            mass: 1.989e30,
            radius: 696_340_000.0,
            velocity: vec3(0., 0., 0.),
        },
        // Mercury
        Body {
            color: Color::from_rgba(0x91, 0x91, 0x91, 255), // gray
            position: vec3(57_910_000_000.0, 0., 0.),
            mass: 3.301e23,
            radius: 2_439_700.0,
            velocity: vec3(0., 0., 47_360.0),
        },
        // Venus
        Body {
            color: Color::from_rgba(0xFF, 0xA5, 0x00, 255), // orange
            position: vec3(108_200_000_000.0, 0., 0.),
            mass: 4.867e24,
            radius: 6_052_000.0,
            velocity: vec3(0., 0., 35_020.0),
        },
        // Earth
        Body {
            color: Color::from_rgba(0x00, 0x00, 0xFF, 255), // blue
            position: vec3(149_597_870_000.0, 0., 0.),
            mass: 5.972e24,
            radius: 6_371_000.0,
            velocity: vec3(0., 0., 29_780.0),
        },
        // Mars
        Body {
            color: Color::from_rgba(0xFF, 0x00, 0x00, 255), // red
            position: vec3(227_940_000_000.0, 0., 0.),
            mass: 6.417e23,
            radius: 3_390_000.0,
            velocity: vec3(0., 0., 24_070.0),
        },
        // Jupiter
        Body {
            color: Color::from_rgba(0xFF, 0xA5, 0x00, 255), // orange
            position: vec3(778_330_000_000.0, 0., 0.),
            mass: 1.898e27,
            radius: 69_911_000.0,
            velocity: vec3(0., 0., 13_070.0),
        },
        // Saturn
        Body {
            color: Color::from_rgba(0xFF, 0xFF, 0x99, 255), // pale yellow
            position: vec3(1_429_400_000_000.0, 0., 0.),
            mass: 5.683e26,
            radius: 58_232_000.0,
            velocity: vec3(0., 0., 9_680.0),
        },
        // Uranus
        Body {
            color: Color::from_rgba(0x00, 0xFF, 0xFF, 255), // cyan
            position: vec3(2_870_990_000_000.0, 0., 0.),
            mass: 8.681e25,
            radius: 25_362_000.0,
            velocity: vec3(0., 0., 6_800.0),
        },
        // Neptune
        Body {
            color: Color::from_rgba(0x00, 0x00, 0xFF, 255), // blue
            position: vec3(4_504_000_000_000.0, 0., 0.),
            mass: 1.024e26,
            radius: 24_622_000.0,
            velocity: vec3(0., 0., 5_430.0),
        },
        // Pluto
        Body {
            color: Color::from_rgba(0xFF, 0xFF, 0xFF, 255), // white
            position: vec3(5_906_400_000_000.0, 0., 0.),
            mass: 1.309e22,
            radius: 1_188_300.0,
            velocity: vec3(0., 0., 4_740.0),
        },
    ];

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

        if is_key_down(KeyCode::Delete) {
            break;
        }

        // mouse grab
        if is_key_pressed(KeyCode::Escape) {
            grabbed = !grabbed;
            set_cursor_grab(grabbed);
            show_mouse(!grabbed);
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
                if i == 0 {
                    body.radius / SCALE
                } else {
                    body.radius / SCALE * body_scale
                },
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
            format!("Yaw & Pitch: [{:.2}, {:.2}]", radiants_to_deg(yaw), radiants_to_deg(pitch)).as_str(),
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
        draw_text(
            "Press [ESCAPE] to lock/unlock mouse!",
            20.0,
            screen_height() - 20.0,
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
