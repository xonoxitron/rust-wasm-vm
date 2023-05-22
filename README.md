# Rust WebAssembly VM

The Rust WASM (WebAssembly) VM (Virtual Machine) project demonstrates the interop between Rust and JavaScript using WebAssembly. It showcases a simple Virtual Machine implemented in Rust, with the ability to `store` and `retrieve` values in `registers`. This documentation provides an overview of the project, explains how to get started, describes the Rust code structure, and provides code samples with explanations.

## Table of Contents

- [Rust WebAssembly VM](#rust-webassembly-vm)
  - [Table of Contents](#table-of-contents)
  - [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
  - [Rust Code](#rust-code)
    - [VirtualMachine Struct](#virtualmachine-struct)
      - [`new`](#new)
      - [`get_register_value`](#get_register_value)
      - [`store_value_in_register`](#store_value_in_register)
  - [JavaScript Code](#javascript-code)
    - [runVirtualMachine Function](#runvirtualmachine-function)
      - [Example:](#example)
  - [License](#license)

## Getting Started

This setup involves creating a Rust library project, implementing the virtual machine logic, building it into a WebAssembly module using `wasm-pack`, and integrating it into an HTML file for execution in the browser.

### Prerequisites

Install the `wasm-pack` module, by running the following command:

```shell
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

To run the project, follow these steps:

1. Install [Rust](https://www.rust-lang.org/tools/install) if you haven't already.
2. Open a terminal and navigate to the project directory.
3. Build the WebAssembly module and JavaScript interop code by running the following command:

   ```bash
   wasm-pack build --target web --out-dir ./dist
   ```

4. Open the `js-wasm-vm-interop.html` file in your web browser to see the output in the browser console.

## Rust Code

The Rust code resides in the `lib.rs` file. It defines a `VirtualMachine` struct with methods to interact with the virtual machine.

### VirtualMachine Struct

The `VirtualMachine` struct represents a simple virtual machine with registers. It has the following methods:

#### `new`

The `new` method creates a new instance of the virtual machine.

```rust
#[wasm_bindgen]
pub fn new() -> Self {
    // Initialize the virtual machine with default register values
    VirtualMachine {
        registers: [0; 16],
    }
}
```

#### `get_register_value`

The `get_register_value` method retrieves the value stored in the specified register.

```rust
#[wasm_bindgen]
pub fn get_register_value(&self, register_index: usize) -> u32 {
    // Retrieve the value from the specified register
    self.registers[register_index]
}
```

#### `store_value_in_register`

The `store_value_in_register` method stores the specified value in the specified register.

```rust
#[wasm_bindgen]
pub fn store_value_in_register(&mut self, register_index: usize, value: u32) {
    // Store the value in the specified register
    self.registers[register_index] = value;
}
```

## JavaScript Code

The JavaScript code resides in the `js-wasm-vm-interop.html` file. It imports and utilizes the Rust WebAssembly module and JavaScript interop code.

### runVirtualMachine Function

The `runVirtualMachine` function initializes the WebAssembly module, creates a new instance of the `VirtualMachine` class, stores a value in register 0, and retrieves the value from register 0.

#### Example

```javascript
import init, { VirtualMachine } from './dist/rust_wasm_vm.js';

async function runVirtualMachine() {
    // Initialize the WebAssembly module
    await init();

    // Create a new instance of the virtual machine
    const vm = new VirtualMachine();

    // Store a value in register 0
    vm.store_value_in_register(0, 42);

    // Retrieve the value from register 0
    const registerValue = vm.get_register_value(0);
    console.log('Value from register 0:', registerValue);
}

runVirtualMachine().catch(console.error);
```

## License

This project is licensed under the [MIT License](LICENSE).

Feel free to modify and enhance the documentation according to your specific needs.
