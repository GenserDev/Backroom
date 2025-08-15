/// Universidad del Valle de Guatemala
/// 
/// Backrooms -- Proyecto 1 -- Graficas
/// 
/// Genser Catalán -- 23401

use macroquad::prelude::*;
use macroquad::audio::{load_sound, play_sound, play_sound_once, stop_sound, Sound, PlaySoundParams};
use std::collections::HashMap;

mod player;
mod textures;
mod game_state;
mod minimap;

use player::Player;
use textures::{load_textures, TextureManager};
use game_state::{GameState, Screen};
use minimap::Minimap;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

#[macroquad::main("Backrooms - Escape the Liminal")]
async fn main() {
    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);
    
    let texture_manager = load_textures().await;
    let sounds = load_sounds().await;
    let mut game_state = GameState::new();
    let mut player = Player::new(1.5, 1.5, 0.0);
    let minimap = Minimap::new();
    
    let mut background_music_playing = false;
    let mut gameplay_music_playing = false; 
    let mut footstep_playing = false; 
    
    // Configurar el mouse para captura relativa cuando estemos en el juego
    set_cursor_grab(false);
    show_mouse(true);
    
    loop {
        match game_state.current_screen {
            Screen::Menu => {
                set_cursor_grab(false);
                show_mouse(true);
                
                // Detener audio de pasos si está reproduciéndose
                if footstep_playing {
                    if let Some(footstep) = sounds.get("footstep") {
                        stop_sound(footstep);
                    }
                    footstep_playing = false;
                }
                
                // Detener música de gameplay si está reproduciéndose
                if gameplay_music_playing {
                    if let Some(gameplay_sound) = sounds.get("gameplay_sound") {
                        stop_sound(gameplay_sound);
                    }
                    gameplay_music_playing = false;
                }
                
                // Reproducir música de fondo en el menú
                if !background_music_playing {
                    if let Some(bg_music) = sounds.get("background") {
                        play_sound(
                            bg_music,
                            PlaySoundParams {
                                looped: true,
                                volume: 0.7,
                            },
                        );
                        background_music_playing = true;
                        println!("Música de fondo iniciada");
                    }
                }
                
                handle_menu(&mut game_state).await;
                draw_menu(&texture_manager);
            }
            Screen::Game => {
                set_cursor_grab(true);
                show_mouse(false);
                
                // Detener música de menú al entrar al juego
                if background_music_playing {
                    if let Some(bg_music) = sounds.get("background") {
                        stop_sound(bg_music);
                    }
                    background_music_playing = false;
                }
                
                // Iniciar música de gameplay
                if !gameplay_music_playing {
                    if let Some(gameplay_sound) = sounds.get("gameplay_sound") {
                        play_sound(
                            gameplay_sound,
                            PlaySoundParams {
                                looped: true,
                                volume: 0.5, 
                            },
                        );
                        gameplay_music_playing = true;
                        println!("Música de gameplay iniciada");
                    }
                }
                
                if game_state.escaped {
                    // Detener audio de pasos al ganar
                    if footstep_playing {
                        if let Some(footstep) = sounds.get("footstep") {
                            stop_sound(footstep);
                        }
                        footstep_playing = false;
                    }
                    
                    // Detener música de gameplay al ganar
                    if gameplay_music_playing {
                        if let Some(gameplay_sound) = sounds.get("gameplay_sound") {
                            stop_sound(gameplay_sound);
                        }
                        gameplay_music_playing = false;
                    }
                    
                    handle_victory(&mut game_state, &sounds).await;
                    draw_victory();
                } else {
                    update_game(
                        &mut player, 
                        &mut game_state, 
                        &sounds, 
                        &mut footstep_playing
                    ).await;
                    draw_game(&player, &game_state, &texture_manager, &minimap);
                }
            }
        }
        
        next_frame().await;
    }
}

async fn load_sounds() -> HashMap<&'static str, Sound> {
    let mut sounds = HashMap::new();
    
    println!("Intentando cargar sonidos...");
    println!("Directorio de trabajo actual: {:?}", std::env::current_dir());
    
    //Audios a cargar
    let sound_files = vec![
        ("footstep", "footstep.wav"),
        ("background", "background.wav"),
        ("gameplay_sound", "gameplay_sound.wav"),
        ("victory", "victory.wav"),
    ];
    
    for (name, path) in sound_files {
        if std::path::Path::new(path).exists() {
            match load_sound(path).await {
                Ok(sound) => {
                    sounds.insert(name, sound);
                    println!("✓ Sonido '{}' cargado desde: {}", name, path);
                }
                Err(e) => println!("✗ Error cargando {}: {}", path, e),
            }
        } else {
            println!("✗ Archivo no encontrado: {}", path);
            let alternative_paths = vec![
                format!("./{}", path),
                format!("../{}", path),
                path.replace("assets/", "./assets/"),
            ];
            
            let mut found = false;
            for alt_path in alternative_paths {
                if std::path::Path::new(&alt_path).exists() {
                    println!("  → Intentando ruta alternativa: {}", alt_path);
                    match load_sound(&alt_path).await {
                        Ok(sound) => {
                            sounds.insert(name, sound);
                            println!("  ✓ Sonido '{}' cargado desde ruta alternativa: {}", name, alt_path);
                            found = true;
                            break;
                        }
                        Err(e) => println!("  ✗ Error en ruta alternativa {}: {}", alt_path, e),
                    }
                }
            }
            
            if !found {
                println!("  ✗ No se pudo encontrar {} en ninguna ubicación", name);
            }
        }
    }
    
    println!("Sonidos cargados: {}/{}", sounds.len(), 4);
    sounds
}

async fn handle_menu(game_state: &mut GameState) {
    if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
        game_state.current_screen = Screen::Game;
    }
}

//Menu Inicial
fn draw_menu(_texture_manager: &TextureManager) {
    clear_background(BLACK);
    

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

async fn update_game(
    player: &mut Player, 
    game_state: &mut GameState, 
    sounds: &HashMap<&str, Sound>,
    footstep_playing: &mut bool
) {
    let dt = get_frame_time();

    player.update(dt, &game_state.world_map);

    if let Some(footstep) = sounds.get("footstep") {
        if player.moving && !*footstep_playing {
            play_sound(
                footstep,
                PlaySoundParams {
                    looped: true,
                    volume: 0.3, 
                },
            );
            *footstep_playing = true;
            println!("Iniciando audio de pasos en loop");
        } else if !player.moving && *footstep_playing {
            stop_sound(footstep);
            *footstep_playing = false;
            println!("Deteniendo audio de pasos");
        }
    }
    
    //Verificar victoria
    let player_map_x = player.x as usize;
    let player_map_y = player.y as usize;
    
    if player_map_x < 20 && player_map_y < 15 && game_state.world_map[player_map_y][player_map_x] == 3 {
        game_state.escaped = true;
    }
}

fn draw_game(player: &Player, game_state: &GameState, texture_manager: &TextureManager, minimap: &Minimap) {
    clear_background(Color::from_rgba(20, 20, 10, 255));
    
    //Raycasting
    render_world(player, game_state, texture_manager);
    
    //Minimapa
    minimap.draw(player, &game_state.world_map);
    
    //HUD
    draw_hud();
}

fn render_world(player: &Player, game_state: &GameState, _texture_manager: &TextureManager) {
    let fov = std::f32::consts::PI / 3.0; 
    let half_fov = fov / 2.0;
    let num_rays = SCREEN_WIDTH as usize;
    let delta_angle = fov / num_rays as f32;
    
    for i in 0..num_rays {
        let angle = player.angle - half_fov + i as f32 * delta_angle;
        let (distance, wall_type, hit_vertical) = cast_ray(player, angle, &game_state.world_map);
        
        if distance > 0.0 {
            draw_wall_slice(i, distance, wall_type, hit_vertical, angle - player.angle);
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
            return (1000.0, 1, false); 
        }
        
        let wall_type = world_map[map_y][map_x];
        if wall_type != 0 {
            let distance = ((x - player.x).powi(2) + (y - player.y).powi(2)).sqrt();
            let hit_vertical = x.fract() < 0.1 || x.fract() > 0.9;
            return (distance, wall_type, hit_vertical);
        }
    }
}

fn draw_wall_slice(x: usize, distance: f32, wall_type: u8, hit_vertical: bool, ray_angle: f32) {
    let corrected_distance = distance * ray_angle.cos();
    
    let wall_height = (SCREEN_HEIGHT / (corrected_distance + 0.0001)) * 0.6;
    let wall_top = (SCREEN_HEIGHT - wall_height) / 2.0;
    let wall_bottom = wall_top + wall_height;
    
    //Colores de las paredes
    let mut color = match wall_type {
        1 => Color::from_rgba(160, 160, 100, 255), 
        2 => Color::from_rgba(100, 60, 40, 255),   
        3 => Color::from_rgba(40, 180, 40, 255),   
        _ => GRAY,
    };
    
    // Oscurecer paredes verticales para dar profundidad
    if hit_vertical {
        color.r *= 0.8;
        color.g *= 0.8;
        color.b *= 0.8;
    }
    
    // Oscurecer basado en distancia para mejor atmósfera
    let brightness = (1.0 - (corrected_distance / 12.0).min(0.7)).max(0.3);
    color.r *= brightness;
    color.g *= brightness;
    color.b *= brightness;
    
    // Dibujar pared
    draw_line(x as f32, wall_top, x as f32, wall_bottom, 1.0, color);
    
    // Dibujar suelo y techo con gradiente
    if wall_top > 0.0 {
        let ceiling_color = Color::from_rgba(
            (30.0 * brightness) as u8, 
            (25.0 * brightness) as u8, 
            (15.0 * brightness) as u8, 
            255
        );
        draw_line(x as f32, 0.0, x as f32, wall_top, 1.0, ceiling_color);
    }
    if wall_bottom < SCREEN_HEIGHT {
        let floor_color = Color::from_rgba(
            (50.0 * brightness) as u8, 
            (40.0 * brightness) as u8, 
            (20.0 * brightness) as u8, 
            255
        );
        draw_line(x as f32, wall_bottom, x as f32, SCREEN_HEIGHT, 1.0, floor_color);
    }
}

//HUD simulacion de camara
fn draw_hud() {
    // Efecto de cámara vintage con bordes más sutiles
    draw_rectangle_lines(5.0, 5.0, SCREEN_WIDTH - 10.0, SCREEN_HEIGHT - 10.0, 1.5, Color::from_rgba(200, 50, 50, 180));
    
    // Timestamp estilo cámara
    let timestamp = "00:02:47";
    draw_text(timestamp, 15.0, SCREEN_HEIGHT - 60.0, 18.0, Color::from_rgba(255, 100, 100, 200));
    
    // Fecha
    let date = "19. SEP. 1998";
    draw_text(date, 15.0, SCREEN_HEIGHT - 40.0, 16.0, Color::from_rgba(255, 100, 100, 180));
    
    //Indicador de grabación
    draw_circle(SCREEN_WIDTH - 35.0, 25.0, 4.0, Color::from_rgba(255, 50, 50, 200));
    draw_text("REC", SCREEN_WIDTH - 70.0, 30.0, 14.0, Color::from_rgba(255, 100, 100, 200));
    
    //Botón PLAY
    draw_text("PLAY >", 15.0, 25.0, 18.0, Color::from_rgba(200, 200, 200, 180));
    draw_text("WASD: Move | Mouse: Look", 15.0, SCREEN_HEIGHT - 20.0, 12.0, Color::from_rgba(150, 150, 150, 120));
}

async fn handle_victory(game_state: &mut GameState, sounds: &HashMap<&str, Sound>) {
    set_cursor_grab(false);
    show_mouse(true);
    
    if !game_state.victory_sound_played {
        if let Some(victory) = sounds.get("victory") {
            play_sound_once(victory);
        }
        game_state.victory_sound_played = true;
    }
    
    if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Space) {
        game_state.reset();
    }
}

fn draw_victory() {
    clear_background(Color::from_rgba(10, 40, 10, 255));
    
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
    let message_size = 22.0;
    let message_width = measure_text(message, None, message_size as u16, 1.0).width;
    draw_text(
        message,
        (SCREEN_WIDTH - message_width) / 2.0,
        280.0,
        message_size,
        WHITE,
    );
    
    let instruction = "Press ESC or SPACE to play again";
    let instruction_size = 18.0;
    let instruction_width = measure_text(instruction, None, instruction_size as u16, 1.0).width;
    draw_text(
        instruction,
        (SCREEN_WIDTH - instruction_width) / 2.0,
        350.0,
        instruction_size,
        GRAY,
    );
}