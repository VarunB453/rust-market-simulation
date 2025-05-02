# Rust Market Simulation : 

A simple command-line market simulation written in Rust that demonstrates basic concepts of the language including structs, modules, error handling, and user input processing.

---

## Features

- Multiple customer accounts with unique IDs- Product inventory with prices and stock quantities
- Purchase validation (sufficient funds, available stock)- Detailed error messages
- Interactive command-line interface- Modular code organization

---

## Project Structure
- `src/main.rs` - Core application logic, data structures, and main function
- `src/ui.rs` - User interface functions for display and input handling

---

## How to Run
1. Make sure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/)
2. Clone this repository
3. Navigate to the project directory4. Run the application:
```bash

cargo run```

## Usage
1. Select a customer by entering their ID (or 'q' to quit)
2. Browse available products
3. Select a product by number
4. Enter the quantity you wish to purchase
5. Continue shopping or return to customer selection
6. Quit the application when done

## Sample Customer IDs
- A1: Alice Smith (Balance: $100.00)
- B2: Bob Johnson (Balance: $50.00)
- C3: Charlie Brown (Balance: $200.00)

## Available Products
1. Book - $15.00 (Stock: 10)
2. Pen - $2.50 (Stock: 50)
3. Notebook - $5.00 (Stock: 25)

## Learning Outcomes
This project demonstrates:

- Struct definitions and implementations- Result type for error handling
- Module organization- HashMap for data storage
- User input processing- String formatting
- Mutable references- Option and Result types

## Future Improvements
- Persistent storage for customer and product data
- Admin interface for managing inventory- Transaction history
- Shopping cart functionality
- Unit tests