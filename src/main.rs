use std::io;
use std::collections::HashMap;

fn main() {

    struct Item {
        name: String,
        quantity: i32,
        price: f64
    }

    let mut count:u32 = 1;
    let mut inventory: HashMap<u32, Item> = HashMap::new();

    loop{
    println!("|Add a new item to inventory |(Enter '1')|");
    println!("|View current inventory      |(Enter '2')|");
    println!("|EXIT!                       |(Enter '3')|");
    let mut entval1 = String::new();
    io::stdin().read_line(&mut entval1).expect("Failes to readline");

    if entval1.trim() == "1" {
        println!("Enter Item name :");
        let mut itmname = String::new();
        io::stdin().read_line(&mut itmname).expect("failes to get itemname");

        println!("Enter quantity :");
        let mut itmqua = String::new();
        io::stdin().read_line(&mut itmqua).expect("failes to get quantity");
        let mut itmquaint: i32 = itmqua.trim().parse().expect("typecastig error");

        println!("Enter Price :");
        let mut itmprice = String::new();
        io::stdin().read_line(&mut itmprice).expect("failes to get item price");
        let mut itmpriceflt: f64 = itmprice.trim().parse().expect("typecasing error");

        let item1 = Item {
            name: itmname.trim().to_string(),
            quantity: itmquaint,
            price: itmpriceflt
        };

        inventory.insert(count, item1);
        count += 1;

    }
    else if entval1.trim() == "2" {
        println!("||Inventory Items||");

        for (id, item) in &inventory {
            println!("ID:{}\t Name:{}\t Quantity:{}\t Price:{}", id,item.name, item.quantity, item.price);
        }
    } 
    else if entval1.trim() == "3" {
        break;
    }
    else {
        println!("Invalid input");
    }

    }
}
