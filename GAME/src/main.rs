// ADead-BIB Vulkan Bird Game - INTERACTIVE VERSION
// A Flappy Bird clone with real graphics window
//
// Author: Eddi Andreé Salazar Matos
// Made with ❤️ in Peru 🇵🇪

mod game;

use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::Instant;
use winit::event::{Event, KeyEvent, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::WindowBuilder;
use game::GameState;
use rand::Rng;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║     🎮 ADead-BIB Vulkan Bird - INTERACTIVE                 ║");
    println!("║     Press SPACE to flap! Avoid the pipes!                  ║");
    println!("║     Author: Eddi Andreé Salazar Matos 🇵🇪                   ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
    println!("🎮 Controls:");
    println!("   SPACE = Flap (jump)");
    println!("   R     = Restart");
    println!("   ESC   = Quit");
    println!();
    
    // Create event loop and window
    let event_loop = EventLoop::new().unwrap();
    let window = Rc::new(
        WindowBuilder::new()
            .with_title("🎮 ADead-BIB Vulkan Bird - Press SPACE to flap!")
            .with_inner_size(winit::dpi::LogicalSize::new(WIDTH, HEIGHT))
            .with_resizable(false)
            .build(&event_loop)
            .unwrap()
    );
    
    // Create software buffer for rendering
    let context = softbuffer::Context::new(window.clone()).unwrap();
    let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();
    
    // Initialize game
    let mut game = GameState::new();
    let mut last_frame = Instant::now();
    let mut frame_count: u64 = 0;
    let mut rng = rand::thread_rng();
    
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
                    if state == winit::event::ElementState::Pressed {
                        match logical_key {
                            Key::Named(NamedKey::Space) => {
                                if game.game_over {
                                    game.reset();
                                    println!("🔄 Game restarted!");
                                } else {
                                    game.flap();
                                }
                            }
                            Key::Character(c) if c.as_str() == "r" || c.as_str() == "R" => {
                                game.reset();
                                println!("🔄 Game restarted!");
                            }
                            Key::Named(NamedKey::Escape) => {
                                println!("\n📊 Final Score: {}", game.score);
                                elwt.exit();
                            }
                            _ => {}
                        }
                    }
                }
                WindowEvent::RedrawRequested => {
                    // Frame timing
                    let now = Instant::now();
                    let dt = now.duration_since(last_frame).as_secs_f32();
                    last_frame = now;
                    frame_count += 1;
                    
                    // Update game (60 FPS target)
                    if dt < 0.1 {
                        game.update();
                    }
                    
                    // Spawn pipes with randomness
                    if game.frame % 120 == 0 && !game.game_over {
                        let gap_y = 100.0 + rng.gen::<f32>() * 250.0;
                        game.spawn_pipe(gap_y);
                    }
                    
                    // Render
                    surface.resize(
                        NonZeroU32::new(WIDTH).unwrap(),
                        NonZeroU32::new(HEIGHT).unwrap(),
                    ).unwrap();
                    
                    let mut buffer = surface.buffer_mut().unwrap();
                    render_game(&mut buffer, &game, WIDTH, HEIGHT);
                    buffer.present().unwrap();
                    
                    // Print score updates
                    if frame_count % 60 == 0 && !game.game_over {
                        let fps = 1.0 / dt;
                        println!("Score: {} | FPS: {:.0}", game.score, fps);
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

fn render_game(buffer: &mut [u32], game: &GameState, width: u32, height: u32) {
    // Clear to sky blue
    let sky_color = 0xFF87CEEB;
    for pixel in buffer.iter_mut() {
        *pixel = sky_color;
    }
    
    // Draw ground
    let ground_y = height - 80;
    for y in ground_y..height {
        for x in 0..width {
            let idx = (y * width + x) as usize;
            if y < ground_y + 10 {
                buffer[idx] = 0xFF228B22; // Green grass
            } else {
                buffer[idx] = 0xFF8B4513; // Brown dirt
            }
        }
    }
    
    // Draw pipes
    for pipe in &game.pipes {
        let px = pipe.x as i32;
        let gap_top = pipe.gap_y as i32;
        let gap_bottom = (pipe.gap_y + pipe.gap_size) as i32;
        let pw = pipe.width as i32;
        
        // Top pipe
        for y in 0..gap_top {
            for x in px.max(0)..(px + pw).min(width as i32) {
                if y >= 0 && x >= 0 {
                    let idx = (y as u32 * width + x as u32) as usize;
                    if idx < buffer.len() {
                        // Pipe with border
                        if x == px || x == px + pw - 1 {
                            buffer[idx] = 0xFF005500; // Dark green border
                        } else {
                            buffer[idx] = 0xFF00AA00; // Green pipe
                        }
                    }
                }
            }
        }
        
        // Pipe cap (top)
        for y in (gap_top - 30).max(0)..gap_top {
            for x in (px - 5).max(0)..(px + pw + 5).min(width as i32) {
                let idx = (y as u32 * width + x as u32) as usize;
                if idx < buffer.len() {
                    buffer[idx] = 0xFF008800;
                }
            }
        }
        
        // Bottom pipe
        for y in gap_bottom..(ground_y as i32) {
            for x in px.max(0)..(px + pw).min(width as i32) {
                if y >= 0 && x >= 0 && (y as u32) < height {
                    let idx = (y as u32 * width + x as u32) as usize;
                    if idx < buffer.len() {
                        if x == px || x == px + pw - 1 {
                            buffer[idx] = 0xFF005500;
                        } else {
                            buffer[idx] = 0xFF00AA00;
                        }
                    }
                }
            }
        }
        
        // Pipe cap (bottom)
        for y in gap_bottom..((gap_bottom + 30).min(ground_y as i32)) {
            for x in (px - 5).max(0)..(px + pw + 5).min(width as i32) {
                let idx = (y as u32 * width + x as u32) as usize;
                if idx < buffer.len() {
                    buffer[idx] = 0xFF008800;
                }
            }
        }
    }
    
    // Draw bird
    let bx = game.bird.x as i32;
    let by = game.bird.y as i32;
    let bw = game.bird.width as i32;
    let bh = game.bird.height as i32;
    
    // Bird body (yellow)
    for y in by.max(0)..(by + bh).min(height as i32) {
        for x in bx.max(0)..(bx + bw).min(width as i32) {
            let idx = (y as u32 * width + x as u32) as usize;
            if idx < buffer.len() {
                buffer[idx] = 0xFFFFCC00; // Yellow
            }
        }
    }
    
    // Bird eye (white)
    let eye_x = bx + bw * 2 / 3;
    let eye_y = by + bh / 4;
    for y in eye_y.max(0)..(eye_y + 8).min(height as i32) {
        for x in eye_x.max(0)..(eye_x + 8).min(width as i32) {
            let idx = (y as u32 * width + x as u32) as usize;
            if idx < buffer.len() {
                buffer[idx] = 0xFFFFFFFF; // White
            }
        }
    }
    
    // Bird pupil (black)
    let pupil_x = eye_x + 3;
    let pupil_y = eye_y + 3;
    for y in pupil_y.max(0)..(pupil_y + 4).min(height as i32) {
        for x in pupil_x.max(0)..(pupil_x + 4).min(width as i32) {
            let idx = (y as u32 * width + x as u32) as usize;
            if idx < buffer.len() {
                buffer[idx] = 0xFF000000; // Black
            }
        }
    }
    
    // Bird beak (orange)
    let beak_x = bx + bw;
    let beak_y = by + bh / 3;
    for y in beak_y.max(0)..(beak_y + 10).min(height as i32) {
        for x in beak_x.max(0)..(beak_x + 15).min(width as i32) {
            let idx = (y as u32 * width + x as u32) as usize;
            if idx < buffer.len() {
                buffer[idx] = 0xFFFF8800; // Orange
            }
        }
    }
    
    // Draw score (simple rectangles as digits)
    draw_score(buffer, game.score, width);
    
    // Draw game over overlay
    if game.game_over {
        draw_game_over(buffer, width, height);
    }
}

fn draw_score(buffer: &mut [u32], score: u32, width: u32) {
    let score_str = format!("{}", score);
    let start_x = 20;
    let start_y = 20;
    
    for (i, _) in score_str.chars().enumerate() {
        let x = start_x + (i as i32) * 25;
        // Draw digit background
        for dy in 0..30 {
            for dx in 0..20 {
                let px = x + dx;
                let py = start_y + dy;
                if px >= 0 && (px as u32) < width && py >= 0 {
                    let idx = (py as u32 * width + px as u32) as usize;
                    if idx < buffer.len() {
                        buffer[idx] = 0xFFFFFFFF; // White
                    }
                }
            }
        }
    }
}

fn draw_game_over(buffer: &mut [u32], width: u32, height: u32) {
    let box_w = 300;
    let box_h = 100;
    let box_x = (width as i32 - box_w) / 2;
    let box_y = (height as i32 - box_h) / 2;
    
    // Semi-transparent red overlay
    for y in box_y..(box_y + box_h) {
        for x in box_x..(box_x + box_w) {
            if x >= 0 && (x as u32) < width && y >= 0 && (y as u32) < height {
                let idx = (y as u32 * width + x as u32) as usize;
                if idx < buffer.len() {
                    // Red with some transparency effect
                    buffer[idx] = 0xFFCC0000;
                }
            }
        }
    }
    
    // White border
    for y in (box_y + 5)..(box_y + box_h - 5) {
        for x in (box_x + 5)..(box_x + box_w - 5) {
            if x >= 0 && (x as u32) < width && y >= 0 && (y as u32) < height {
                let idx = (y as u32 * width + x as u32) as usize;
                if idx < buffer.len() {
                    buffer[idx] = 0xFF880000;
                }
            }
        }
    }
    
    // "GAME OVER" text area (white rectangle)
    let text_x = box_x + 50;
    let text_y = box_y + 35;
    for y in text_y..(text_y + 30) {
        for x in text_x..(text_x + 200) {
            if x >= 0 && (x as u32) < width && y >= 0 && (y as u32) < height {
                let idx = (y as u32 * width + x as u32) as usize;
                if idx < buffer.len() {
                    buffer[idx] = 0xFFFFFFFF;
                }
            }
        }
    }
}
