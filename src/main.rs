enum ItemType {
    Weapon,
    Armor,
    Potion,
}
struct Item {
    name: String,
    item_type: ItemType,
    value: u32,
}
struct Inventory {
    items: Vec<Item>,
    max_slots: usize,
}
impl Inventory {
    fn is_full(&self) -> bool {
        self.items.len() >= self.max_slots
    }
    fn add_item(&mut self, item: Item) -> Result<(), String> {
       if self.is_full() {
          Err("Inventory is full".to_string())
    } else {
        self.items.push(item);
        Ok(())
    }
  }
    fn remove_item(&mut self, index: usize) -> Result<Item, String> {
       if index >= self.items.len() {
          Err("Invalid index".to_string())
    } else {
        Ok(self.items.remove(index))
    }
  }
    fn get_item(&self, index: usize) -> Option<&Item> {
       self.items.get(index)
 }
    fn total_value(&self) -> u32 {
       self.items.iter().map(|item| item.value).sum()
 }
  
}

fn main() {
    println!("Inventory system starting...");
    let mut inventory = Inventory {
        items: Vec::new(),
        max_slots: 5,
    };

    let items = vec![
        Item {
            name: "Sword".to_string(),
            item_type: ItemType::Weapon,
            value: 150,
        },
        Item {
            name: "Shield".to_string(),
            item_type: ItemType::Armor,
            value: 100,
        },
        Item {
            name: "Helmet".to_string(),
            item_type: ItemType::Armor,
            value: 80,
        },
        Item {
            name: "Health Potion".to_string(),
            item_type: ItemType::Potion,
            value: 50,
        },
        Item {
            name: "Mana Potion".to_string(),
            item_type: ItemType::Potion,
            value: 60,
        },
    ];

    for item in items {
        inventory.add_item(item).unwrap();
    }

    let extra_item = Item {
        name: "Extra Sword".to_string(),
        item_type: ItemType::Weapon,
        value: 200,
    };

    if let Err(err) = inventory.add_item(extra_item) {
        println!("Error adding item: {}", err);
    }

    let removed = inventory.remove_item(2).unwrap();
    println!("Removed item: {}", removed.name);

    let total = inventory.total_value();
    println!("Total inventory value: {}", total);

    println!("Current inventory:");
    for (i, item) in inventory.items.iter().enumerate() {
        println!("{}: {} (value {})", i, item.name, item.value);
    }
  }
