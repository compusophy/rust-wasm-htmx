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

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() {
    console_log!("Rust WebAssembly module loaded!");
} 