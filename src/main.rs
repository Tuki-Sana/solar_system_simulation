use nalgebra as na;
use nannou::prelude::*;

#[derive(Debug)]
struct Planet {
    name: String,
    mass: f64,
    position: na::Vector3<f64>,
    velocity: na::Vector3<f64>,
    color: Rgb<u8>,
}

impl Planet {
    fn new(
        name: &str,
        mass: f64,
        position: na::Vector3<f64>,
        velocity: na::Vector3<f64>,
        color: Rgb<u8>,
    ) -> Self {
        Self {
            name: name.to_string(),
            mass,
            position,
            velocity,
            color,
        }
    }
}

struct SolarSystem {
    planets: Vec<Planet>,
    gravitational_constant: f64,
}

impl SolarSystem {
    fn new() -> Self {
        Self {
            planets: Vec::new(),
            gravitational_constant: 6.67430e-11, // m^3 kg^-1 s^-2
        }
    }

    fn add_planet(&mut self, planet: Planet) {
        self.planets.push(planet);
    }

    fn compute_gravitational_forces(&self) -> Vec<na::Vector3<f64>> {
        let mut forces = vec![na::Vector3::zeros(); self.planets.len()];

        for i in 0..self.planets.len() {
            for j in 0..self.planets.len() {
                if i != j {
                    let direction = self.planets[j].position - self.planets[i].position;
                    let distance = direction.magnitude();
                    let force_magnitude =
                        self.gravitational_constant * self.planets[i].mass * self.planets[j].mass
                            / distance.powi(2);
                    let force = direction.normalize() * force_magnitude;
                    forces[i] += force;
                }
            }
        }

        forces
    }

    fn update_positions(&mut self, dt: f64) {
        let forces = self.compute_gravitational_forces();

        for (i, planet) in self.planets.iter_mut().enumerate() {
            let acceleration = forces[i] / planet.mass;
            planet.velocity += acceleration * dt;
            planet.position += planet.velocity * dt;
        }
    }
}

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    solar_system: SolarSystem,
    speed_multiplier: f64,
    scale_factor: f64,
}

fn model(app: &App) -> Model {
    app.new_window()
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let mut solar_system = SolarSystem::new();

    // 太陽の初期位置と速度は0に設定
    let sun = Planet::new(
        "Sun",
        1.989e30,
        na::Vector3::new(0.0, 0.0, 0.0),
        na::Vector3::new(0.0, 0.0, 0.0),
        ORANGE,
    );
    solar_system.add_planet(sun);

    // 各惑星のデータを追加
    let mercury = Planet::new(
        "Mercury",
        3.3011e23,
        na::Vector3::new(5.79e10, 0.0, 0.0),
        na::Vector3::new(0.0, 47.87e3, 0.0),
        rgb(169, 169, 169),
    );
    solar_system.add_planet(mercury);

    let venus = Planet::new(
        "Venus",
        4.8675e24,
        na::Vector3::new(1.082e11, 0.0, 0.0),
        na::Vector3::new(0.0, 35.02e3, 0.0),
        rgb(255, 204, 153),
    );
    solar_system.add_planet(venus);

    let earth = Planet::new(
        "Earth",
        5.972e24,
        na::Vector3::new(1.496e11, 0.0, 0.0),
        na::Vector3::new(0.0, 29.78e3, 0.0),
        BLUE,
    );
    solar_system.add_planet(earth);

    let mars = Planet::new(
        "Mars",
        6.417e23,
        na::Vector3::new(2.279e11, 0.0, 0.0),
        na::Vector3::new(0.0, 24.07e3, 0.0),
        RED,
    );
    solar_system.add_planet(mars);

    let jupiter = Planet::new(
        "Jupiter",
        1.898e27,
        na::Vector3::new(7.785e11, 0.0, 0.0),
        na::Vector3::new(0.0, 13.07e3, 0.0),
        rgb(255, 165, 0),
    );
    solar_system.add_planet(jupiter);

    let saturn = Planet::new(
        "Saturn",
        5.683e26,
        na::Vector3::new(1.429e12, 0.0, 0.0),
        na::Vector3::new(0.0, 9.68e3, 0.0),
        rgb(255, 223, 186),
    );
    solar_system.add_planet(saturn);

    let uranus = Planet::new(
        "Uranus",
        8.681e25,
        na::Vector3::new(2.871e12, 0.0, 0.0),
        na::Vector3::new(0.0, 6.80e3, 0.0),
        rgb(173, 216, 230),
    );
    solar_system.add_planet(uranus);

    let neptune = Planet::new(
        "Neptune",
        1.024e26,
        na::Vector3::new(4.495e12, 0.0, 0.0),
        na::Vector3::new(0.0, 5.43e3, 0.0),
        rgb(0, 0, 139),
    );
    solar_system.add_planet(neptune);

    Model {
        solar_system,
        speed_multiplier: 1.0,
        scale_factor: 1.0e9, // 初期スケールファクター
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let dt = 60.0 * model.speed_multiplier; // 秒
    model.solar_system.update_positions(dt);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for planet in &model.solar_system.planets {
        let x = planet.position.x as f32 / model.scale_factor as f32;
        let y = planet.position.y as f32 / model.scale_factor as f32;
        let radius = if planet.name == "Sun" { 10.0 } else { 5.0 };
        draw.ellipse().color(planet.color).x_y(x, y).radius(radius);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Up => {
            model.speed_multiplier *= 1.1;
            println!("Speed multiplier increased to: {}", model.speed_multiplier);
        }
        Key::Down => {
            model.speed_multiplier *= 0.9;
            println!("Speed multiplier decreased to: {}", model.speed_multiplier);
        }
        Key::Right => {
            model.scale_factor *= 1.1;
            println!("Scale factor increased to: {}", model.scale_factor);
        }
        Key::Left => {
            model.scale_factor /= 1.1;
            println!("Scale factor decreased to: {}", model.scale_factor);
        }
        _ => {}
    }
}
