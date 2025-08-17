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
            size: 150.0, 
            scale: 5.0,  
            position: (10.0, 10.0),
        }
    }
    
    pub fn draw(&self, player: &Player, world_map: &[[u8; 40]; 30]) { 
        let (map_x, map_y) = self.position;
        
        // Fondo del minimapa con mejor contraste
        draw_rectangle(map_x, map_y, self.size, self.size, Color::from_rgba(0, 0, 0, 220));
        draw_rectangle_lines(map_x, map_y, self.size, self.size, 2.0, WHITE);
        
        // Calcular el área visible alrededor del jugador
        let player_map_x = player.x;
        let player_map_y = player.y;
        
        // Dibujar las celdas visibles
        let cells_per_side = (self.size / self.scale) as i32;
        let half_cells = cells_per_side / 2;
        
        for dy in -half_cells..=half_cells {
            for dx in -half_cells..=half_cells {
                let world_x = player_map_x + dx as f32;
                let world_y = player_map_y + dy as f32;
                
                // Verificar límites del mundo (40x30)
                if world_x >= 0.0 && world_x < 40.0 && world_y >= 0.0 && world_y < 30.0 {
                    let world_x_int = world_x as usize;
                    let world_y_int = world_y as usize;
                    
                    // Asegurar que no salgamos de los límites del array
                    if world_x_int < 40 && world_y_int < 30 {
                        let cell = world_map[world_y_int][world_x_int];
                        
                        // Calcular posición en pantalla centrada
                        let screen_x = map_x + (dx + half_cells) as f32 * self.scale;
                        let screen_y = map_y + (dy + half_cells) as f32 * self.scale;
                        
                        // Verificar que la celda esté dentro del minimapa
                        if screen_x >= map_x && screen_x < map_x + self.size - self.scale &&
                           screen_y >= map_y && screen_y < map_y + self.size - self.scale {
                            
                            let color = match cell {
                                0 => Color::from_rgba(40, 40, 25, 255),   
                                1 => Color::from_rgba(200, 190, 120, 255), 
                                2 => Color::from_rgba(140, 60, 60, 255),   
                                3 => Color::from_rgba(60, 220, 60, 255),   
                                _ => GRAY,
                            };
                            
                            draw_rectangle(screen_x, screen_y, self.scale, self.scale, color);
                            
                            // Añadir borde sutil a las paredes para mejor definición
                            if cell == 1 || cell == 2 {
                                draw_rectangle_lines(screen_x, screen_y, self.scale, self.scale, 1.0, 
                                    Color::from_rgba(80, 80, 60, 100));
                            }
                        }
                    }
                }
            }
        }
        
        // Dibujar al jugador como un punto rojo en el centro
        let center_x = map_x + self.size / 2.0;
        let center_y = map_y + self.size / 2.0;
        draw_circle(center_x, center_y, 4.0, RED);
        
        // Dibujar dirección del jugador con línea más visible
        let dir_length = 10.0;
        let end_x = center_x + player.angle.cos() * dir_length;
        let end_y = center_y + player.angle.sin() * dir_length;
        draw_line(center_x, center_y, end_x, end_y, 3.0, Color::from_rgba(255, 100, 100, 255));
        
        // Etiqueta del minimapa con mejor contraste
        draw_text("MAP", map_x, map_y + self.size + 15.0, 16.0, WHITE);
        
        // Mostrar coordenadas del jugador para debugging
        let coord_text = format!("X:{:.1} Y:{:.1}", player.x, player.y);
        draw_text(&coord_text, map_x, map_y + self.size + 30.0, 12.0, 
            Color::from_rgba(180, 180, 180, 200));
    }
}