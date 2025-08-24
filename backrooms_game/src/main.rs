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
mod enemy;

use player::Player;
use textures::{load_textures, TextureManager};
use game_state::{GameState, Screen};
use minimap::Minimap;
use enemy::Enemy;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

#[macroquad::main("Backrooms - Escape the Liminal")]
async fn main() {
    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);
    
    let texture_manager = load_textures().await;
    let sounds = load_sounds().await;
    let screamer_texture = load_screamer_texture().await;
    let screamer2_texture = load_screamer2_texture().await; 
    let screamer3_texture = load_screamer3_texture().await; 
    let mut game_state = GameState::new();
    let mut player = Player::new(2.5, 2.5, 0.0);
    let mut enemy = Enemy::new();
    let minimap = Minimap::new();
    
    // Cargar textura del enemigo
    enemy.load_texture().await;
    
    let mut background_music_playing = false;
    let mut gameplay_music_playing = false; 
    let mut footstep_playing = false;
    let mut enemy_sound_playing = false;
    
    // Configurar el mouse para captura relativa
    set_cursor_grab(false);
    show_mouse(true);
    
    loop {
        let dt = get_frame_time();
        
        match game_state.current_screen {
            Screen::Menu => {
                set_cursor_grab(false);
                show_mouse(true);
                
                // Detener todos los sonidos cuando estamos en el menú
                stop_all_game_sounds(&sounds, &mut footstep_playing, &mut enemy_sound_playing, &mut gameplay_music_playing);
                
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
                    // Detener todos los sonidos al ganar
                    stop_all_game_sounds(&sounds, &mut footstep_playing, &mut enemy_sound_playing, &mut gameplay_music_playing);
                    
                    handle_victory(&mut game_state, &sounds).await;
                    draw_victory();
                } else {
                    // Actualizar el estado del juego
                    game_state.update(dt);
                    
                    if !game_state.game_over {
                        // Actualizar juego normal
                        update_game(
                            &mut player, 
                            &mut enemy,
                            &mut game_state, 
                            &sounds, 
                            &mut footstep_playing,
                            &mut enemy_sound_playing
                        ).await;
                        
                        draw_game(&player, &enemy, &game_state, &texture_manager, &minimap);
                        
                        // Dibujar screamers si están activos
                        if game_state.screamer_active {
                            draw_screamer(&screamer_texture);
                        } else if game_state.random_screamer_active {
                            draw_screamer2(&screamer2_texture); 
                        } else if game_state.death_screamer_active {
                            draw_death_screamer(&screamer3_texture);
                        }
                    } else {
                        // Mostrar screamer de muerte o game over
                        if game_state.death_screamer_active {
                            draw_death_screamer(&screamer3_texture);
                        }
                    }
                }
            }
            Screen::GameOver => {
                set_cursor_grab(false);
                show_mouse(true);
                
                // Detener todos los sonidos durante game over
                stop_all_game_sounds(&sounds, &mut footstep_playing, &mut enemy_sound_playing, &mut gameplay_music_playing);
                
                // Manejar input para regresar al menú
                if is_key_pressed(KeyCode::Space) {
                    game_state.reset();
                }
                
                draw_game_over_with_input();
            }
        }
        
        next_frame().await;
    }
}

fn stop_all_game_sounds(
    sounds: &HashMap<&str, Sound>,
    footstep_playing: &mut bool,
    enemy_sound_playing: &mut bool,
    gameplay_music_playing: &mut bool
) {
    if *footstep_playing {
        if let Some(footstep) = sounds.get("footstep") {
            stop_sound(footstep);
        }
        *footstep_playing = false;
    }
    
    if *enemy_sound_playing {
        if let Some(enemy_bg) = sounds.get("enemigoBackground") {
            stop_sound(enemy_bg);
        }
        *enemy_sound_playing = false;
    }
    
    if *gameplay_music_playing {
        if let Some(gameplay_sound) = sounds.get("gameplay_sound") {
            stop_sound(gameplay_sound);
        }
        *gameplay_music_playing = false;
    }
}

async fn load_screamer_texture() -> Option<Texture2D> {
    println!("Intentando cargar imagen del screamer...");
    
    if std::path::Path::new("scream.png").exists() {
        match load_texture("scream.png").await {
            Ok(texture) => {
                println!("✓ Imagen screamer cargada: scream.png");
                return Some(texture);
            }
            Err(e) => println!("✗ Error cargando scream.png: {}", e),
        }
    }
    
    // Intentar rutas alternativas
    let alternative_paths = vec![
        "./scream.png",
        "../scream.png",
        "assets/images/scream.png",
        "./assets/images/scream.png",
        "assets/scream.png",
    ];
    
    for alt_path in alternative_paths {
        if std::path::Path::new(&alt_path).exists() {
            println!("  → Intentando ruta alternativa: {}", alt_path);
            match load_texture(&alt_path).await {
                Ok(texture) => {
                    println!("  ✓ Imagen screamer cargada desde ruta alternativa: {}", alt_path);
                    return Some(texture);
                }
                Err(e) => println!("  ✗ Error en ruta alternativa {}: {}", alt_path, e),
            }
        }
    }
    
    println!("  ✗ No se pudo encontrar scream.png en ninguna ubicación");
    println!("  → Se usará un screamer generado por código");
    None
}

async fn load_screamer2_texture() -> Option<Texture2D> {
    println!("Intentando cargar imagen del screamer2...");
    
    if std::path::Path::new("screamer2.png").exists() {
        match load_texture("screamer2.png").await {
            Ok(texture) => {
                println!("✓ Imagen screamer2 cargada: screamer2.png");
                return Some(texture);
            }
            Err(e) => println!("✗ Error cargando screamer2.png: {}", e),
        }
    }
    
    // Intentar rutas alternativas
    let alternative_paths = vec![
        "./screamer2.png",
        "../screamer2.png",
        "assets/screamer2.png",
        "./assets/screamer2.png",
        "assets/images/screamer2.png",
    ];
    
    for alt_path in alternative_paths {
        if std::path::Path::new(&alt_path).exists() {
            println!("  → Intentando ruta alternativa: {}", alt_path);
            match load_texture(&alt_path).await {
                Ok(texture) => {
                    println!("  ✓ Imagen screamer2 cargada desde ruta alternativa: {}", alt_path);
                    return Some(texture);
                }
                Err(e) => println!("  ✗ Error en ruta alternativa {}: {}", alt_path, e),
            }
        }
    }
    
    println!("  ✗ No se pudo encontrar screamer2.png en ninguna ubicación");
    println!("  → Se usará un screamer2 generado por código");
    None
}

async fn load_screamer3_texture() -> Option<Texture2D> {
    println!("Intentando cargar imagen del screamer3 (muerte)...");
    
    if std::path::Path::new("scream3.png").exists() {
        match load_texture("scream3.png").await {
            Ok(texture) => {
                println!("✓ Imagen screamer3 cargada: scream3.png");
                return Some(texture);
            }
            Err(e) => println!("✗ Error cargando scream3.png: {}", e),
        }
    }
    
    // Intentar rutas alternativas
    let alternative_paths = vec![
        "./scream3.png",
        "../scream3.png",
        "assets/scream3.png",
        "./assets/scream3.png",
        "assets/images/scream3.png",
    ];
    
    for alt_path in alternative_paths {
        if std::path::Path::new(&alt_path).exists() {
            println!("  → Intentando ruta alternativa: {}", alt_path);
            match load_texture(&alt_path).await {
                Ok(texture) => {
                    println!("  ✓ Imagen screamer3 cargada desde ruta alternativa: {}", alt_path);
                    return Some(texture);
                }
                Err(e) => println!("  ✗ Error en ruta alternativa {}: {}", alt_path, e),
            }
        }
    }
    
    println!("  ✗ No se pudo encontrar scream3.png en ninguna ubicación");
    println!("  → Se usará un screamer3 generado por código");
    None
}

fn draw_game_over_with_input() {
    clear_background(Color::from_rgba(20, 0, 0, 255));
    
    let game_over_text = "GAME OVER";
    let game_over_size = 50.0;
    let game_over_width = measure_text(game_over_text, None, game_over_size as u16, 1.0).width;
    draw_text(
        game_over_text,
        (SCREEN_WIDTH - game_over_width) / 2.0,
        250.0,
        game_over_size,
        Color::from_rgba(255, 100, 100, 255),
    );
    
    let message = "You were caught by the entity...";
    let message_size = 22.0;
    let message_width = measure_text(message, None, message_size as u16, 1.0).width;
    draw_text(
        message,
        (SCREEN_WIDTH - message_width) / 2.0,
        320.0,
        message_size,
        WHITE,
    );
    
    let instruction = "Press SPACE to return to menu";
    let instruction_size = 18.0;
    let instruction_width = measure_text(instruction, None, instruction_size as u16, 1.0).width;
    draw_text(
        instruction,
        (SCREEN_WIDTH - instruction_width) / 2.0,
        370.0,
        instruction_size,
        Color::from_rgba(200, 200, 200, 255),
    );
}

fn draw_screamer(screamer_texture: &Option<Texture2D>) {
    // Fondo negro semi-transparente
    draw_rectangle(0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT, Color::from_rgba(0, 0, 0, 200));
    
    if let Some(texture) = screamer_texture {
        // Dibujar la imagen del screamer centrada y escalada
        let scale = 0.8;
        let texture_width = texture.width() * scale;
        let texture_height = texture.height() * scale;
        let x = (SCREEN_WIDTH - texture_width) / 2.0;
        let y = (SCREEN_HEIGHT - texture_height) / 2.0;
        
        draw_texture_ex(
            texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(texture_width, texture_height)),
                ..Default::default()
            },
        );
    } else {
        // Screamer generado por código si no hay imagen
        draw_screamer_generated();
    }
}

fn draw_screamer2(screamer2_texture: &Option<Texture2D>) {
    // Fondo rojo pulsante para diferenciarlo del primer screamer
    let pulse = (get_time() * 8.0).sin() * 0.3 + 0.7;
    draw_rectangle(0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT, 
        Color::from_rgba((50.0 * pulse) as u8, 0, 0, 180));
    
    if let Some(texture) = screamer2_texture {
        // Efecto de vibración
        let shake_x = (get_time() * 50.0).sin() * 5.0;
        let shake_y = (get_time() * 43.0).cos() * 3.0;
        
        let scale = 0.9;
        let texture_width = texture.width() * scale;
        let texture_height = texture.height() * scale;
        let x = (SCREEN_WIDTH - texture_width) as f32 / 2.0 + shake_x as f32;
        let y = (SCREEN_HEIGHT - texture_height) as f32 / 2.0 + shake_y as f32;
        
        draw_texture_ex(
            texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(texture_width, texture_height)),
                ..Default::default()
            },
        );
    } else {
        // Screamer2 generado por código si no hay imagen
        draw_screamer2_generated();
    }
}

fn draw_death_screamer(screamer3_texture: &Option<Texture2D>) {
    // Fondo negro intenso con pulso rojo
    let pulse = (get_time() * 12.0).sin() * 0.4 + 0.6;
    draw_rectangle(0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT, 
        Color::from_rgba((100.0 * pulse) as u8, 0, 0, 230));
    
    if let Some(texture) = screamer3_texture {
        // Efecto de vibración más intenso para el screamer de muerte
        let shake_x = (get_time() * 80.0).sin() * 8.0;
        let shake_y = (get_time() * 65.0).cos() * 6.0;
        
        let scale = 1.0; // Más grande que los otros
        let texture_width = texture.width() * scale;
        let texture_height = texture.height() * scale;
        let x = (SCREEN_WIDTH - texture_width) as f32 / 2.0 + shake_x as f32;
        let y = (SCREEN_HEIGHT - texture_height) as f32 / 2.0 + shake_y as f32;
        
        // Efecto de parpadeo
        let alpha = if (get_time() * 15.0).sin() > 0.0 { 255 } else { 200 };
        
        draw_texture_ex(
            texture,
            x,
            y,
            Color::from_rgba(255, 255, 255, alpha),
            DrawTextureParams {
                dest_size: Some(vec2(texture_width, texture_height)),
                ..Default::default()
            },
        );
    } else {
        // Screamer3 generado por código si no hay imagen
        draw_death_screamer_generated();
    }
}

fn draw_screamer_generated() {
    // Crear un screamer terrorífico usando formas básicas
    let center_x = SCREEN_WIDTH / 2.0;
    let center_y = SCREEN_HEIGHT / 2.0;
    
    // Cara base (oval distorsionado)
    draw_ellipse(center_x, center_y, 150.0, 200.0, 0.0, Color::from_rgba(120, 100, 80, 255));
    
    // Ojos rojos brillantes
    draw_circle(center_x - 40.0, center_y - 30.0, 25.0, Color::from_rgba(255, 0, 0, 255));
    draw_circle(center_x + 40.0, center_y - 30.0, 25.0, Color::from_rgba(255, 0, 0, 255));
    
    // Pupilas negras
    draw_circle(center_x - 40.0, center_y - 30.0, 8.0, BLACK);
    draw_circle(center_x + 40.0, center_y - 30.0, 8.0, BLACK);
    
    // Boca abierta gritando
    draw_ellipse(center_x, center_y + 40.0, 60.0, 80.0, 0.0, BLACK);
    
    // Dientes
    for i in 0..6 {
        let tooth_x = center_x - 25.0 + (i as f32 * 10.0);
        draw_rectangle(tooth_x, center_y + 10.0, 6.0, 15.0, Color::from_rgba(240, 240, 200, 255));
    }
    
    // Sangre goteando
    for i in 0..8 {
        let drop_x = center_x - 60.0 + (i as f32 * 15.0);
        let drop_y = center_y + 100.0 + (i as f32 * 5.0);
        draw_circle(drop_x, drop_y, 3.0, Color::from_rgba(180, 20, 20, 255));
        draw_rectangle(drop_x - 1.0, center_y + 80.0, 2.0, drop_y - center_y - 80.0, Color::from_rgba(180, 20, 20, 255));
    }
    
    // Texto terrorífico
    let scream_text = "YOU CAN'T ESCAPE";
    let text_size = 40.0;
    let text_width = measure_text(scream_text, None, text_size as u16, 1.0).width;
    draw_text(
        scream_text,
        (SCREEN_WIDTH - text_width) / 2.0,
        center_y + 150.0,
        text_size,
        Color::from_rgba(255, 50, 50, 255),
    );
}

fn draw_screamer2_generated() {
    // Screamer diferente - más distorsionado y perturbador
    let center_x = SCREEN_WIDTH / 2.0;
    let center_y = SCREEN_HEIGHT / 2.0;
    
    // Efecto de vibración
    let shake_x = (get_time() * 50.0).sin() * 3.0;
    let shake_y = (get_time() * 43.0).cos() * 2.0;
    let face_x = center_x + shake_x as f32;
    let face_y = center_y + shake_y as f32;
    
    // Cara más pálida y distorsionada
    draw_ellipse(face_x, face_y, 180.0, 160.0, 0.0, Color::from_rgba(200, 200, 190, 255));
    
    // Múltiples ojos en posiciones extrañas
    let eye_positions = [
        (-50.0, -40.0, 20.0),
        (50.0, -40.0, 20.0),
        (0.0, -20.0, 15.0), 
        (-30.0, -60.0, 12.0), 
    ];
    
    for &(x_offset, y_offset, size) in &eye_positions {
        // Ojo blanco
        draw_circle(face_x + x_offset, face_y + y_offset, size, WHITE);
        // Iris negro más grande
        draw_circle(face_x + x_offset, face_y + y_offset, size * 0.6, BLACK);
        // Pupila roja
        draw_circle(face_x + x_offset, face_y + y_offset, size * 0.3, RED);
    }
    
    // Boca más grande y distorsionada
    draw_ellipse(face_x, face_y + 50.0, 90.0, 60.0, 0.0, BLACK);
    
    // Dientes más irregulares
    let teeth_positions = [-35.0, -20.0, -10.0, 0.0, 10.0, 20.0, 35.0];
    for &tooth_x in &teeth_positions {
        let height = 10.0_f32 + (tooth_x * 0.3_f32).sin().abs() * 8.0_f32;
        draw_rectangle(face_x + tooth_x, face_y + 25.0, 4.0, height, 
            Color::from_rgba(220, 220, 200, 255));
    }
    
    // Grietas en la cara
    for i in 0..5 {
        let crack_start_x = face_x + (i as f32 - 2.0) * 30.0;
        let crack_start_y = face_y - 60.0;
        let crack_end_x = crack_start_x + (i as f32 * 13.0).sin() * 20.0;
        let crack_end_y = face_y + 80.0;
        
        draw_line(crack_start_x, crack_start_y, crack_end_x, crack_end_y, 2.0, 
            Color::from_rgba(100, 0, 0, 200));
    }
    
    // Texto diferente y más perturbador
    let scream_text = "BEHIND YOU";
    let text_size = 45.0;
    let text_width = measure_text(scream_text, None, text_size as u16, 1.0).width;
    
    // Texto con efecto de vibración
    let text_x = (SCREEN_WIDTH - text_width) as f32 / 2.0 + shake_x as f32;
    let text_y = center_y + 180.0 + shake_y as f32;
    
    draw_text(
        scream_text,
        text_x,
        text_y,
        text_size,
        Color::from_rgba(255, 255, 255, 255),
    );
}

fn draw_death_screamer_generated() {
    // Screamer de muerte - el más intenso y aterrador
    let center_x = SCREEN_WIDTH / 2.0;
    let center_y = SCREEN_HEIGHT / 2.0;
    
    // Vibración más intensa
    let shake_x = (get_time() * 80.0).sin() * 8.0;
    let shake_y = (get_time() * 65.0).cos() * 6.0;
    let face_x = center_x + shake_x as f32;
    let face_y = center_y + shake_y as f32;
    
    // Cara demoníaca
    draw_ellipse(face_x, face_y, 200.0, 220.0, 0.0, Color::from_rgba(80, 60, 50, 255));
    
    // Ojos enormes y terroríficos
    draw_circle(face_x - 60.0, face_y - 40.0, 35.0, Color::from_rgba(255, 0, 0, 255));
    draw_circle(face_x + 60.0, face_y - 40.0, 35.0, Color::from_rgba(255, 0, 0, 255));
    
    // Pupilas que se mueven
    let pupil_offset_x = (get_time() * 3.0).sin() * 5.0;
    let pupil_offset_y = (get_time() * 2.0).cos() * 3.0;
    draw_circle(face_x - 60.0 + pupil_offset_x as f32, face_y - 40.0 + pupil_offset_y as f32, 12.0, BLACK);
    draw_circle(face_x + 60.0 - pupil_offset_x as f32, face_y - 40.0 + pupil_offset_y as f32, 12.0, BLACK);
    
    // Boca gigante abierta
    draw_ellipse(face_x, face_y + 60.0, 120.0, 100.0, 0.0, Color::from_rgba(20, 0, 0, 255));
    
    // Dientes afilados e irregulares
    for i in 0..8 {
        let tooth_x = face_x - 50.0 + (i as f32 * 12.0);
        let tooth_height = 15.0 + (i as f32 * 3.0).sin() * 10.0;
        draw_poly(
            tooth_x, face_y + 20.0,
            3,
            8.0,
            0.0,
            Color::from_rgba(250, 250, 230, 255)
        );
        draw_rectangle(tooth_x - 3.0, face_y + 20.0, 6.0, tooth_height, 
            Color::from_rgba(250, 250, 230, 255));
    }
    
    // Sangre abundante - CORREGIDO: convertir f64 a f32
    for i in 0..12 {
        let drop_x = face_x - 80.0 + (i as f32 * 13.0);
        let drop_y = face_y + 120.0 + (i as f32 * 8.0) + ((get_time() * 2.0 + i as f64).sin() * 10.0) as f32;
        draw_circle(drop_x, drop_y, 4.0, Color::from_rgba(150, 0, 0, 255));
        draw_rectangle(drop_x - 2.0, face_y + 80.0, 4.0, 
            drop_y - face_y - 80.0, Color::from_rgba(150, 0, 0, 255));
    }
    
    // Texto de muerte parpadeante
    let death_text = "YOU DIED";
    let text_size = 60.0;
    let text_width = measure_text(death_text, None, text_size as u16, 1.0).width;
    
    // Parpadeo intenso
    let alpha = if (get_time() * 15.0).sin() > 0.0 { 255 } else { 100 };
    
    let text_x = (SCREEN_WIDTH - text_width) as f32 / 2.0 + shake_x as f32;
    let text_y = center_y + 200.0 + shake_y as f32;
    
    draw_text(
        death_text,
        text_x,
        text_y,
        text_size,
        Color::from_rgba(255, 0, 0, alpha),
    );
}

async fn load_sounds() -> HashMap<&'static str, Sound> {
    let mut sounds = HashMap::new();
    
    println!("Intentando cargar sonidos...");
    println!("Directorio de trabajo actual: {:?}", std::env::current_dir());
    
    // Audios a cargar (incluyendo los nuevos sonidos del enemigo)
    let sound_files = vec![
        ("footstep", "/assets/sounds/footstep.wav"),
        ("scream", "/assets/sounds/scream.wav"),
        ("screamer2", "/assets/sounds/screamer2.wav"), 
        ("scream3", "/assets/sounds/scream3.wav"), 
        ("enemigoBackground", "/assets/sounds/enemigoBackground.wav"), 
        ("background", "/assets/sounds/background.wav"),
        ("gameplay_sound", "/assets/sounds/gameplay_sound.wav"),
        ("victory", "/assets/sounds/victory.wav"),
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
                format!("./{}", path.trim_start_matches('/')),
                format!("../{}", path.trim_start_matches('/')),
                path.replace("/assets/", "./assets/"),
                path.replace("/assets/", "assets/"),
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
    
    println!("Sonidos cargados: {}/{}", sounds.len(), 8);
    sounds
}

async fn handle_menu(game_state: &mut GameState) {
    if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
        game_state.start_game();
    }
}

// Menu Inicial
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
    
    let warning = "⚠️ WARNING: Contains jump scares and horror elements";
    let warning_size = 16.0;
    let warning_width = measure_text(warning, None, warning_size as u16, 1.0).width;
    draw_text(
        warning,
        (SCREEN_WIDTH - warning_width) / 2.0,
        500.0,
        warning_size,
        Color::from_rgba(255, 100, 100, 200),
    );
}

async fn update_game(
    player: &mut Player, 
    enemy: &mut Enemy,
    game_state: &mut GameState, 
    sounds: &HashMap<&str, Sound>,
    footstep_playing: &mut bool,
    enemy_sound_playing: &mut bool
) {
    let dt = get_frame_time();

    player.update(dt, &game_state.world_map);

    // Activar enemigo si es momento
    if game_state.enemy_should_activate && !enemy.active {
        enemy.activate(player, &game_state.world_map);
    }

    // Actualizar enemigo
    if enemy.active {
        enemy.update(dt, player, &game_state.world_map);
        
        // Verificar si el enemigo atrapó al jugador
        if enemy.check_player_collision(player) && !game_state.game_over {
            game_state.trigger_death();
            
            // Reproducir sonido del screamer de muerte
            if !game_state.death_screamer_sound_played {
                if let Some(death_scream) = sounds.get("scream3") {
                    play_sound_once(death_scream);
                    println!("¡ENEMIGO TE ATRAPÓ! SCREAMER DE MUERTE ACTIVADO!");
                }
                game_state.death_screamer_sound_played = true;
            }
            
            // Detener sonidos del enemigo
            if *enemy_sound_playing {
                if let Some(enemy_bg) = sounds.get("enemigoBackground") {
                    stop_sound(enemy_bg);
                }
                *enemy_sound_playing = false;
            }
            
            enemy.deactivate();
            return;
        }
        
        // Controlar sonido del enemigo basado en distancia
        let distance_to_player = enemy.get_distance_to_player(player);
        if distance_to_player < 15.0 { // Solo reproducir si está relativamente cerca
            if let Some(enemy_bg) = sounds.get("enemigoBackground") {
                // Calcular volumen basado en distancia
                let volume = (1.0 - (distance_to_player / 15.0)).max(0.1).min(0.8);
                
                if !*enemy_sound_playing {
                    play_sound(
                        enemy_bg,
                        PlaySoundParams {
                            looped: true,
                            volume,
                        },
                    );
                    *enemy_sound_playing = true;
                    println!("Sonido de enemigo iniciado (distancia: {:.1})", distance_to_player);
                }
       
            }
        } else {
            // Detener sonido si está muy lejos
            if *enemy_sound_playing {
                if let Some(enemy_bg) = sounds.get("enemigoBackground") {
                    stop_sound(enemy_bg);
                }
                *enemy_sound_playing = false;
            }
        }
    }

    // Verificar si el screamer de salida debe activarse
    if game_state.check_screamer_distance(player.x, player.y) {
        // Reproducir sonido del screamer
        if let Some(scream_sound) = sounds.get("scream") {
            play_sound_once(scream_sound);
            println!("¡SCREAMER ACTIVADO!");
        }
    }
    
    // Verificar si el screamer aleatorio debe activarse
    if game_state.check_random_screamer() {
        // Reproducir sonido del screamer2
        if let Some(scream2_sound) = sounds.get("screamer2") {
            play_sound_once(scream2_sound);
            println!("¡SCREAMER ALEATORIO ACTIVADO!");
        }
    }

    // Control de pasos
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
        } else if !player.moving && *footstep_playing {
            stop_sound(footstep);
            *footstep_playing = false;
        }
    }
    
    // Verificar victoria - ACTUALIZADO para mapa 40x30
    let player_map_x = player.x as usize;
    let player_map_y = player.y as usize;
    
    if player_map_x < 40 && player_map_y < 30 && game_state.world_map[player_map_y][player_map_x] == 3 {
        game_state.escaped = true;
    }
}

fn draw_game(player: &Player, enemy: &Enemy, game_state: &GameState, texture_manager: &TextureManager, minimap: &Minimap) {
    clear_background(Color::from_rgba(20, 20, 10, 255));
    
    // Raycasting
    render_world(player, game_state, texture_manager);
    
    // Renderizar enemigo en el mundo 3D con oclusión
    enemy.render_in_world(player, SCREEN_WIDTH, SCREEN_HEIGHT, &game_state.world_map);
    
    // Minimapa - CORREGIDO: usar método draw en lugar de draw_with_enemy
    minimap.draw_with_enemy(player, enemy, &game_state.world_map);
    
    // HUD
    draw_hud();
    
    // Indicador de peligro si el enemigo está cerca
    if enemy.active {
        let distance = enemy.get_distance_to_player(player);
        if distance < 5.0 {
            draw_danger_indicator(distance);
        }
    }
}

fn draw_danger_indicator(distance: f32) {
    // Indicador de peligro que se intensifica cuando el enemigo está cerca
    let intensity = (5.0 - distance) / 5.0;
    let alpha = (intensity * 100.0) as u8;
    
    // Borde rojo pulsante - CORREGIDO: convertir f64 a f32
    let pulse = (get_time() * 8.0).sin() as f32 * 0.3 + 0.7;
    draw_rectangle_lines(
        0.0, 0.0, 
        SCREEN_WIDTH, SCREEN_HEIGHT, 
        5.0, 
        Color::from_rgba((255.0 * pulse * intensity) as u8, 0, 0, alpha),
    );
    
    // Texto de advertencia
    if distance < 2.0 {
        let warning = "DANGER!";
        let text_size = 30.0;
        let text_width = measure_text(warning, None, text_size as u16, 1.0).width;
        
        let text_alpha = ((get_time() * 10.0).sin() as f32 * 0.5 + 0.5 * 255.0) as u8;
        
        draw_text(
            warning,
            (SCREEN_WIDTH - text_width) / 2.0,
            50.0,
            text_size,
            Color::from_rgba(255, 50, 50, text_alpha),
        );
    }
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

fn cast_ray(player: &Player, angle: f32, world_map: &[[u8; 40]; 30]) -> (f32, u8, bool) {
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
        
        // ACTUALIZADO para mapa 40x30
        if map_y >= 30 || map_x >= 40 {
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
    
    // Colores de las paredes mejorados
    let mut color = match wall_type {
        1 => Color::from_rgba(160, 160, 100, 255),
        2 => Color::from_rgba(120, 60, 40, 255),   
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

// HUD simulacion de camara
fn draw_hud() {
    // Efecto de cámara vintage con bordes más sutiles
    draw_rectangle_lines(5.0, 5.0, SCREEN_WIDTH - 10.0, SCREEN_HEIGHT - 10.0, 1.5, Color::from_rgba(200, 50, 50, 180));
    
    // Timestamp estilo cámara
    let timestamp = "00:02:47";
    draw_text(timestamp, 15.0, SCREEN_HEIGHT - 60.0, 18.0, Color::from_rgba(255, 100, 100, 200));
    
    // Fecha
    let date = "19. SEP. 1998";
    draw_text(date, 15.0, SCREEN_HEIGHT - 40.0, 16.0, Color::from_rgba(255, 100, 100, 180));
    
    // Indicador de grabación
    draw_circle(SCREEN_WIDTH - 35.0, 25.0, 4.0, Color::from_rgba(255, 50, 50, 200));
    draw_text("REC", SCREEN_WIDTH - 70.0, 30.0, 14.0, Color::from_rgba(255, 100, 100, 200));
    
    // Botón PLAY
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