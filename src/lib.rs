use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct VirtualMachine {
    registers: [u32; 16],
}

#[wasm_bindgen]
impl VirtualMachine {
    /// Creates a new instance of the virtual machine.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        VirtualMachine { registers: [0; 16] }
    }

    /// Retrieves the value stored in the specified register.
    ///
    /// # Arguments
    ///
    /// * `register_index` - The index of the register.
    ///
    /// # Returns
    ///
    /// The value stored in the register.
    #[wasm_bindgen]
    pub fn get_register_value(&self, register_index: usize) -> u32 {
        self.registers[register_index]
    }

    /// Stores the specified value in the specified register.
    ///
    /// # Arguments
    ///
    /// * `register_index` - The index of the register.
    /// * `value` - The value to store in the register.
    #[wasm_bindgen]
    pub fn store_value_in_register(&mut self, register_index: usize, value: u32) {
        self.registers[register_index] = value;
    }
}
