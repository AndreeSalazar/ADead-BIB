// ============================================================================
// ADead-BIB Game Engine - Main Entry Point (GPU Direct)
// ============================================================================
// Motor de juegos: ADead-BIB + Rust + GPU Directo (wgpu/Vulkan)
//
// Arquitectura:
//   - engine/   → Core del motor (gpu_renderer, input, time)
//   - games/    → Juegos (flappy_gpu usa GPU directo)
//
// Author: Eddi Andreé Salazar Matos 🇵🇪
// Made with ❤️ in Peru
// ============================================================================

mod engine;
mod ecs;
mod systems;
mod games;
mod game;

use std::sync::Arc;
use winit::event::{Event, KeyEvent, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::WindowBuilder;

use engine::{EngineConfig, Time, GpuRenderer, GpuColor};
use engine::input::{Input, KeyCode};
use games::FlappyGpu;

// Resolución inicial (se adapta al maximizar)
const INITIAL_WIDTH: u32 = 1280;
const INITIAL_HEIGHT: u32 = 720;

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║     🎮 ADead-BIB Game Engine v2.1 - GPU DIRECT             ║");
    println!("║     Architecture: ADead-BIB + Rust + wgpu (Vulkan/DX12)    ║");
    println!("║     Author: Eddi Andreé Salazar Matos 🇵🇪                   ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
    println!("🎮 Controls:");
    println!("   SPACE = Flap (jump)");
    println!("   R     = Restart");
    println!("   ESC   = Quit");
    println!();
    
    // Configuración del engine
    let config = EngineConfig::new("🎮 ADead-BIB Flappy Bird [GPU]", INITIAL_WIDTH, INITIAL_HEIGHT);
    
    // Crear event loop y ventana
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(
        WindowBuilder::new()
            .with_title(&config.title)
            .with_inner_size(winit::dpi::LogicalSize::new(config.width, config.height))
            .with_resizable(true)
            .build(&event_loop)
            .unwrap()
    );
    
    // Crear GPU renderer (Vulkan/DX12/Metal directo)
    let mut renderer = GpuRenderer::new(window.clone());
    println!("✅ GPU Renderer initialized!");
    
    // Crear sistemas
    let mut input = Input::new();
    let mut time = Time::new();
    
    // Crear juego GPU
    let mut game = FlappyGpu::new(renderer.width as f32, renderer.height as f32);
    
    println!("🚀 Game window opened! Press SPACE to start!");
    
    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);
        
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    println!("\n📊 Final Score: {}", game.score);
                    elwt.exit();
                }
                WindowEvent::KeyboardInput { event: KeyEvent { logical_key, state, .. }, .. } => {
                    let pressed = state == winit::event::ElementState::Pressed;
                    
                    match &logical_key {
                        Key::Named(NamedKey::Space) => {
                            if pressed { input.key_down(KeyCode::Space); }
                            else { input.key_up(KeyCode::Space); }
                        }
                        Key::Named(NamedKey::Escape) => {
                            if pressed {
                                println!("\n📊 Final Score: {}", game.score);
                                elwt.exit();
                            }
                        }
                        Key::Character(c) => {
                            match c.as_str() {
                                "r" | "R" => {
                                    if pressed { input.key_down(KeyCode::R); }
                                    else { input.key_up(KeyCode::R); }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                WindowEvent::Resized(size) => {
                    if size.width > 0 && size.height > 0 {
                        renderer.resize(size.width, size.height);
                        game.resize(size.width as f32, size.height as f32);
                        println!("📐 Resized to {}x{}", size.width, size.height);
                    }
                }
                WindowEvent::RedrawRequested => {
                    // Actualizar tiempo
                    time.update();
                    let delta = time.delta().min(0.1);
                    
                    // Actualizar juego
                    game.update(delta, &input);
                    
                    // Renderizar con GPU directo
                    game.render(&mut renderer);
                    renderer.present(GpuColor::SKY_BLUE.to_array());
                    
                    // Limpiar input
                    input.begin_frame();
                    
                    // Mostrar FPS
                    if time.frame() % 60 == 0 && !game.game_over && game.started {
                        println!("Score: {} | FPS: {} | GPU Direct", game.score, time.fps_int());
                    }
                }
                _ => {}
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        }
    }).unwrap();
}
