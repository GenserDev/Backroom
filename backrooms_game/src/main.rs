use macroquad::prelude::*;
use macroquad::audio::{load_sound_from_bytes, play_sound, play_sound_once, PlaySoundParams, Sound};
use std::collections::HashMap;

mod player;
mod textures;
mod game_state;

use player::Player;
use textures::{load_textures, TextureManager};
use game_state::{GameState, Screen};

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

#[macroquad::main("Backrooms - Escape the Liminal")]
async fn main() {
    // Configurar ventana
    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);
    
    // Cargar recursos
    let texture_manager = load_textures().await;
    let mut sounds = load_sounds().await;
    let mut game_state = GameState::new();
    let mut player = Player::new(1.5, 1.5, 0.0);
    
    loop {
        match game_state.current_screen {
            Screen::Menu => {
                handle_menu(&mut game_state).await;
                draw_menu(&texture_manager);
            }
            Screen::Game => {
                if game_state.escaped {
                    handle_victory(&mut game_state, &sounds).await;
                    draw_victory();
                } else {
                    update_game(&mut player, &mut game_state, &sounds).await;
                    draw_game(&player, &game_state, &texture_manager);
                }
            }
        }
        
        next_frame().await;
    }
}

async fn load_sounds() -> HashMap<&'static str, Sound> {
    let mut sounds = HashMap::new();
    
    // Como no podemos cargar sonidos reales sin archivos externos,
    // mantenemos el HashMap vacío pero funcional
    // En un proyecto real cargarías archivos .ogg o .wav aquí
    
    sounds
}

async fn handle_menu(game_state: &mut GameState) {
    if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
        game_state.current_screen = Screen::Game;
    }
}

fn draw_menu(_texture_manager: &TextureManager) {
    clear_background(BLACK);
    
    // Título
    let title = "BACKROOMS";
    let title_size = 60.0;
    let title_width = measure_text(title, None, title_size as u16, 1.0).width;
    draw_text(
        title,
        (SCREEN_WIDTH - title_width) / 2.0,
        200.0,
        title_size,
        YELLOW,
    );
    
    // Subtítulo
    let subtitle = "Escape the Liminal Space";
    let subtitle_size = 30.0;
    let subtitle_width = measure_text(subtitle, None, subtitle_size as u16, 1.0).width;
    draw_text(
        subtitle,
        (SCREEN_WIDTH - subtitle_width) / 2.0,
        250.0,
        subtitle_size,
        DARKGRAY,
    );
    
    // Instrucciones
    let instruction = "Press SPACE to Start";
    let instruction_size = 25.0;
    let instruction_width = measure_text(instruction, None, instruction_size as u16, 1.0).width;
    draw_text(
        instruction,
        (SCREEN_WIDTH - instruction_width) / 2.0,
        400.0,
        instruction_size,
        WHITE,
    );
    
    // Controles
    let controls = "Use WASD to move, Mouse to look around";
    let controls_size = 20.0;
    let controls_width = measure_text(controls, None, controls_size as u16, 1.0).width;
    draw_text(
        controls,
        (SCREEN_WIDTH - controls_width) / 2.0,
        450.0,
        controls_size,
        GRAY,
    );
}

async fn update_game(player: &mut Player, game_state: &mut GameState, sounds: &HashMap<&str, Sound>) {
    let dt = get_frame_time();
    
    // Actualizar jugador
    let was_moving = player.is_moving();
    player.update(dt, &game_state.world_map);
    
    // Reproducir sonido de pasos
    if player.is_moving() && !was_moving {
        if let Some(footstep) = sounds.get("footstep") {
            play_sound_once(footstep);
        }
    }
    
    // Verificar si llegó a la salida
    let player_map_x = player.x as usize;
    let player_map_y = player.y as usize;
    
    if game_state.world_map[player_map_y][player_map_x] == 3 {
        game_state.escaped = true;
    }
}

fn draw_game(player: &Player, game_state: &GameState, texture_manager: &TextureManager) {
    clear_background(Color::from_rgba(20, 20, 10, 255));
    
    // Renderizar mundo 3D usando raycasting
    render_world(player, game_state, texture_manager);
    
    // HUD
    draw_hud();
}

fn render_world(player: &Player, game_state: &GameState, texture_manager: &TextureManager) {
    let fov = std::f32::consts::PI / 3.0; // 60 grados
    let half_fov = fov / 2.0;
    let num_rays = SCREEN_WIDTH as usize;
    let delta_angle = fov / num_rays as f32;
    
    for i in 0..num_rays {
        let angle = player.angle - half_fov + i as f32 * delta_angle;
        let (distance, wall_type, hit_vertical) = cast_ray(player, angle, &game_state.world_map);
        
        if distance > 0.0 {
            draw_wall_slice(i, distance, wall_type, hit_vertical, texture_manager);
        }
    }
}

fn cast_ray(player: &Player, angle: f32, world_map: &[[u8; 20]; 15]) -> (f32, u8, bool) {
    let dx = angle.cos();
    let dy = angle.sin();
    let mut x = player.x;
    let mut y = player.y;
    let step_size = 0.02;
    
    loop {
        x += dx * step_size;
        y += dy * step_size;
        
        let map_x = x as usize;
        let map_y = y as usize;
        
        if map_y >= world_map.len() || map_x >= world_map[0].len() {
            return (1000.0, 1, false); // Muy lejos
        }
        
        let wall_type = world_map[map_y][map_x];
        if wall_type != 0 {
            let distance = ((x - player.x).powi(2) + (y - player.y).powi(2)).sqrt();
            let hit_vertical = x.fract() < 0.1 || x.fract() > 0.9;
            return (distance, wall_type, hit_vertical);
        }
    }
}

fn draw_wall_slice(x: usize, distance: f32, wall_type: u8, hit_vertical: bool, texture_manager: &TextureManager) {
    // Corregir distancia para evitar efecto ojo de pez
    let corrected_distance = distance * (x as f32 - SCREEN_WIDTH / 2.0).to_radians().cos().abs();
    
    let wall_height = (SCREEN_HEIGHT / (corrected_distance + 0.0001)) * 0.5;
    let wall_top = (SCREEN_HEIGHT - wall_height) / 2.0;
    let wall_bottom = wall_top + wall_height;
    
    // Color basado en el tipo de pared y orientación
    let mut color = match wall_type {
        1 => Color::from_rgba(180, 180, 120, 255), // Pared normal (amarillo backrooms)
        2 => Color::from_rgba(120, 80, 40, 255),   // Pared con manchas de sangre
        3 => Color::from_rgba(40, 200, 40, 255),   // Salida (verde)
        _ => GRAY,
    };
    
    // Oscurecer paredes verticales para dar profundidad
    if hit_vertical {
        color.r *= 0.7;
        color.g *= 0.7;
        color.b *= 0.7;
    }
    
    // Oscurecer basado en distancia
    let brightness = (1.0 - (corrected_distance / 10.0).min(0.8)).max(0.2);
    color.r *= brightness;
    color.g *= brightness;
    color.b *= brightness;
    
    // Dibujar pared
    draw_line(x as f32, wall_top, x as f32, wall_bottom, 1.0, color);
    
    // Dibujar suelo y techo
    if wall_top > 0.0 {
        draw_line(x as f32, 0.0, x as f32, wall_top, 1.0, Color::from_rgba(40, 30, 20, 255)); // Techo
    }
    if wall_bottom < SCREEN_HEIGHT {
        draw_line(x as f32, wall_bottom, x as f32, SCREEN_HEIGHT, 1.0, Color::from_rgba(60, 50, 30, 255)); // Suelo
    }
}

fn draw_hud() {
    // Efecto de cámara vintage
    draw_rectangle_lines(10.0, 10.0, SCREEN_WIDTH - 20.0, SCREEN_HEIGHT - 20.0, 2.0, RED);
    
    // Timestamp estilo cámara
    let timestamp = "00:02:47";
    draw_text(timestamp, 20.0, SCREEN_HEIGHT - 60.0, 20.0, RED);
    
    // Fecha
    let date = "19. SEP. 1998";
    draw_text(date, 20.0, SCREEN_HEIGHT - 40.0, 20.0, RED);
    
    // Indicador de grabación
    draw_circle(SCREEN_WIDTH - 40.0, 30.0, 5.0, RED);
    draw_text("REC", SCREEN_WIDTH - 80.0, 35.0, 16.0, RED);
    
    // Botón PLAY en la esquina superior izquierda
    draw_text("PLAY >", 20.0, 30.0, 20.0, WHITE);
}

async fn handle_victory(game_state: &mut GameState, sounds: &HashMap<&str, Sound>) {
    if !game_state.victory_sound_played {
        if let Some(victory) = sounds.get("victory") {
            play_sound_once(victory);
        }
        game_state.victory_sound_played = true;
    }
    
    if is_key_pressed(KeyCode::Escape) {
        game_state.reset();
    }
}

fn draw_victory() {
    clear_background(Color::from_rgba(10, 50, 10, 255));
    
    let victory_text = "¡FELICIDADES!";
    let victory_size = 50.0;
    let victory_width = measure_text(victory_text, None, victory_size as u16, 1.0).width;
    draw_text(
        victory_text,
        (SCREEN_WIDTH - victory_width) / 2.0,
        200.0,
        victory_size,
        GREEN,
    );
    
    let message = "Has encontrado la salida y escapado del Backroom";
    let message_size = 25.0;
    let message_width = measure_text(message, None, message_size as u16, 1.0).width;
    draw_text(
        message,
        (SCREEN_WIDTH - message_width) / 2.0,
        300.0,
        message_size,
        WHITE,
    );
    
    let instruction = "Press ESC to return to menu";
    let instruction_size = 20.0;
    let instruction_width = measure_text(instruction, None, instruction_size as u16, 1.0).width;
    draw_text(
        instruction,
        (SCREEN_WIDTH - instruction_width) / 2.0,
        400.0,
        instruction_size,
        GRAY,
    );
}