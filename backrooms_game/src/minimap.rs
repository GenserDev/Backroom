use macroquad::prelude::*;
use crate::player::Player;

pub struct Minimap {
    size: f32,
    scale: f32,
    position: (f32, f32),
}

impl Minimap {
    pub fn new() -> Self {
        Self {
            size: 120.0,
            scale: 6.0,
            position: (10.0, 10.0),
        }
    }
    
    pub fn draw(&self, player: &Player, world_map: &[[u8; 20]; 15]) { 
        let (map_x, map_y) = self.position;
        
        // Fondo del minimapa
        draw_rectangle(map_x, map_y, self.size, self.size, Color::from_rgba(0, 0, 0, 200));
        draw_rectangle_lines(map_x, map_y, self.size, self.size, 2.0, WHITE);
        
        // Calcular el área visible alrededor del jugador
        let player_map_x = player.x as i32;
        let player_map_y = player.y as i32;
        let view_range = (self.size / self.scale) as i32 / 2;
        
        for dy in -view_range..=view_range {
            for dx in -view_range..=view_range {
                let world_x = player_map_x + dx;
                let world_y = player_map_y + dy;
                
                // Verificar límites del mundo (20x15)
                if world_x >= 0 && world_x < 20 && world_y >= 0 && world_y < 15 {
                    let world_x = world_x as usize;
                    let world_y = world_y as usize;
                    
                    let cell = world_map[world_y][world_x];
                    
                    let screen_x = map_x + (dx + view_range) as f32 * self.scale;
                    let screen_y = map_y + (dy + view_range) as f32 * self.scale;
                    
                    let color = match cell {
                        0 => Color::from_rgba(50, 50, 30, 255),   
                        1 => Color::from_rgba(180, 180, 100, 255), 
                        2 => Color::from_rgba(120, 60, 60, 255), 
                        3 => Color::from_rgba(50, 200, 50, 255),   
                        _ => GRAY,
                    };
                    
                    draw_rectangle(screen_x, screen_y, self.scale, self.scale, color);
                }
            }
        }
        
        // Dibujar al jugador como un punto rojo en el centro
        let center_x = map_x + self.size / 2.0;
        let center_y = map_y + self.size / 2.0;
        draw_circle(center_x, center_y, 3.0, RED);
        
        // Dibujar dirección del jugador
        let dir_length = 8.0;
        let end_x = center_x + player.angle.cos() * dir_length;
        let end_y = center_y + player.angle.sin() * dir_length;
        draw_line(center_x, center_y, end_x, end_y, 2.0, RED);
        
        // Etiqueta del minimapa
        draw_text("MAP", map_x, map_y + self.size + 15.0, 16.0, WHITE);
    }
}