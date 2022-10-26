// Buatlah sebuah program I/O (Input/ Output) yang menerapkan setidaknya 4 konsep dari
// materi Basic Programming. Adapun konsep yang dimaksud adalah sebagai berikut:
// 1. Struct dan Enum (DONE)
// 2. Loop (DONE)
// 3. Handling process with Result<T> (DONE)
// 4. Manipulate Vector using built-in func  (DONE)
// 5. Date and time using chrono crate (DONE)
// 6. Trait --> Trait attack/fire ? (not yet)
// 7. Unit Test (Optional)

//CASE SOLUTION (Henry H. Prasetya)
//Weaponsmith
//FItur: 
//Setiap weapon punya name, class, caliber, dan accuracy
//CRUD
//display time date di menu (Done)
//'fire weapon', rng bool. Output 'Hit' atau 'Miss', (not yet)
//kemungkinan % nya ditentukan accuracy weapon (not yet)

use std::collections::HashMap;
use std::io;
use rand::{thread_rng, Rng};
use chrono::prelude::*;

#[derive(Debug, Clone)]
enum Weapon_Class {
    Smg, 
    AssaultRifle, 
    LongRifle, 
    Melee,
    Lmg,
    Shotgun,
}

#[derive(Debug, Clone)]
struct Weapon {
    name: String,
    class: String, //Should be weapon class tapi test dulu String.
    caliber: f64,
    accuracy: f64,
}

struct Weapons {
    inner: HashMap<String, Weapon>
}

impl Weapons { 
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn add(&mut self, weapon:Weapon){
        self.inner.insert(weapon.name.clone(), weapon);
    }

    fn get_all(&self) -> Vec<&Weapon> {
        let mut weapons = vec![];
        for weapon in self.inner.values() {
            weapons.push(weapon)
        }
        weapons
    }

    fn remove(&mut self, name: &str) -> bool {
        self.inner.remove(name).is_some()
    }
}

fn get_input_string() -> Option <String> {
    //buffer itu string, stdin sebagai string.
    //di trim di input supaya valid.
    let mut buffer = String::new();

    while io::stdin()
        .read_line(&mut buffer)
        .is_err() {
        println!("Invalid Data")
    }

    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else { 
        Some(input)
    }
}

fn get_input_float() -> Option <f64> {
    loop {
        let input = match get_input_string() {
            Some(input) => input,
            None => return None
        };

        if &input == "" {
            return None;
        }

        let input_parsed: Result<f64,_> = input.parse();
        match input_parsed{
            Ok(float) => return Some(float),
            Err(_) => println!("Please enter a number") 
        }
    }
}

fn add_weapon_menu(weapons: &mut Weapons) {
    println!("Weapon name:");
    let name = match get_input_string(){
        Some(input) => input,
        None => return
    };
    println!("Weapon Class / type :");
    let class = match get_input_string(){
        Some(input) => input,
        None => return
    };
    println!("Weapon caliber (float) :");
    let caliber = match get_input_float() {
        Some(caliber) => caliber,
        None => return
    };
    println!("Weapon accuracy (float between 0 and 1)");
    let accuracy = match get_input_float() {
        Some(accuracy) => accuracy,
        None => return
    };

    let weapon = Weapon {name, class, caliber, accuracy};
    weapons.add(weapon);
    println!("Weapon Added to list.");
}

fn remove_weapon_menu(weapons: &mut Weapons) {
    for weapon in weapons.get_all() {
        println!("{:?}", weapon);
    }
    println!("Enter weapon name to remove:");
    let name = match get_input_string() {
        Some(input) => input,
        None => return
    };
    if weapons.remove(&name) {
        println!("{:?} Removed from list", name)
    } else {
        println!("Weapon not found")
    }
}

fn view_weapons_menu(weapons: &Weapons) {
    for weapon in weapons.get_all() {
        println!("{:?}", weapon);
    }
}

fn print_date() {
    let local: DateTime<Local> = Local::now();
    println!("{}", local.format("%A %e %B %Y, %T").to_string());
}

fn menu() {
    fn show() {
        println!("");
        println!("=== Weaponsmith 0.5 ===");
        println!("=== By : Henry Hamilton Prasetya ===");
        print_date();
        println!("");
        println!("1. Add New Weapon");
        println!("2. View All Weapons");
        println!("3. Remove Weapons");
        // println!("4. Update Weapons");
        // println!("5. Fire Weapon");
        println!("Press any other key to exit.");
        println!("");
        println!("Enter Selection >>>");
    }
    //new struct Weapons (list)
    let mut weapons = Weapons::new();

    //loop display menu
    loop {
        show();
        let input = match get_input_string(){
            Some(input) => input,
            None => return 
        };
        match input.as_str() {
            "1" => add_weapon_menu(&mut weapons),
            "2" => view_weapons_menu(&weapons),
            "3" => remove_weapon_menu(&mut weapons),
            // "4" => ,
            // "5" => ,
            _ => break
        }
    }
}

fn main() {
    menu();

}
