
use std::collections::HashMap;
use std::io;

#[derive(Debug)]
pub struct Member {
    name: String,
    savings: f64,
}

pub struct SavingsGroup {
  members: HashMap<String, Member>,
}

impl SavingsGroup {

  pub fn new() -> Self { SavingsGroup { members: HashMap::new() } }
  pub fn add_member(&mut self, name: String) {
    if !self.members.contains_key(&name) {
        self.members.insert(name.clone(), Member { name: name.clone(), savings: 0.0 });
        println!("Member {} added.", name);
    } else {
        println!("Member {} already exists.", name);
    }
  }
  pub fn remove_member(&mut self, name: String) {
    if let Some(member) = self.members.get(&name) {
        if member.savings > 0.0 {
            println!("Cannot remove {}. Balance: {}", name, member.savings);
        } else {
            self.members.remove(&name);
            println!("Removed {}.", name);
        }
    } else {
        println!("No such member: {}", name);
    }
  
}
pub fn record_savings(&mut self, name: String, amount: f64) {
  if let Some(member) = self.members.get_mut(&name) {
      member.savings += amount;
      println!("{} saved {}. Total: {}", name, amount, member.savings);
  } else {
      println!("No such member: {}", name);
  }
}

pub fn update_member_name(&mut self, old_name: String, new_name: String) {
  if let Some(member) = self.members.remove(&old_name) {
      if self.members.contains_key(&new_name) {
          println!("Member with the name {} already exists.", new_name);
          self.members.insert(old_name, member);
      } else {
          self.members.insert(new_name.clone(), Member { name: new_name.clone(), savings: member.savings });
          println!("Member name updated from {} to {}.", old_name, new_name);
      }
  } else {
      println!("No such member: {}", old_name);
  }
}

pub fn update_savings(&mut self, name: String) {
  if let Some(member) = self.members.get_mut(&name) {
      println!("Enter new savings for {} (current: {}):", name, member.savings);
      let mut input = String::new();
      io::stdin().read_line(&mut input).unwrap();
      if let Ok(new_savings) = input.trim().parse::<f64>() {
          member.savings = new_savings;
          println!("Updated savings for {} to {}.", name, new_savings);
      } else {
          println!("Invalid amount.");
      }
  } else {
      println!("No such member: {}", name);
  }
}
pub fn view_all(&self) {
  if self.members.is_empty() {
      println!("No members.");
  } else {
      self.members.values().for_each(|m| println!("{}: {}", m.name, m.savings));
  }
}
pub fn view_savings(&self, name: String) {
  if let Some(member) = self.members.get(&name) {
      println!("{}: {}", name, member.savings);
  } else {
      println!("No such member: {}", name);
  }
}
}
