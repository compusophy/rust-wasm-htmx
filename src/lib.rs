use wasm_bindgen::prelude::*;

// Import the `console.log` function from the `console` module
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Define a macro to make it easier to call console.log
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    console_log!("Adding {} + {} = {}", a, b, a + b);
    a + b
}

#[wasm_bindgen]
pub fn factorial(n: i32) -> i32 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

#[wasm_bindgen]
pub struct Calculator {
    value: f64,
}

#[wasm_bindgen]
impl Calculator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Calculator {
        console_log!("Calculator created");
        Calculator { value: 0.0 }
    }
    
    #[wasm_bindgen(getter)]
    pub fn value(&self) -> f64 {
        self.value
    }
    
    #[wasm_bindgen]
    pub fn add(&mut self, num: f64) -> f64 {
        self.value += num;
        console_log!("Added {}, new value: {}", num, self.value);
        self.value
    }
    
    #[wasm_bindgen]
    pub fn multiply(&mut self, num: f64) -> f64 {
        self.value *= num;
        console_log!("Multiplied by {}, new value: {}", num, self.value);
        self.value
    }
    
    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.value = 0.0;
        console_log!("Calculator reset");
    }
}

// Grid structure for the 32x32 game
#[wasm_bindgen]
pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<u8>,
}

#[wasm_bindgen]
impl Grid {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Grid {
        let width = 32;
        let height = 32;
        let cells = vec![0; width * height];
        
        console_log!("Created new 32x32 grid");
        Grid { width, height, cells }
    }
    
    #[wasm_bindgen(getter)]
    pub fn width(&self) -> usize {
        self.width
    }
    
    #[wasm_bindgen(getter)]
    pub fn height(&self) -> usize {
        self.height
    }
    
    #[wasm_bindgen]
    pub fn get_index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }
    
    #[wasm_bindgen]
    pub fn get_cell(&self, row: usize, col: usize) -> u8 {
        let idx = self.get_index(row, col);
        self.cells[idx]
    }
    
    #[wasm_bindgen]
    pub fn set_cell(&mut self, row: usize, col: usize, value: u8) {
        let idx = self.get_index(row, col);
        self.cells[idx] = value;
        console_log!("Set cell ({}, {}) to {}", row, col, value);
    }
    
    #[wasm_bindgen]
    pub fn toggle_cell(&mut self, row: usize, col: usize) -> u8 {
        let idx = self.get_index(row, col);
        self.cells[idx] = if self.cells[idx] == 0 { 1 } else { 0 };
        console_log!("Toggled cell ({}, {}) to {}", row, col, self.cells[idx]);
        self.cells[idx]
    }
    
    #[wasm_bindgen]
    pub fn clear(&mut self) {
        self.cells = vec![0; self.width * self.height];
        console_log!("Grid cleared");
    }
    
    #[wasm_bindgen]
    pub fn randomize(&mut self) {
        use js_sys::Math;
        
        for i in 0..self.cells.len() {
            self.cells[i] = if Math::random() > 0.5 { 1 } else { 0 };
        }
        console_log!("Grid randomized");
    }
    
    #[wasm_bindgen]
    pub fn get_cells_ptr(&self) -> *const u8 {
        self.cells.as_ptr()
    }
    
    #[wasm_bindgen]
    pub fn count_active_cells(&self) -> usize {
        self.cells.iter().filter(|&&cell| cell == 1).count()
    }
    
    #[wasm_bindgen]
    pub fn fill_pattern(&mut self, pattern: &str) {
        match pattern {
            "checkerboard" => {
                for row in 0..self.height {
                    for col in 0..self.width {
                        let idx = self.get_index(row, col);
                        self.cells[idx] = if (row + col) % 2 == 0 { 1 } else { 0 };
                    }
                }
            },
            "cross" => {
                self.clear();
                let mid = self.width / 2;
                for i in 0..self.width {
                    self.set_cell(mid, i, 1);
                    self.set_cell(i, mid, 1);
                }
            },
            "border" => {
                self.clear();
                for i in 0..self.width {
                    self.set_cell(0, i, 1);
                    self.set_cell(self.height - 1, i, 1);
                    self.set_cell(i, 0, 1);
                    self.set_cell(i, self.width - 1, 1);
                }
            },
            _ => {
                console_log!("Unknown pattern: {}", pattern);
            }
        }
        console_log!("Applied pattern: {}", pattern);
    }
}

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() {
    console_log!("Rust WebAssembly module loaded!");
} 