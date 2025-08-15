use macroquad::prelude::*;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    speed: f32,
    turn_speed: f32,
    last_mouse_x: f32,
    was_moving: bool,
    pub moving: bool,
}

impl Player {
    pub fn new(x: f32, y: f32, angle: f32) -> Self {
        Self {
            x,
            y,
            angle,
            speed: 3.0,
            turn_speed: 2.0,
            last_mouse_x: 0.0,
            was_moving: false,
            moving: false,
        }
    }
    
    pub fn update(&mut self, dt: f32, world_map: &[[u8; 20]; 15]) {
        self.was_moving = self.moving;
        self.moving = false;
        
        // Movimiento con WASD
        let mut dx = 0.0;
        let mut dy = 0.0;
        
        if is_key_down(KeyCode::W) {
            dx += self.angle.cos();
            dy += self.angle.sin();
            self.moving = true;
        }
        if is_key_down(KeyCode::S) {
            dx -= self.angle.cos();
            dy -= self.angle.sin();
            self.moving = true;
        }
        if is_key_down(KeyCode::A) {
            dx += (self.angle - std::f32::consts::PI / 2.0).cos();
            dy += (self.angle - std::f32::consts::PI / 2.0).sin();
            self.moving = true;
        }
        if is_key_down(KeyCode::D) {
            dx += (self.angle + std::f32::consts::PI / 2.0).cos();
            dy += (self.angle + std::f32::consts::PI / 2.0).sin();
            self.moving = true;
        }
        
        // Normalizar movimiento diagonal
        if dx != 0.0 && dy != 0.0 {
            let length = (dx * dx + dy * dy).sqrt();
            dx /= length;
            dy /= length;
        }
        
        // Aplicar velocidad y delta time
        dx *= self.speed * dt;
        dy *= self.speed * dt;
        
        // Verificar colisiones y mover
        self.move_with_collision(dx, dy, world_map);
        
        // Rotación con mouse
        let (mouse_x, _) = mouse_position();
        let mouse_delta = mouse_x - self.last_mouse_x;
        
        if mouse_delta.abs() > 0.5 {
            self.angle += mouse_delta * 0.003; // Sensibilidad del mouse
            
            // Mantener el ángulo en el rango 0-2π
            if self.angle < 0.0 {
                self.angle += std::f32::consts::PI * 2.0;
            } else if self.angle >= std::f32::consts::PI * 2.0 {
                self.angle -= std::f32::consts::PI * 2.0;
            }
        }
        
        self.last_mouse_x = mouse_x;
        
        // Rotación con flechas como alternativa
        if is_key_down(KeyCode::Left) {
            self.angle -= self.turn_speed * dt;
        }
        if is_key_down(KeyCode::Right) {
            self.angle += self.turn_speed * dt;
        }
    }
    
    fn move_with_collision(&mut self, dx: f32, dy: f32, world_map: &[[u8; 20]; 15]) {
        let collision_padding = 0.2;
        
        // Intentar moverse en X
        let new_x = self.x + dx;
        if self.can_move_to(new_x, self.y, collision_padding, world_map) {
            self.x = new_x;
        }
        
        // Intentar moverse en Y
        let new_y = self.y + dy;
        if self.can_move_to(self.x, new_y, collision_padding, world_map) {
            self.y = new_y;
        }
    }
    
    fn can_move_to(&self, x: f32, y: f32, padding: f32, world_map: &[[u8; 20]; 15]) -> bool {
        // Verificar las cuatro esquinas del jugador
        let corners = [
            (x - padding, y - padding),
            (x + padding, y - padding),
            (x - padding, y + padding),
            (x + padding, y + padding),
        ];
        
        for (corner_x, corner_y) in corners.iter() {
            let map_x = *corner_x as usize;
            let map_y = *corner_y as usize;
            
            // Verificar límites del mapa
            if map_y >= world_map.len() || map_x >= world_map[0].len() {
                return false;
            }
            
            // Verificar colisión con paredes (pero permitir salida)
            let cell = world_map[map_y][map_x];
            if cell == 1 || cell == 2 { // Paredes normales y con sangre
                return false;
            }
        }
        
        true
    }
    
    pub fn is_moving(&self) -> bool {
        self.moving && !self.was_moving
    }
}