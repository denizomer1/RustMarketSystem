// Define the Customer structure
struct Customer {
    name: String,
    surname: String,
    balance: f64, // Balance in the customer's account
}

// Define the Product structure
struct Product {
    name: String,
    price: f64, // Price per unit of the product
    stock: u32, // Number of units available in stock
}

// Implement functions for the Customer struct
impl Customer {
    // Function for purchasing a product
    fn buy_product(&mut self, product: &mut Product, quantity: u32) -> bool {
        // Check if the product is available in sufficient quantity
        if product.stock < quantity {
            println!("Not enough stock for {}.", product.name);
            return false;
        }

        // Calculate the total price for the purchase
        let total_cost = product.price * quantity as f64;

        // Check if the customer has enough balance
        if self.balance < total_cost {
            println!("{} {} does not have enough balance to buy {} units of {}.", self.name, self.surname, quantity, product.name);
            return false;
        }

        // Deduct the price from the customer's balance and update the stock
        self.balance -= total_cost;
        product.stock -= quantity;
        println!("{} {} successfully bought {} units of {} for ${:.2}.", self.name, self.surname, quantity, product.name, total_cost);
        true
    }
}

// Main function
fn main() {
    // Create sample customers
    let mut customer1 = Customer {
        name: String::from("Alice"),
        surname: String::from("Johnson"),
        balance: 100.0, // Initial balance is $100.00
    };

    let mut customer2 = Customer {
        name: String::from("Bob"),
        surname: String::from("Smith"),
        balance: 50.0, // Initial balance is $50.00
    };

    // Create a sample product
    let mut product = Product {
        name: String::from("Laptop"),
        price: 30.0, // Price per unit is $30.00
        stock: 10,   // There are 10 units in stock
    };

    // Customer 1 tries to buy 3 units of the product
    println!("{} is trying to buy a product...", customer1.name);
    if customer1.buy_product(&mut product, 3) {
        println!("{} successfully purchased the product.", customer1.name);
    } else {
        println!("{} couldn't purchase the product.", customer1.name);
    }

    // Customer 2 tries to buy 8 units of the product
    println!("\n{} is trying to buy a product...", customer2.name);
    if customer2.buy_product(&mut product, 8) {
        println!("{} successfully purchased the product.", customer2.name);
    } else {
        println!("{} couldn't purchase the product.", customer2.name);
    }

    // Final state of the product and customers
    println!("\nFinal product stock: {}", product.stock);
    println!("Final balance for {}: ${:.2}", customer1.name, customer1.balance);
    println!("Final balance for {}: ${:.2}", customer2.name, customer2.balance);
}
