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
    
    // Generar texturas  
    let wall_texture = generate_wall_texture();
    texture_manager.add_texture("wall".to_string(), wall_texture);
    
    let bloody_wall_texture = generate_bloody_wall_texture();
    texture_manager.add_texture("bloody_wall".to_string(), bloody_wall_texture);
    
    let exit_texture = generate_exit_texture();
    texture_manager.add_texture("exit".to_string(), exit_texture);
    
    let floor_texture = generate_floor_texture();
    texture_manager.add_texture("floor".to_string(), floor_texture);
    
    texture_manager
}

fn generate_wall_texture() -> Texture2D {
    const SIZE: usize = 64;
    let mut pixels = vec![0u8; SIZE * SIZE * 4];
    
    for y in 0..SIZE {
        for x in 0..SIZE {
            let idx = (y * SIZE + x) * 4;
            
            // Crear textura de pared amarillenta estilo backrooms
            let base_color = 180;
            let variation = ((x + y) % 8) as u8 * 5;
            
            pixels[idx] = base_color - variation;     
            pixels[idx + 1] = base_color - variation; 
            pixels[idx + 2] = 100 - variation / 2;    
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

fn generate_bloody_wall_texture() -> Texture2D {
    const SIZE: usize = 64;
    let mut pixels = vec![0u8; SIZE * SIZE * 4];
    
    for y in 0..SIZE {
        for x in 0..SIZE {
            let idx = (y * SIZE + x) * 4;
            
            // Base amarilla
            let mut r = 180u8;
            let mut g = 180u8;
            let mut b = 100u8;
            
            // Agregar manchas de sangre
            let blood_pattern = (x * 7 + y * 13) % 100;
            if blood_pattern < 20 {
                r = 120;
                g = 30;
                b = 30;
            }
            
            let variation = ((x + y) % 8) as u8 * 3;
            
            pixels[idx] = r.saturating_sub(variation);
            pixels[idx + 1] = g.saturating_sub(variation);
            pixels[idx + 2] = b.saturating_sub(variation);
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

fn generate_exit_texture() -> Texture2D {
    const SIZE: usize = 64;
    let mut pixels = vec![0u8; SIZE * SIZE * 4];
    
    for y in 0..SIZE {
        for x in 0..SIZE {
            let idx = (y * SIZE + x) * 4;
            
            // Crear textura verde brillante para la salida
            let brightness = if (x + y) % 10 < 5 { 200 } else { 150 };
            
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
            
            // Patrón de alfombra húmeda
            let pattern = (x / 8 + y / 8) % 2;
            let base = if pattern == 0 { 60 } else { 50 };
            let variation = ((x + y) % 4) as u8 * 2;
            
            pixels[idx] = base + variation;     
            pixels[idx + 1] = base + variation; 
            pixels[idx + 2] = 30 + variation;   
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