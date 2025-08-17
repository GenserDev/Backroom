extern crate rand;
use rand::{Rng, seq::SliceRandom};

#[derive(PartialEq)]
pub enum Screen {
    Menu,
    Game,
}

pub struct GameState {
    pub current_screen: Screen,
    pub world_map: [[u8; 40]; 30],
    pub escaped: bool,
    pub victory_sound_played: bool,
    pub screamer_triggered: bool,
    pub screamer_timer: f32,
    pub screamer_active: bool,
    pub exit_position: (usize, usize),
    // Nuevos campos para el screamer aleatorio
    pub random_screamer_triggered: bool,
    pub random_screamer_active: bool,
    pub random_screamer_timer: f32,
    pub random_screamer_cooldown: f32,
    pub game_timer: f32,
}

impl GameState {
    pub fn new() -> Self {
        let mut game_state = Self {
            current_screen: Screen::Menu,
            world_map: [[0; 40]; 30],
            escaped: false,
            victory_sound_played: false,
            screamer_triggered: false,
            screamer_timer: 0.0,
            screamer_active: false,
            exit_position: (0, 0),
            random_screamer_triggered: false,
            random_screamer_active: false,
            random_screamer_timer: 0.0,
            random_screamer_cooldown: 0.0,
            game_timer: 0.0,
        };
        
        game_state.generate_world();
        game_state
    }
    
    pub fn reset(&mut self) {
        self.current_screen = Screen::Menu;
        self.escaped = false;
        self.victory_sound_played = false;
        self.screamer_triggered = false;
        self.screamer_timer = 0.0;
        self.screamer_active = false;
        self.random_screamer_triggered = false;
        self.random_screamer_active = false;
        self.random_screamer_timer = 0.0;
        self.random_screamer_cooldown = 0.0;
        self.game_timer = 0.0;
        self.generate_world();
    }
    
    pub fn update(&mut self, dt: f32) {
        self.game_timer += dt;
        
        // Actualizar screamer de salida
        self.update_screamer(dt);
        
        // Actualizar screamer aleatorio
        self.update_random_screamer(dt);
    }
    
    pub fn update_screamer(&mut self, dt: f32) {
        if self.screamer_active {
            self.screamer_timer += dt;
            if self.screamer_timer >= 3.0 {
                self.screamer_active = false;
                self.screamer_timer = 0.0;
            }
        }
    }
    
    pub fn update_random_screamer(&mut self, dt: f32) {
        // Actualizar cooldown del screamer aleatorio
        if self.random_screamer_cooldown > 0.0 {
            self.random_screamer_cooldown -= dt;
        }
        
        // Actualizar timer del screamer activo
        if self.random_screamer_active {
            self.random_screamer_timer += dt;
            if self.random_screamer_timer >= 2.5 { // Duración ligeramente diferente
                self.random_screamer_active = false;
                self.random_screamer_timer = 0.0;
                // Establecer un cooldown después del screamer
                self.random_screamer_cooldown = 45.0; // 45 segundos de cooldown
            }
        }
    }
    
    pub fn check_screamer_distance(&mut self, player_x: f32, player_y: f32) -> bool {
        if !self.screamer_triggered {
            let exit_x = self.exit_position.0 as f32;
            let exit_y = self.exit_position.1 as f32;
            let distance = ((player_x - exit_x).powi(2) + (player_y - exit_y).powi(2)).sqrt();
            
            if distance <= 3.0 {
                self.screamer_triggered = true;
                self.screamer_active = true;
                self.screamer_timer = 0.0;
                return true;
            }
        }
        false
    }
    
    pub fn check_random_screamer(&mut self) -> bool {
        // Solo puede activarse si no hay cooldown y no está ya activo
        if self.random_screamer_cooldown <= 0.0 && !self.random_screamer_active && 
           !self.screamer_active && self.game_timer > 20.0 { // Esperar al menos 20 segundos
            
            // Probabilidad muy baja por frame (aproximadamente cada 30-60 segundos en promedio)
            let mut rng = rand::thread_rng();
            if rng.gen_bool(0.0001) { // 0.01% de probabilidad por frame
                self.random_screamer_active = true;
                self.random_screamer_timer = 0.0;
                return true;
            }
        }
        false
    }
    
    fn generate_world(&mut self) {
        let mut rng = rand::thread_rng();
        
        // Inicializar con paredes
        for row in &mut self.world_map {
            for cell in row {
                *cell = 1;
            }
        }
        
        // Crear laberinto usando algoritmo de generación mejorado
        self.carve_maze(&mut rng);
        
        // Agregar algunas paredes con sangre aleatoriamente
        self.add_bloody_walls(&mut rng);
        
        // Colocar la salida
        self.place_exit(&mut rng);
        
        // Asegurar que el punto de inicio esté libre (área más grande)
        for y in 1..4 {
            for x in 1..4 {
                if y < 30 && x < 40 {
                    self.world_map[y][x] = 0;
                }
            }
        }
    }
    
    fn carve_maze(&mut self, rng: &mut impl Rng) {
        // Algoritmo mejorado de generación de laberinto para mapas grandes
        let mut stack = Vec::new();
        let mut visited = [[false; 40]; 30];
        
        let start_x = 1;
        let start_y = 1;
        
        stack.push((start_x, start_y));
        visited[start_y][start_x] = true;
        self.world_map[start_y][start_x] = 0;
        
        while let Some((x, y)) = stack.pop() {
            let neighbors = self.get_unvisited_neighbors(x, y, &visited);
            
            if !neighbors.is_empty() {
                stack.push((x, y));
                
                let &(nx, ny) = neighbors.choose(rng).unwrap();
                
                // Carvar el camino hacia el vecino
                let wall_x = (x + nx) / 2;
                let wall_y = (y + ny) / 2;
                
                self.world_map[wall_y][wall_x] = 0;
                self.world_map[ny][nx] = 0;
                visited[ny][nx] = true;
                
                stack.push((nx, ny));
            }
        }
        
        // Crear pasillos adicionales para hacer el laberinto más interesante
        for _ in 0..50 { // Más pasillos para el mapa grande
            let x = rng.gen_range(1..39); // Ajustado para 40 columnas
            let y = rng.gen_range(1..29); // Ajustado para 30 filas
            if x % 2 == 1 && y % 2 == 1 {
                self.world_map[y][x] = 0;
                
                // Conectar con un pasillo vecino ocasionalmente
                if rng.gen_bool(0.3) {
                    let directions = [(0, 2), (2, 0), (0, -2), (-2, 0)];
                    if let Some(&(dx, dy)) = directions.choose(rng) {
                        let new_x = x as i32 + dx;
                        let new_y = y as i32 + dy;
                        if new_x >= 1 && new_x < 39 && new_y >= 1 && new_y < 29 {
                            let bridge_x = x as i32 + dx / 2;
                            let bridge_y = y as i32 + dy / 2;
                            self.world_map[bridge_y as usize][bridge_x as usize] = 0;
                        }
                    }
                }
            }
        }
        
        // Crear algunas áreas abiertas para hacer el juego más interesante
        self.create_open_areas(rng);
    }
    
    fn create_open_areas(&mut self, rng: &mut impl Rng) {
        // Crear 3-5 áreas abiertas pequeñas en el mapa
        let num_areas = rng.gen_range(3..6);
        
        for _ in 0..num_areas {
            let center_x = rng.gen_range(5..35);
            let center_y = rng.gen_range(5..25);
            let size = rng.gen_range(2..4);
            
            // Crear área abierta
            for dy in -(size as i32)..=(size as i32) {
                for dx in -(size as i32)..=(size as i32) {
                    let x = (center_x as i32 + dx) as usize;
                    let y = (center_y as i32 + dy) as usize;
                    
                    if x < 40 && y < 30 {
                        // Crear área circular
                        let distance = (dx * dx + dy * dy) as f32;
                        if distance <= (size * size) as f32 {
                            self.world_map[y][x] = 0;
                        }
                    }
                }
            }
        }
    }
    
    fn get_unvisited_neighbors(&self, x: usize, y: usize, visited: &[[bool; 40]; 30]) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let directions = [(0, 2), (2, 0), (0, -2), (-2, 0)];
        
        for &(dx, dy) in &directions {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;
            
            if new_x >= 1 && new_x < 39 && new_y >= 1 && new_y < 29 { // Ajustado para mapa 40x30
                let nx = new_x as usize;
                let ny = new_y as usize;
                
                if !visited[ny][nx] {
                    neighbors.push((nx, ny));
                }
            }
        }
        
        neighbors
    }
    
    fn add_bloody_walls(&mut self, rng: &mut impl Rng) {
        // Convertir algunas paredes normales en paredes con sangre
        for y in 0..30 { // Ajustado para mapa 30 filas
            for x in 0..40 { // Ajustado para mapa 40 columnas
                if self.world_map[y][x] == 1 && rng.gen_bool(0.18) { // Ligeramente más probabilidad
                    // Solo agregar sangre si hay al menos un espacio vacío adyacente
                    let adjacent_empty = [
                        (x.wrapping_sub(1), y),
                        (x + 1, y),
                        (x, y.wrapping_sub(1)),
                        (x, y + 1),
                    ].iter().any(|&(ax, ay)| {
                        ax < 40 && ay < 30 && self.world_map[ay][ax] == 0 // Ajustado
                    });
                    
                    if adjacent_empty {
                        self.world_map[y][x] = 2; // Pared con sangre
                    }
                }
            }
        }
    }
    
    fn place_exit(&mut self, rng: &mut impl Rng) {
        // Encontrar posiciones vacías lejas del inicio
        let mut far_positions = Vec::new();
        let start_x = 2.0;
        let start_y = 2.0;
        
        for y in 1..29 { // Ajustado para mapa 30 filas
            for x in 1..39 { // Ajustado para mapa 40 columnas
                if self.world_map[y][x] == 0 {
                    let distance = ((x as f32 - start_x).powi(2) + (y as f32 - start_y).powi(2)).sqrt();
                    if distance > 15.0 { // Distancia mínima mayor para mapa grande
                        far_positions.push((x, y));
                    }
                }
            }
        }
        
        // Colocar la salida en una posición lejana aleatoria
        if let Some(&(exit_x, exit_y)) = far_positions.choose(rng) {
            self.world_map[exit_y][exit_x] = 3; // Salida
            self.exit_position = (exit_x, exit_y);
        } else {
            // Fallback: colocar en una esquina lejana
            self.world_map[28][37] = 3; // Ajustado para mapa 40x30
            self.exit_position = (37, 28);
        }
    }
}