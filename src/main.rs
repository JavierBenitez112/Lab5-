// main.rs

mod framebuffer;
mod triangle;
mod line;
mod vertex;
mod fragment;
mod shaders;
mod obj;
mod matrix;
mod camera;
mod light;

use crate::matrix::{create_model_matrix, create_projection_matrix, create_viewport_matrix};
use crate::camera::Camera;
use crate::light::Light;
use framebuffer::Framebuffer;
use vertex::Vertex;
use triangle::triangle;
use shaders::{vertex_shader, vertex_shader_sun, fragment_shader_planet, PlanetType};
use obj::Obj;
use raylib::prelude::*;
use std::thread;
use std::time::Duration;
use std::f32::consts::PI;

pub struct Uniforms {
    pub model_matrix: Matrix,
    pub view_matrix: Matrix,
    pub projection_matrix: Matrix,
    pub viewport_matrix: Matrix,
    pub time: f32,
}

// Estructura para representar un planeta en el sistema solar
struct Planet {
    orbital_radius: f32,      // Radio de la órbita
    orbital_angle: f32,         // Ángulo actual en la órbita
    orbital_speed: f32,         // Velocidad angular de la órbita
    rotation_speed: f32,        // Velocidad de rotación propia
    scale: f32,                 // Escala del planeta
    planet_type: PlanetType,    // Tipo de shader del planeta
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex], light: &Light, planet_type: PlanetType) {
    // Vertex Shader Stage
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Log the first 3 transformed vertices for debugging
    // println!("--- Transformed Vertices (first 3) ---");
    // for i in 0..3.min(transformed_vertices.len()) {
    //     println!("Vertex {}: {:?}", i, transformed_vertices[i].transformed_position);
    // }

    // Primitive Assembly Stage
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterization Stage
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2], light));
    }

    // Fragment Processing Stage
    for fragment in fragments {
        // Run fragment shader to compute final color with planet type
        let final_color = fragment_shader_planet(&fragment, uniforms, planet_type);

        framebuffer.point(
            fragment.position.x as i32,
            fragment.position.y as i32,
            final_color,
            fragment.depth
        );
    }
}

/// Función especializada para renderizar el sol con vertex shader especial
fn render_sun(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex], light: &Light) {
    // Vertex Shader Stage - Usa el vertex shader especial del sol
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader_sun(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Primitive Assembly Stage
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterization Stage
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2], light));
    }

    // Fragment Processing Stage - Usa el shader del sol
    for fragment in fragments {
        let final_color = fragment_shader_planet(&fragment, uniforms, PlanetType::Sun);

        framebuffer.point(
            fragment.position.x as i32,
            fragment.position.y as i32,
            final_color,
            fragment.depth
        );
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;

    let (mut window, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Solar System")
        .log_level(TraceLogLevel::LOG_WARNING) // Suppress INFO messages
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);
    framebuffer.set_background_color(Vector3::new(0.02, 0.02, 0.05)); // Espacio profundo casi negro

    // Initialize the texture inside the framebuffer
    framebuffer.init_texture(&mut window, &thread);

    // Camera setup
    let camera_position = Vector3::new(0.0, 1.0, 10.0);
    let camera_target = Vector3::new(0.0, 0.0, 0.0);
    let camera_up = Vector3::new(0.0, 1.0, 0.0);
    let mut camera = Camera::new(camera_position, camera_target, camera_up);

    // Projection setup
    let fov_y = PI / 3.0; // 60 degrees
    let aspect = window_width as f32 / window_height as f32;
    let near = 0.1;
    let far = 100.0;

    // Light setup
    let light = Light::new(Vector3::new(5.0, 5.0, 5.0));

    // Generate sphere mesh programmatically (usaremos el mismo modelo para todos los planetas)
    let sphere = Obj::generate_sphere(1.0, 32); // Radio 1.0, 32 segmentos
    let vertex_array = sphere.get_vertex_array();

    // Crear sistema solar con 5 planetas orbitando
    let mut planets = vec![
        Planet {
            orbital_radius: 4.0,      // Órbita cercana
            orbital_angle: 0.0,        // Empieza en ángulo 0
            orbital_speed: 0.5,        // Velocidad rápida
            rotation_speed: 0.05,      // Rotación propia
            scale: 0.8,                 // Planeta pequeño
            planet_type: PlanetType::Rocky,
        },
        Planet {
            orbital_radius: 6.0,       // Órbita media
            orbital_angle: PI * 2.0 / 5.0, // Empieza a 72 grados
            orbital_speed: 0.3,        // Velocidad media
            rotation_speed: 0.03,
            scale: 1.2,                // Planeta mediano
            planet_type: PlanetType::GasGiant,
        },
        Planet {
            orbital_radius: 8.0,       // Órbita lejana
            orbital_angle: PI * 4.0 / 5.0, // Empieza a 144 grados
            orbital_speed: 0.2,        // Velocidad lenta
            rotation_speed: 0.02,
            scale: 1.0,                // Planeta normal
            planet_type: PlanetType::SciFi,
        },
        Planet {
            orbital_radius: 10.0,      // Órbita muy lejana
            orbital_angle: PI * 6.0 / 5.0, // Empieza a 216 grados
            orbital_speed: 0.15,       // Velocidad muy lenta
            rotation_speed: 0.04,
            scale: 0.9,                // Planeta helado
            planet_type: PlanetType::Ice,
        },
        Planet {
            orbital_radius: 12.0,      // Órbita más lejana
            orbital_angle: PI * 8.0 / 5.0, // Empieza a 288 grados
            orbital_speed: 0.12,       // Velocidad muy lenta
            rotation_speed: 0.06,
            scale: 1.1,                // Planeta volcánico
            planet_type: PlanetType::Volcanic,
        },
    ];

    // Generar geometría para anillos (alrededor del gigante gaseoso)
    let rings = Obj::generate_rings(2.5, 3.5, 16, 32);
    let rings_vertex_array = rings.get_vertex_array();

    // Generar luna (pequeña esfera que orbita alrededor del planeta rocoso)
    let moon = Obj::generate_sphere(0.3, 16);
    let moon_vertex_array = moon.get_vertex_array();

    // Generar el SOL (esfera en el centro del sistema solar)
    // Usar más segmentos para un sol más suave y detallado
    let sun = Obj::generate_sphere(2.0, 64); // Radio 2.0, 64 segmentos para máxima calidad
    let sun_vertex_array = sun.get_vertex_array();

    let mut elapsed_time = 0.0f32;

    while !window.window_should_close() {
        // Get delta time from Raylib
        let delta_time = window.get_frame_time();
        elapsed_time += delta_time;

        // Process camera input
        camera.process_input(&window);

        // Update orbital positions and rotations
        for planet in &mut planets {
            planet.orbital_angle += planet.orbital_speed * delta_time;
            if planet.orbital_angle >= 2.0 * PI {
                planet.orbital_angle -= 2.0 * PI;
            }
        }

        framebuffer.clear();

        let view_matrix = camera.get_view_matrix();
        let projection_matrix = create_projection_matrix(fov_y, aspect, near, far);
        let viewport_matrix = create_viewport_matrix(0.0, 0.0, window_width as f32, window_height as f32);

        // ======================================
        // RENDERIZAR EL SOL EN EL CENTRO
        // ======================================
        let sun_translation = Vector3::new(0.0, 0.0, 0.0); // Centro del sistema
        let sun_rotation = Vector3::new(0.0, elapsed_time * 0.1, 0.0); // Rotación lenta del sol
        let sun_model_matrix = create_model_matrix(sun_translation, 1.0, sun_rotation);
        
        let sun_uniforms = Uniforms {
            model_matrix: sun_model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time: elapsed_time,
        };

        // Usar la función especializada render_sun
        render_sun(&mut framebuffer, &sun_uniforms, &sun_vertex_array, &light);

        // Renderizar cada planeta en su órbita
        for (idx, planet) in planets.iter().enumerate() {
            // Calcular posición orbital en el plano XY
            let orbit_x = planet.orbital_radius * planet.orbital_angle.cos();
            let orbit_z = planet.orbital_radius * planet.orbital_angle.sin();
            let orbit_y = 0.0; // Todos en el mismo plano horizontal
            
            let translation = Vector3::new(orbit_x, orbit_y, orbit_z);
            
            // Rotación propia del planeta alrededor de su eje Y
            let planet_self_rotation = elapsed_time * planet.rotation_speed;
            let rotation = Vector3::new(0.0, planet_self_rotation, 0.0);
            
            let model_matrix = create_model_matrix(translation, planet.scale, rotation);
            
            let uniforms = Uniforms {
                model_matrix,
                view_matrix,
                projection_matrix,
                viewport_matrix,
                time: elapsed_time,
            };

            render(&mut framebuffer, &uniforms, &vertex_array, &light, planet.planet_type);

            // Renderizar anillos alrededor del gigante gaseoso (índice 1)
            if idx == 1 {
                // Anillos están en la misma posición que el planeta pero sin rotación propia
                let rings_rotation = Vector3::new(0.0, 0.0, 0.0); // Los anillos no rotan
                let rings_matrix = create_model_matrix(translation, 1.0, rings_rotation);
                let rings_uniforms = Uniforms {
                    model_matrix: rings_matrix,
                    view_matrix,
                    projection_matrix,
                    viewport_matrix,
                    time: elapsed_time,
                };
                render(&mut framebuffer, &rings_uniforms, &rings_vertex_array, &light, PlanetType::Ring);
            }

            // Renderizar luna orbitando alrededor del primer planeta (índice 0)
            if idx == 0 {
                // La luna orbita alrededor del planeta rocoso
                let moon_orbital_radius = 1.5;
                let moon_orbital_angle = elapsed_time * 1.0; // Velocidad orbital de la luna
                let moon_orbit_x = orbit_x + moon_orbital_radius * moon_orbital_angle.cos();
                let moon_orbit_z = orbit_z + moon_orbital_radius * moon_orbital_angle.sin();
                let moon_orbit_y = 0.2; // Ligeramente elevada
                
                let moon_translation = Vector3::new(moon_orbit_x, moon_orbit_y, moon_orbit_z);
                let moon_rotation = Vector3::new(0.0, elapsed_time * 0.1, 0.0);
                let moon_matrix = create_model_matrix(moon_translation, 1.0, moon_rotation);
                
                let moon_uniforms = Uniforms {
                    model_matrix: moon_matrix,
                    view_matrix,
                    projection_matrix,
                    viewport_matrix,
                    time: elapsed_time,
                };
                render(&mut framebuffer, &moon_uniforms, &moon_vertex_array, &light, PlanetType::Moon);
            }
        }

        // Actualizar textura del framebuffer y dibujar todo en un solo frame
        framebuffer.update_texture();

        let mut d = window.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        framebuffer.draw_to(&mut d);

        // Crosshair centrado
        let center_x = window_width / 2;
        let center_y = window_height / 2;
        let crosshair_size = 10;
        d.draw_line(center_x - crosshair_size, center_y, center_x + crosshair_size, center_y, Color::WHITE);
        d.draw_line(center_x, center_y - crosshair_size, center_x, center_y + crosshair_size, Color::WHITE);


        thread::sleep(Duration::from_millis(16));
    }
}
