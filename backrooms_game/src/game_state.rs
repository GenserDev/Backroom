extern crate rand;
use rand::{Rng, seq::SliceRandom};

#[derive(PartialEq)]
pub enum Screen {
    Menu,
    Game,
}

pub struct GameState {
    pub current_screen: Screen,
    pub world_map: [[u8; 20]; 15], // Corregido: 20 columnas, 15 filas
    pub escaped: bool,
    pub victory_sound_played: bool,
}

impl GameState {
    pub fn new() -> Self {
        let mut game_state = Self {
            current_screen: Screen::Menu,
            world_map: [[0; 20]; 15], // Corregido
            escaped: false,
            victory_sound_played: false,
        };
        
        game_state.generate_world();
        game_state
    }
    
    pub fn reset(&mut self) {
        self.current_screen = Screen::Menu;
        self.escaped = false;
        self.victory_sound_played = false;
        self.generate_world();
    }
    
    fn generate_world(&mut self) {
        let mut rng = rand::thread_rng();
        
        // Inicializar con paredes
        for row in &mut self.world_map {
            for cell in row {
                *cell = 1;
            }
        }
        
        // Crear laberinto usando algoritmo de generación simple
        self.carve_maze(&mut rng);
        
        // Agregar algunas paredes con sangre aleatoriamente
        self.add_bloody_walls(&mut rng);
        
        // Colocar la salida
        self.place_exit(&mut rng);
        
        // Asegurar que el punto de inicio esté libre
        self.world_map[1][1] = 0;
        self.world_map[1][2] = 0;
        self.world_map[2][1] = 0;
    }
    
    fn carve_maze(&mut self, rng: &mut impl Rng) {
        // Algoritmo simple de generación de laberinto
        let mut stack = Vec::new();
        let mut visited = [[false; 20]; 15]; // Corregido
        
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
        
        // Crear algunos pasillos adicionales para hacer el laberinto más interesante
        for _ in 0..10 { // Reducido para mapas más pequeños
            let x = rng.gen_range(1..19); // Ajustado para 20 columnas
            let y = rng.gen_range(1..14); // Ajustado para 15 filas
            if x % 2 == 1 && y % 2 == 1 {
                self.world_map[y][x] = 0;
                
                // Conectar con un pasillo vecino ocasionalmente
                if rng.gen_bool(0.3) {
                    let directions = [(0, 2), (2, 0), (0, -2), (-2, 0)];
                    if let Some(&(dx, dy)) = directions.choose(rng) {
                        let new_x = x as i32 + dx;
                        let new_y = y as i32 + dy;
                        if new_x >= 1 && new_x < 19 && new_y >= 1 && new_y < 14 {
                            let bridge_x = x as i32 + dx / 2;
                            let bridge_y = y as i32 + dy / 2;
                            self.world_map[bridge_y as usize][bridge_x as usize] = 0;
                        }
                    }
                }
            }
        }
    }
    
    fn get_unvisited_neighbors(&self, x: usize, y: usize, visited: &[[bool; 20]; 15]) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let directions = [(0, 2), (2, 0), (0, -2), (-2, 0)];
        
        for &(dx, dy) in &directions {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;
            
            if new_x >= 1 && new_x < 19 && new_y >= 1 && new_y < 14 { // Ajustado
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
        for y in 0..15 { // Ajustado
            for x in 0..20 { // Ajustado
                if self.world_map[y][x] == 1 && rng.gen_bool(0.15) {
                    // Solo agregar sangre si hay al menos un espacio vacío adyacente
                    let adjacent_empty = [
                        (x.wrapping_sub(1), y),
                        (x + 1, y),
                        (x, y.wrapping_sub(1)),
                        (x, y + 1),
                    ].iter().any(|&(ax, ay)| {
                        ax < 20 && ay < 15 && self.world_map[ay][ax] == 0 // Ajustado
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
        let start_x = 1.5;
        let start_y = 1.5;
        
        for y in 1..14 { // Ajustado
            for x in 1..19 { // Ajustado
                if self.world_map[y][x] == 0 {
                    let distance = ((x as f32 - start_x).powi(2) + (y as f32 - start_y).powi(2)).sqrt();
                    if distance > 6.0 { // Reducido para mapas más pequeños
                        far_positions.push((x, y));
                    }
                }
            }
        }
        
        // Colocar la salida en una posición lejana aleatoria
        if let Some(&(exit_x, exit_y)) = far_positions.choose(rng) {
            self.world_map[exit_y][exit_x] = 3; // Salida
        } else {
            // Fallback: colocar en una esquina
            self.world_map[13][18] = 3; // Ajustado
        }
    }
}