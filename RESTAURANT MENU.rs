use std::io;

fn main() {
    println!("========================================");
    println!("          PROJECT: THE RESTAURANT MENU");
    println!("========================================");
    println!(" [P] Poundo Yam / Edinkaiko Soup  - N3,200");
    println!(" [F] Fried Rice & Chicken         - N3,000");
    println!(" [A] Amala & Ewedu Soup           - N2,500");
    println!(" [E] Eba & Egusi Soup             - N2,000");
    println!(" [W] White Rice & Stew            - N2,500");
    println!("========================================\n");

    println!("Enter the letter corresponding to your food choice (P, F, A, E, W):");
    let mut food_choice = String::new();
    io::stdin()
        .read_line(&mut food_choice)
        .expect("Failed to read input");
    
    let food_choice = food_choice.trim().to_uppercase();

    println!("Enter the quantity you wish to order:");
    let mut quantity_input = String::new();
    io::stdin()
        .read_line(&mut quantity_input)
        .expect("Failed to read input");
    
    let quantity: u32 = match quantity_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number for quantity.");
            return;
        }
    };

    let unit_price = match food_choice.as_str() {
        "P" => 3200.0,
        "F" => 3000.0,
        "A" => 2500.0,
        "E" => 2000.0,
        "W" => 2500.0,
        _ => {
            println!("Invalid food choice selection. Please restart and choose a valid letter.");
            return;
        }
    };

    let mut total_charge = unit_price * (quantity as f64);
    let mut discount_applied = false;

    if total_charge > 10_000.0 {
        total_charge *= 0.95; // Apply 5% discount (pay 95%)
        discount_applied = true;
    }

    println!("\n----------------------------------------");
    println!("             ORDER SUMMARY");
    println!("----------------------------------------");
    println!("Quantity ordered: {}", quantity);
    if discount_applied {
        println!("Discount: 5% applied for orders over N10,000!");
    }
    println!("Final Total Charge: N{:.2}", total_charge);
    println!("----------------------------------------");
}