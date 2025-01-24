pub mod members_config;
use std::io;

fn main() {
    let mut group = members_config::SavingsGroup::new();
    loop {
        println!("\n1. Add Member\n2. Record Savings\n3. View Savings\n4. View All Savings\n5. Remove Member\n6. Update Savings\n7. View All Members\n8. Update Member Name\n9. Exit");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim().parse::<u32>().unwrap_or(0) {
            1 => { 
                println!("Enter name: ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                group.add_member(name.trim().to_string());
            }
            2 => { 
                println!("Enter name: ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                println!("Enter amount: ");
                let mut amount = String::new();
                io::stdin().read_line(&mut amount).unwrap();
                if let Ok(amount) = amount.trim().parse::<f64>() {
                    group.record_savings(name.trim().to_string(), amount);
                } else {
                    println!("Invalid amount.");
                }
            }
            3 => { 
                println!("Enter name: ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                group.view_savings(name.trim().to_string());
            }
            4 => group.view_all(),
            5 => { 
                println!("Enter name to remove: ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                group.remove_member(name.trim().to_string());
            }
            6 => { 
                println!("Enter name to update: ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                group.update_savings(name.trim().to_string());
            }
            7 => group.view_all(),
            8 => {
                println!("Enter current member name: ");
                let mut old_name = String::new();
                io::stdin().read_line(&mut old_name).unwrap();
                println!("Enter new member name: ");
                let mut new_name = String::new();
                io::stdin().read_line(&mut new_name).unwrap();
                group.update_member_name(old_name.trim().to_string(), new_name.trim().to_string());
            }
            9 => break,
            _ => println!("Invalid choice."),
        }
    }
}
