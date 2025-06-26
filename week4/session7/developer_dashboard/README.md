# Developer Dashboard – Contributors Manager

This is a simple Rust program that simulates managing a list of project contributors.

## Features

- Creates a list of contributor names using `Vec<String>`
- Adds a status tag ("Active") to each contributor
- Previews the first 3 contributors
- Displays the total number of contributors
- Adds a new contributor using `.push()`
- Removes the last contributor using `.pop()`
- Uses a generic function to print any list (e.g., `Vec<String>`, `Vec<i32>`)

## Running the Project

1. Install Rust: https://www.rust-lang.org/tools/install
2. Clone this repository:
   ```bash
   git clone https://github.com/polkadotkisumubootcamp/polkadot-kisumu-workshop.git
   
   ```
3. Switch to branch cynthia
    ```bash
    git checkout cynthia
    ```
4. Navigate to the project
     ```bash
    cd week4
    cd session7
    cd developer_dashboard
    cd src
     ```
5. Run the project
    ```bash
    cargo run
    ```
6. Expected results
    ```bash

    First 3 contributors preview:
    Active: Cynthia
    Active: Brian
    Active: Aisha

    Total contributors: 6

    Contributors before push/pop:
    Active: Cynthia
    Active: Brian
    Active: Aisha
    Active: Kelvin
    Active: Grace
    Active: Liam

    Contributors after push 
    Active: Cynthia
    Active: Brian
    Active: Aisha
    Active: Kelvin
    Active: Grace
    Active: Liam
    Active: Zainab

    Contributors after pop:
    Active: Cynthia
    Active: Brian
    Active: Aisha
    Active: Kelvin
    Active: Grace
    Active: Liam
    ```