use macroquad::prelude::*;
use crate::player::Player;
// use rand::Rng;

pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub alive: bool,
    pub catch_distance: f32,
    pub texture: Option<Texture2D>,
    pub size: f32,
    pub active: bool, // Para controlar si el enemigo está activo
}

impl Enemy {
    pub fn new() -> Self {
        Self {
            x: 10.0, // Posición inicial alejada del jugador
            y: 10.0,
            speed: 2.8, // Aumentado de 1.8 a 2.8 para ser más rápido
            alive: true,
            catch_distance: 1.0, // Distancia para atrapar al jugador
            texture: None,
            size: 1.8, // Tamaño del enemigo en el mundo
            active: false, // Inicia inactivo
        }
    }
    
    pub async fn load_texture(&mut self) {
        println!("Intentando cargar imagen del enemigo...");
        
        if std::path::Path::new("enemigo.png").exists() {
            match load_texture("enemigo.png").await {
                Ok(texture) => {
                    self.texture = Some(texture);
                    println!("✓ Imagen enemigo cargada: enemigo.png");
                    return;
                }
                Err(e) => println!("✗ Error cargando enemigo.png: {}", e),
            }
        }
        
        // Intentar rutas alternativas
        let alternative_paths = vec![
            "./enemigo.png",
            "../enemigo.png",
            "assets/images/enemigo.png",
            "./assets/images/enemigo.png",
            "assets/enemigo.png",
        ];
        
        for alt_path in alternative_paths {
            if std::path::Path::new(&alt_path).exists() {
                println!("  → Intentando ruta alternativa: {}", alt_path);
                match load_texture(&alt_path).await {
                    Ok(texture) => {
                        self.texture = Some(texture);
                        println!("  ✓ Imagen enemigo cargada desde ruta alternativa: {}", alt_path);
                        return;
                    }
                    Err(e) => println!("  ✗ Error en ruta alternativa {}: {}", alt_path, e),
                }
            }
        }
        
        println!("  ✗ No se pudo encontrar enemigo.png en ninguna ubicación");
        println!("  → Se usará un enemigo generado por código");
    }
    
    pub fn activate(&mut self, player: &Player, world_map: &[[u8; 40]; 30]) {
        self.active = true;
        self.alive = true;
        // Encontrar una posición válida lejos del jugador para spawnear
        self.find_spawn_position(player, world_map);
    }
    
    fn find_spawn_position(&mut self, player: &Player, world_map: &[[u8; 40]; 30]) {
        let mut spawn_attempts = 0;
        let max_attempts = 100;
        
        while spawn_attempts < max_attempts {
            // Generar posición aleatoria
            let spawn_x = rand::gen_range(5, 35) as f32 + 0.5;
            let spawn_y = rand::gen_range(5, 25) as f32 + 0.5;
            
            // Verificar que esté en un espacio libre
            let map_x = spawn_x as usize;
            let map_y = spawn_y as usize;
            
            if map_x < 40 && map_y < 30 && 
               (world_map[map_y][map_x] == 0 || world_map[map_y][map_x] == 3) {
                
                // Verificar que esté a una distancia mínima del jugador
                let distance_to_player = ((spawn_x - player.x).powi(2) + (spawn_y - player.y).powi(2)).sqrt();
                
                if distance_to_player > 8.0 && distance_to_player < 20.0 {
                    self.x = spawn_x;
                    self.y = spawn_y;
                    println!("Enemigo spawneado en posición: ({:.1}, {:.1})", spawn_x, spawn_y);
                    return;
                }
            }
            
            spawn_attempts += 1;
        }
        
        // Posición de fallback si no se encuentra una buena posición
        self.x = 35.0;
        self.y = 25.0;
        println!("Enemigo spawneado en posición de fallback");
    }
    
    pub fn update(&mut self, dt: f32, player: &Player, world_map: &[[u8; 40]; 30]) {
        if !self.alive || !self.active {
            return;
        }
        
        // Calcular dirección hacia el jugador
        let dx = player.x - self.x;
        let dy = player.y - self.y;
        let distance = (dx * dx + dy * dy).sqrt();
        
        if distance > 0.1 {
            // Normalizar dirección
            let dir_x = dx / distance;
            let dir_y = dy / distance;
            
            // Hacer al enemigo más rápido cuando está cerca del jugador
            let speed_multiplier = if distance < 5.0 { 1.8 } else if distance < 10.0 { 1.4 } else { 1.0 };
            let current_speed = self.speed * speed_multiplier;
            
            // Calcular nueva posición
            let new_x = self.x + dir_x * current_speed * dt;
            let new_y = self.y + dir_y * current_speed * dt;
            
            // Verificar colisiones con paredes y mover
            if self.can_move_to(new_x, self.y, world_map) {
                self.x = new_x;
            }
            if self.can_move_to(self.x, new_y, world_map) {
                self.y = new_y;
            }
        }
    }
    
    pub fn check_player_collision(&self, player: &Player) -> bool {
        if !self.alive || !self.active {
            return false;
        }
        
        let dx = player.x - self.x;
        let dy = player.y - self.y;
        let distance = (dx * dx + dy * dy).sqrt();
        
        distance <= self.catch_distance
    }
    
    pub fn get_distance_to_player(&self, player: &Player) -> f32 {
        if !self.active {
            return f32::MAX;
        }
        
        let dx = player.x - self.x;
        let dy = player.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }
    
    fn can_move_to(&self, x: f32, y: f32, world_map: &[[u8; 40]; 30]) -> bool {
        let map_x = x as usize;
        let map_y = y as usize;
        
        // Verificar límites
        if map_x >= 40 || map_y >= 30 {
            return false;
        }
        
        // El enemigo puede moverse por espacios vacíos (0) y por la salida (3)
        world_map[map_y][map_x] == 0 || world_map[map_y][map_x] == 3
    }
    
    pub fn reset(&mut self) {
        self.x = 10.0;
        self.y = 10.0;
        self.alive = true;
        self.active = false;
    }
    
    pub fn deactivate(&mut self) {
        self.active = false;
        self.alive = false;
    }
    
    // Nueva función para verificar si hay línea de vista clara al jugador
    fn has_line_of_sight(&self, player: &Player, world_map: &[[u8; 40]; 30]) -> bool {
        let dx = player.x - self.x;
        let dy = player.y - self.y;
        let distance = (dx * dx + dy * dy).sqrt();
        
        if distance == 0.0 {
            return true;
        }
        
        // Normalizar la dirección
        let dir_x = dx / distance;
        let dir_y = dy / distance;
        
        // Usar pasos más pequeños para mayor precisión
        let step_size = 0.1;
        let num_steps = (distance / step_size) as i32;
        
        // Verificar cada punto a lo largo de la línea
        for i in 1..num_steps {
            let check_x = self.x + dir_x * step_size * i as f32;
            let check_y = self.y + dir_y * step_size * i as f32;
            
            let map_x = check_x as usize;
            let map_y = check_y as usize;
            
            // Si está fuera de límites, no hay línea de vista
            if map_x >= 40 || map_y >= 30 {
                return false;
            }
            
            // Si hay una pared en el camino, no hay línea de vista
            let cell_value = world_map[map_y][map_x];
            if cell_value == 1 || cell_value == 2 { // Paredes normales y con sangre
                return false;
            }
        }
        
        true
    }
    
    // Función para renderizar el enemigo en el mundo 3D con oclusión
    pub fn render_in_world(&self, player: &Player, screen_width: f32, screen_height: f32, world_map: &[[u8; 40]; 30]) {
        if !self.alive || !self.active {
            return;
        }
        
        // Calcular posición relativa al jugador
        let dx = self.x - player.x;
        let dy = self.y - player.y;
        let distance = (dx * dx + dy * dy).sqrt();
        
        // No renderizar si está muy lejos
        if distance > 20.0 {
            return;
        }
        
        // Verificar si hay línea de vista clara
        if !self.has_line_of_sight(player, world_map) {
            return; // No renderizar si hay paredes en el camino
        }
        
        // Calcular ángulo desde el jugador
        let angle_to_enemy = dy.atan2(dx);
        let player_angle = player.angle;
        let mut relative_angle = angle_to_enemy - player_angle;
        
        // Normalizar ángulo
        while relative_angle > std::f32::consts::PI {
            relative_angle -= 2.0 * std::f32::consts::PI;
        }
        while relative_angle < -std::f32::consts::PI {
            relative_angle += 2.0 * std::f32::consts::PI;
        }
        
        // Campo de visión
        let fov = std::f32::consts::PI / 3.0;
        
        // Verificar si está dentro del campo de visión
        if relative_angle.abs() <= fov / 2.0 {
            // Calcular posición en pantalla
            let screen_x = screen_width / 2.0 + (relative_angle / (fov / 2.0)) * (screen_width / 2.0);
            
            // Tamaño basado en distancia
            let sprite_size = (screen_height / distance) * self.size;
            let sprite_y = (screen_height - sprite_size) / 2.0;
            
            // Renderizar enemigo
            if let Some(texture) = &self.texture {
                // Efecto de parpadeo cuando está muy cerca
                let alpha = if distance < 3.0 {
                    let flicker = (get_time() * 10.0).sin() * 0.3 + 0.7;
                    flicker.max(0.4)
                } else {
                    1.0
                };
                
                draw_texture_ex(
                    texture,
                    screen_x - sprite_size / 2.0,
                    sprite_y,
                    Color::from_rgba(255, 255, 255, (alpha * 255.0) as u8),
                    DrawTextureParams {
                        dest_size: Some(vec2(sprite_size, sprite_size)),
                        ..Default::default()
                    },
                );
            } else {
                // Enemigo generado por código si no hay textura
                self.draw_generated_enemy(screen_x, sprite_y, sprite_size, distance);
            }
        }
    }
    
    fn draw_generated_enemy(&self, x: f32, y: f32, size: f32, distance: f32) {
        // Crear un enemigo terrorífico usando formas básicas
        let half_size = size / 2.0;
        
        // Efecto de parpadeo cuando está cerca
        let alpha = if distance < 3.0 {
            let flicker = (get_time() * 10.0).sin() * 0.3 + 0.7;
            (flicker.max(0.4) * 255.0) as u8
        } else {
            255
        };
        
        // Cuerpo principal (sombra oscura más grande)
        draw_ellipse(x, y + size * 0.2, size * 0.9, size * 0.8, 0.0, 
            Color::from_rgba(10, 10, 10, alpha));
        
        // Ojos rojos brillantes más grandes y amenazantes
        let eye_size = size * 0.12;
        let eye_glow = (get_time() * 5.0).sin() * 0.2 + 0.8;
        let eye_color = Color::from_rgba(
            (255.0 * eye_glow) as u8, 
            (30.0 * eye_glow) as u8, 
            (30.0 * eye_glow) as u8, 
            alpha
        );
        
        draw_circle(x - half_size * 0.3, y + size * 0.15, eye_size, eye_color);
        draw_circle(x + half_size * 0.3, y + size * 0.15, eye_size, eye_color);
        
        // Pupilas más pequeñas y brillantes
        draw_circle(x - half_size * 0.3, y + size * 0.15, eye_size * 0.3, 
            Color::from_rgba(255, 255, 200, alpha));
        draw_circle(x + half_size * 0.3, y + size * 0.15, eye_size * 0.3, 
            Color::from_rgba(255, 255, 200, alpha));
        
        // Boca amenazante
        draw_ellipse(x, y + size * 0.5, size * 0.4, size * 0.2, 0.0, 
            Color::from_rgba(80, 0, 0, alpha));
        
        // Efecto de flotación más pronunciado
        let float_offset = (get_time() * 4.0).sin() * size as f64 * 0.15;
        draw_circle(x, y + size * 0.85 + float_offset as f32, size * 0.15,
            Color::from_rgba(20, 20, 20, (alpha as f32 * 0.8) as u8));
        
        // Aura siniestra cuando está muy cerca
        if distance < 5.0 {
            let aura_size = size * 1.5;
            let aura_alpha = ((5.0 - distance) / 5.0 * 50.0) as u8;
            draw_circle(x, y + size * 0.4, aura_size, 
                Color::from_rgba(100, 0, 0, aura_alpha));
        }
    }
}