use macroquad::prelude::*;
use std::collections::HashMap;

pub struct TextureManager {
    pub textures: HashMap<String, Texture2D>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }
    
    pub fn add_texture(&mut self, name: String, texture: Texture2D) {
        self.textures.insert(name, texture);
    }
}

pub async fn load_textures() -> TextureManager {
    let mut texture_manager = TextureManager::new();
    
    // Generar texturas mejoradas con ladrillos
    let wall_texture = generate_brick_wall_texture();
    texture_manager.add_texture("wall".to_string(), wall_texture);
    
    let bloody_wall_texture = generate_bloody_brick_wall_texture();
    texture_manager.add_texture("bloody_wall".to_string(), bloody_wall_texture);
    
    let exit_texture = generate_exit_texture();
    texture_manager.add_texture("exit".to_string(), exit_texture);
    
    let floor_texture = generate_floor_texture();
    texture_manager.add_texture("floor".to_string(), floor_texture);
    
    texture_manager
}

fn generate_brick_wall_texture() -> Texture2D {
    const SIZE: usize = 128;
    let mut pixels = vec![0u8; SIZE * SIZE * 4];
    
    let brick_width = 20;
    let brick_height = 8;
    let mortar_thickness = 2;
    
    for y in 0..SIZE {
        for x in 0..SIZE {
            let idx = (y * SIZE + x) * 4;
            
            let row = y / (brick_height + mortar_thickness);
            let col = if row % 2 == 0 {
                x / (brick_width + mortar_thickness)
            } else {
                (x + brick_width / 2) / (brick_width + mortar_thickness)
            };
            
            let brick_x = x % (brick_width + mortar_thickness);
            let brick_y = y % (brick_height + mortar_thickness);
            
            let is_mortar = brick_x >= brick_width || brick_y >= brick_height;
            
            if is_mortar {
                pixels[idx] = 90;
                pixels[idx + 1] = 85;
                pixels[idx + 2] = 80;
                pixels[idx + 3] = 255;
            } else {
                let mut r = 200u8;
                let mut g = 190u8;
                let mut b = 120u8;
                
                let brick_seed = (row * 1000 + col) as u32;
                let variation = (brick_seed % 30) as i16 - 15;
                
                r = (r as i16 + variation).max(150).min(220) as u8;
                g = (g as i16 + variation).max(140).min(210) as u8;
                b = (b as i16 + variation/2).max(80).min(150) as u8;
                
                //Convertir todo a u32 para la operación
                let noise = ((x as u32 * 13 + y as u32 * 17 + brick_seed) % 20) as i16 - 10;
                r = (r as i16 + noise/3).max(0).min(255) as u8;
                g = (g as i16 + noise/3).max(0).min(255) as u8;
                b = (b as i16 + noise/4).max(0).min(255) as u8;
                
                //Crear un binding intermedio para edge_distance
                let edge_distances = [brick_x, brick_width - brick_x, brick_y, brick_height - brick_y];
                let edge_distance = *edge_distances.iter().min().unwrap();
                
                if edge_distance < 2 {
                    let fade = 0.9;
                    r = (r as f32 * fade) as u8;
                    g = (g as f32 * fade) as u8;
                    b = (b as f32 * fade) as u8;
                }
                
                pixels[idx] = r;
                pixels[idx + 1] = g;
                pixels[idx + 2] = b;
                pixels[idx + 3] = 255;
            }
        }
    }
    
    let image = Image {
        bytes: pixels,
        width: SIZE as u16,
        height: SIZE as u16,
    };
    
    Texture2D::from_image(&image)
}

fn generate_bloody_brick_wall_texture() -> Texture2D {
    const SIZE: usize = 128;
    let mut pixels = vec![0u8; SIZE * SIZE * 4];
    
    let brick_width = 20;
    let brick_height = 8;
    let mortar_thickness = 2;
    
    for y in 0..SIZE {
        for x in 0..SIZE {
            let idx = (y * SIZE + x) * 4;
            
            let row = y / (brick_height + mortar_thickness);
            let col = if row % 2 == 0 {
                x / (brick_width + mortar_thickness)
            } else {
                (x + brick_width / 2) / (brick_width + mortar_thickness)
            };
            
            let brick_x = x % (brick_width + mortar_thickness);
            let brick_y = y % (brick_height + mortar_thickness);
            
            let is_mortar = brick_x >= brick_width || brick_y >= brick_height;
            
            if is_mortar {
                pixels[idx] = 70;
                pixels[idx + 1] = 60;
                pixels[idx + 2] = 55;
                pixels[idx + 3] = 255;
            } else {
                let mut r = 160u8;
                let mut g = 150u8;
                let mut b = 90u8;
                
                let brick_seed = (row * 1000 + col) as u32;
                let variation = (brick_seed % 20) as i16 - 10;
                
                r = (r as i16 + variation).max(120).min(180) as u8;
                g = (g as i16 + variation).max(110).min(170) as u8;
                b = (b as i16 + variation/2).max(60).min(120) as u8;
                
                pixels[idx] = r;
                pixels[idx + 1] = g;
                pixels[idx + 2] = b;
                pixels[idx + 3] = 255;
            }
        }
    }
    
    add_blood_stains(&mut pixels, SIZE);
    add_help_messages(&mut pixels, SIZE);
    
    let image = Image {
        bytes: pixels,
        width: SIZE as u16,
        height: SIZE as u16,
    };
    
    Texture2D::from_image(&image)
}

fn add_blood_stains(pixels: &mut Vec<u8>, size: usize) {
    let blood_patterns = [
        (20, 30, 15, BloodType::Drip),
        (70, 20, 20, BloodType::Splatter),
        (40, 80, 12, BloodType::Handprint),
        (90, 60, 18, BloodType::Drip),
        (15, 90, 25, BloodType::Splatter),
    ];
    
    for &(center_x, center_y, radius, blood_type) in &blood_patterns {
        if center_x < size && center_y < size {
            match blood_type {
                BloodType::Drip => create_blood_drip(pixels, size, center_x, center_y, radius),
                BloodType::Splatter => create_blood_splatter(pixels, size, center_x, center_y, radius),
                BloodType::Handprint => create_handprint(pixels, size, center_x, center_y),
            }
        }
    }
}

#[derive(Clone, Copy)]
enum BloodType {
    Drip,
    Splatter,
    Handprint,
}

fn create_blood_drip(pixels: &mut Vec<u8>, size: usize, start_x: usize, start_y: usize, length: usize) {
    for i in 0..length {
        let y = start_y + i;
        if y >= size { break; }
        
        let x = start_x;
        if x < size {
            apply_blood_color(pixels, size, x, y, 255);
        }
        
        if i > 5 && (i % 3 == 0) {
            let side_offset = if i % 6 < 3 { 1 } else { -1 };
            let side_x = (start_x as i32 + side_offset) as usize;
            if side_x < size {
                apply_blood_color(pixels, size, side_x, y, 180);
            }
        }
        
        if i == length - 1 {
            for dx in -2..=2 {
                for dy in 0..3 {
                    let drop_x = (start_x as i32 + dx) as usize;
                    let drop_y = y + dy;
                    if drop_x < size && drop_y < size {
                        // FIX: Convertir a f32 desde el inicio
                        let distance = ((dx as f32) * (dx as f32) + (dy as f32) * (dy as f32)).sqrt();
                        if distance < 4.0 {
                            apply_blood_color(pixels, size, drop_x, drop_y, 200);
                        }
                    }
                }
            }
        }
    }
}

fn create_blood_splatter(pixels: &mut Vec<u8>, size: usize, center_x: usize, center_y: usize, radius: usize) {
    for y in 0..size {
        for x in 0..size {
            let dx = x as i32 - center_x as i32;
            let dy = y as i32 - center_y as i32;
            let distance = (dx * dx + dy * dy) as f32;
            let max_distance = (radius * radius) as f32;
            
            if distance < max_distance {
                let noise = ((x * 13 + y * 17) % 100) as f32 / 100.0;
                let splatter_probability = (1.0 - distance / max_distance) * noise;
                
                if splatter_probability > 0.3 {
                    let intensity = (splatter_probability * 255.0) as u8;
                    apply_blood_color(pixels, size, x, y, intensity);
                }
                
                if distance > max_distance * 0.7 && noise > 0.8 {
                    apply_blood_color(pixels, size, x, y, 150);
                }
            }
        }
    }
}
//Textura de huellas de mano
fn create_handprint(pixels: &mut Vec<u8>, size: usize, center_x: usize, center_y: usize) {
    let hand_pattern = [
        (0, 0, 3), (1, 0, 3), (2, 0, 3), (-1, 0, 3), (-2, 0, 3),
        (0, 1, 3), (1, 1, 3), (2, 1, 3), (-1, 1, 3), (-2, 1, 3),
        (0, 2, 3), (1, 2, 3), (2, 2, 2), (-1, 2, 3), (-2, 2, 2),
        
        (0, -1, 2), (0, -2, 2), (0, -3, 1),
        (1, -1, 2), (1, -2, 1),
        (-1, -1, 2), (-1, -2, 1),
        (2, -1, 1),
        (-2, -1, 1),
        
        (-3, 0, 2), (-3, -1, 1),
    ];
    
    for &(dx, dy, intensity) in &hand_pattern {
        let x = (center_x as i32 + dx * 3) as usize;
        let y = (center_y as i32 + dy * 3) as usize;
        
        if x < size && y < size {
            apply_blood_color(pixels, size, x, y, intensity * 80);
            
            for adj_y in 0..2 {
                for adj_x in 0..2 {
                    let adj_px = x + adj_x;
                    let adj_py = y + adj_y;
                    if adj_px < size && adj_py < size {
                        apply_blood_color(pixels, size, adj_px, adj_py, intensity * 60);
                    }
                }
            }
        }
    }
}

fn apply_blood_color(pixels: &mut Vec<u8>, size: usize, x: usize, y: usize, intensity: u8) {
    let idx = (y * size + x) * 4;
    
    let existing_r = pixels[idx] as f32;
    let existing_g = pixels[idx + 1] as f32;
    let existing_b = pixels[idx + 2] as f32;
    
    let blood_factor = intensity as f32 / 255.0;
    let keep_factor = 1.0 - blood_factor * 0.7;
    
    let blood_r = 120.0;
    let blood_g = 20.0;
    let blood_b = 20.0;
    
    pixels[idx] = (existing_r * keep_factor + blood_r * blood_factor) as u8;
    pixels[idx + 1] = (existing_g * keep_factor + blood_g * blood_factor) as u8;
    pixels[idx + 2] = (existing_b * keep_factor + blood_b * blood_factor) as u8;
}

fn add_help_messages(pixels: &mut Vec<u8>, size: usize) {
    draw_text_on_texture(pixels, size, "HELP", 10, 50, 200, 30, 30);
    draw_text_on_texture(pixels, size, "RUN", 80, 20, 180, 40, 40);
    draw_text_on_texture(pixels, size, "27", 90, 100, 160, 50, 50);
}

fn draw_text_on_texture(pixels: &mut Vec<u8>, size: usize, text: &str, start_x: usize, start_y: usize, r: u8, g: u8, b: u8) {
    let patterns = get_letter_patterns();
    
    let mut current_x = start_x;
    for ch in text.chars() {
        if let Some(pattern) = patterns.get(&ch) {
            draw_letter_pattern(pixels, size, pattern, current_x, start_y, r, g, b);
            current_x += 6;
        }
    }
}

fn get_letter_patterns() -> HashMap<char, Vec<Vec<bool>>> {
    let mut patterns = HashMap::new();
    
    patterns.insert('H', vec![
        vec![true, false, false, false, true],
        vec![true, false, false, false, true],
        vec![true, false, false, false, true],
        vec![true, true, true, true, true],
        vec![true, false, false, false, true],
        vec![true, false, false, false, true],
        vec![true, false, false, false, true],
    ]);
    
    patterns.insert('E', vec![
        vec![true, true, true, true, true],
        vec![true, false, false, false, false],
        vec![true, false, false, false, false],
        vec![true, true, true, true, false],
        vec![true, false, false, false, false],
        vec![true, false, false, false, false],
        vec![true, true, true, true, true],
    ]);
    
    patterns.insert('L', vec![
        vec![true, false, false, false, false],
        vec![true, false, false, false, false],
        vec![true, false, false, false, false],
        vec![true, false, false, false, false],
        vec![true, false, false, false, false],
        vec![true, false, false, false, false],
        vec![true, true, true, true, true],
    ]);
    
    patterns.insert('P', vec![
        vec![true, true, true, true, false],
        vec![true, false, false, false, true],
        vec![true, false, false, false, true],
        vec![true, true, true, true, false],
        vec![true, false, false, false, false],
        vec![true, false, false, false, false],
        vec![true, false, false, false, false],
    ]);
    
    patterns.insert('R', vec![
        vec![true, true, true, true, false],
        vec![true, false, false, false, true],
        vec![true, false, false, false, true],
        vec![true, true, true, true, false],
        vec![true, false, false, true, false],
        vec![true, false, false, false, true],
        vec![true, false, false, false, true],
    ]);
    
    patterns.insert('U', vec![
        vec![true, false, false, false, true],
        vec![true, false, false, false, true],
        vec![true, false, false, false, true],
        vec![true, false, false, false, true],
        vec![true, false, false, false, true],
        vec![true, false, false, false, true],
        vec![false, true, true, true, false],
    ]);
    
    patterns.insert('N', vec![
        vec![true, false, false, false, true],
        vec![true, true, false, false, true],
        vec![true, false, true, false, true],
        vec![true, false, true, false, true],
        vec![true, false, false, true, true],
        vec![true, false, false, false, true],
        vec![true, false, false, false, true],
    ]);
    
    patterns.insert('2', vec![
        vec![false, true, true, true, false],
        vec![true, false, false, false, true],
        vec![false, false, false, false, true],
        vec![false, false, false, true, false],
        vec![false, false, true, false, false],
        vec![false, true, false, false, false],
        vec![true, true, true, true, true],
    ]);
    
    patterns.insert('7', vec![
        vec![true, true, true, true, true],
        vec![false, false, false, false, true],
        vec![false, false, false, true, false],
        vec![false, false, true, false, false],
        vec![false, true, false, false, false],
        vec![false, true, false, false, false],
        vec![false, true, false, false, false],
    ]);
    
    patterns
}

fn draw_letter_pattern(pixels: &mut Vec<u8>, size: usize, pattern: &Vec<Vec<bool>>, start_x: usize, start_y: usize, r: u8, g: u8, b: u8) {
    for (row, line) in pattern.iter().enumerate() {
        for (col, &pixel) in line.iter().enumerate() {
            if pixel {
                let x = start_x + col;
                let y = start_y + row;
                if x < size && y < size {
                    let idx = (y * size + x) * 4;
                    pixels[idx] = r;
                    pixels[idx + 1] = g;
                    pixels[idx + 2] = b;
                    pixels[idx + 3] = 255;
                }
            }
        }
    }
}

fn generate_exit_texture() -> Texture2D {
    const SIZE: usize = 64;
    let mut pixels = vec![0u8; SIZE * SIZE * 4];
    
    for y in 0..SIZE {
        for x in 0..SIZE {
            let idx = (y * SIZE + x) * 4;
            
            let center_x = SIZE as f32 / 2.0;
            let center_y = SIZE as f32 / 2.0;
            let distance = ((x as f32 - center_x).powi(2) + (y as f32 - center_y).powi(2)).sqrt();
            let max_distance = SIZE as f32 / 2.0;
            
            let brightness_factor = (1.0 - (distance / max_distance).min(1.0)) * 0.5 + 0.5;
            let base_brightness = if (x + y) % 8 < 4 { 200 } else { 150 };
            let brightness = (base_brightness as f32 * brightness_factor) as u8;
            
            pixels[idx] = 40;        
            pixels[idx + 1] = brightness; 
            pixels[idx + 2] = 40;    
            pixels[idx + 3] = 255;   
        }
    }
    
    let image = Image {
        bytes: pixels,
        width: SIZE as u16,
        height: SIZE as u16,
    };
    
    Texture2D::from_image(&image)
}

fn generate_floor_texture() -> Texture2D {
    const SIZE: usize = 64;
    let mut pixels = vec![0u8; SIZE * SIZE * 4];
    
    for y in 0..SIZE {
        for x in 0..SIZE {
            let idx = (y * SIZE + x) * 4;
            
            let pattern = (x / 8 + y / 8) % 2;
            let base = if pattern == 0 { 70 } else { 60 };
            let variation = ((x + y) % 4) as u8 * 3;
            let noise = ((x * 13 + y * 17) % 10) as u8;
            
            pixels[idx] = base + variation + noise;     
            pixels[idx + 1] = (base + variation + noise / 2).min(255); 
            pixels[idx + 2] = (35 + variation + noise / 3).min(255);   
            pixels[idx + 3] = 255;              
        }
    }
    
    let image = Image {
        bytes: pixels,
        width: SIZE as u16,
        height: SIZE as u16,
    };
    
    Texture2D::from_image(&image)
}