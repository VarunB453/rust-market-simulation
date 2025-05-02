use std::io;
use std::collections::HashMap;

// Structures
struct Customer {
    name: String,
    surname: String,
    balance: f64,
}

struct Product {
    name: String,
    price: f64,
    stock_quantity: u32,
}

// Implementation
impl Customer {
    fn buy_product(&mut self, product: &mut Product, quantity: u32) -> Result<(), String> {
        let total_cost = product.price * quantity as f64;

        if quantity == 0 {
            return Err("Quantity must be greater than zero".to_string());
        }
        
        if product.stock_quantity < quantity {
            return Err(format!("Not enough stock. Only {} units available", product.stock_quantity));
        }
        
        if self.balance < total_cost {
            return Err(format!("Insufficient funds. Need ${:.2} more", total_cost - self.balance));
        }

        product.stock_quantity -= quantity;
        self.balance -= total_cost;
        Ok(())
    }
}

// UI Module
mod ui {
    use super::*;
    
    pub fn display_products(products: &[Product]) {
        println!("\nAvailable products:");
        for (i, product) in products.iter().enumerate() {
            println!("{}. {} - ${:.2} (Stock: {})", 
                     i+1, product.name, product.price, product.stock_quantity);
        }
    }
    
    pub fn display_customers(customers: &HashMap<String, Customer>) {
        println!("\nAvailable customers:");
        for (id, customer) in customers {
            println!("{}: {} {} (Balance: ${:.2})", 
                     id, customer.name, customer.surname, customer.balance);
        }
    }
    
    pub fn get_input(prompt: &str) -> String {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().to_string()
    }
    
    pub fn select_customer(customers: &HashMap<String, Customer>) -> Option<String> {
        display_customers(customers);
        let customer_id = get_input("\nEnter customer ID (or 'q' to quit):");
        
        if customer_id == "q" {
            return None;
        }
        
        if customers.contains_key(&customer_id) {
            Some(customer_id)
        } else {
            println!("Invalid customer ID!");
            None
        }
    }
    
    pub fn select_product(products: &[Product]) -> Option<usize> {
        display_products(products);
        
        let choice = get_input("\nEnter product number (or 'q' to return):");
        
        if choice == "q" {
            return None;
        }
        
        match choice.parse::<usize>() {
            Ok(num) if num > 0 && num <= products.len() => Some(num - 1),
            _ => {
                println!("Invalid product number!");
                None
            }
        }
    }
    
    pub fn get_quantity(product_name: &str) -> Option<u32> {
        let quantity_str = get_input(&format!("How many units of {} do you want to buy?", product_name));
        
        match quantity_str.parse::<u32>() {
            Ok(num) => Some(num),
            Err(_) => {
                println!("Please enter a valid quantity!");
                None
            }
        }
    }
}

fn main() {
    // Initialize customers
    let mut customers = HashMap::new();
    customers.insert("A1".to_string(), Customer {
        name: String::from("Alice"),
        surname: String::from("Smith"),
        balance: 100.0,
    });
    customers.insert("B2".to_string(), Customer {
        name: String::from("Bob"),
        surname: String::from("Johnson"),
        balance: 50.0,
    });
    customers.insert("C3".to_string(), Customer {
        name: String::from("Charlie"),
        surname: String::from("Brown"),
        balance: 200.0,
    });

    // Initialize products
    let mut products = vec![
        Product {
            name: String::from("Book"),
            price: 15.0,
            stock_quantity: 10,
        },
        Product {
            name: String::from("Pen"),
            price: 2.5,
            stock_quantity: 50,
        },
        Product {
            name: String::from("Notebook"),
            price: 5.0,
            stock_quantity: 25,
        },
    ];

    println!("Welcome to the Market Simulation!");
    
    loop {
        // Customer selection
        let customer_id = match ui::select_customer(&customers) {
            Some(id) => id,
            None => break,
        };
        
        let customer = customers.get(&customer_id).unwrap();
        println!("\nLogged in as: {} {} (Balance: ${:.2})", 
                 customer.name, customer.surname, customer.balance);
        
        // Shopping loop for current customer
        loop {
            let product_idx = match ui::select_product(&products) {
                Some(idx) => idx,
                None => break,
            };
            
            let quantity = match ui::get_quantity(&products[product_idx].name) {
                Some(qty) => qty,
                None => continue,
            };
            
            // Need to get mutable references to both customer and product
            let customer = customers.get_mut(&customer_id).unwrap();
            let product = &mut products[product_idx];
            
            match customer.buy_product(product, quantity) {
                Ok(_) => println!("Purchase successful! New balance: ${:.2}", customer.balance),
                Err(msg) => println!("Purchase failed: {}", msg),
            }
            
            println!("\nContinue shopping? (y/n)");
            let continue_shopping = ui::get_input("");
            if continue_shopping.to_lowercase() != "y" {
                break;
            }
        }
    }
    
    println!("Thank you for shopping with us!");
}
