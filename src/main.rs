fn main() {
    //fill machine with goodies
    let mut v_machine = VendingMachine {contents: fill_machine_with_items(), UserCredit: 1.4};
    finite_state_machine(&mut v_machine);
}

fn read_line() -> String {
    use std::io::{stdin,stdout,Write};
    let mut s=String::new();

    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    //println!("You typed: {}",s);
    s
}

fn get_state(s: &str) -> UserState {
    let mut n : u32 = 999;
    let mut inp = s;

    if s.parse::<u32>().is_ok() {
        n = s.parse::<u32>().unwrap();
        inp = "~";
    }
    let u = match inp.to_lowercase().as_ref() {
        "a" => UserState::AddMoney,
        "b" => UserState::Balance,
        "~" => UserState::Buy(n),
        "c" => UserState::CommandList,
        "i" => UserState::Inventory,
        "q"  => UserState::Quit,
        "r" => UserState::Restock,
        _ => UserState::Invalid,
    };
    return u
}


// STRUCTS
struct Item {
    name: String,
    price: f32,
    stock: u32
}
struct VendingMachine {
    contents: Vec<Item>,
    UserCredit: f32
}
//ENUMS
enum UserState{
    AddMoney,
    Balance,
    CommandList,
    Buy(u32),
    Inventory,
    Quit,
    Restock,
    Invalid
}

//FUNCTIONS
fn fill_machine_with_items() -> Vec<Item> {
    let mut coffee = Item {name: String::from("coffee"), price: 1.25, stock: 3};
    let mut fizzy_water = Item {name: String::from("cascade ice"), price: 1.5, stock: 5};
    let mut candy = Item {name: String::from("life savers"), price: 0.75, stock: 1};
    let mut mints = Item {name: String::from("mints"), price: 1.0, stock: 2};

    let mut contents = vec![coffee, fizzy_water, candy, mints];
    contents
}


fn finite_state_machine(mut m: &mut VendingMachine) {
    println!("Welcome to the Rusty ol Vending Machine");
    print_commands();

    loop {
        print!("Make a selection: ");

        let state = get_state(&read_line());

        match state {
            UserState::AddMoney => add_money(m),
            UserState::CommandList => print_commands(),
            UserState::Balance => println!("${} available", m.UserCredit),
            UserState::Buy(n) => {
                if n+1 > *&m.contents.len() as u32 {println!("Invalid entry: {}",n); continue;}

                buy_item(n,&mut m);
            }
            UserState::Inventory => print_inventory(&m.contents),
            UserState::Quit => break,
            UserState::Restock => restock(m),
            UserState::Invalid => println!("Invalid entry"),
        }


    }
}

fn buy_item(n: u32, v: &mut VendingMachine) {
    let item: &mut Item = &mut v.contents[n as usize];
    if item.price > v.UserCredit {println!("you dont have enough money");
    }
    else if item.stock < 1 {println!("{} is out of stock", item.name);}
    else {
        v.UserCredit -= item.price;
        item.stock -=1;
        println!("vending a {}, you have {} left", item.name, v.UserCredit);
    }
}

fn add_money(v: &mut VendingMachine) {

    let mut isValidEntry = false;
    while !isValidEntry {
        println!("enter how much money you want to add: ");
        let m = read_line();
        let inp = m.parse::<f32>();
        if inp.is_ok() {
            let inp = inp.unwrap();
            if inp > 0.0 {
                v.UserCredit += inp;
                println!("Added ${}, new balance is: ${}",inp, v.UserCredit);
                isValidEntry = true;
            }
            else {
                println!("this isn't an atm, you tried to withdraw [${}]", &m);
            }
        }
        else {
            println!("invalid entry [{}]", &m);
        }
    }
}

fn print_inventory(contents: &Vec<Item>) {
    let mut i=0;
    for item in contents {
        println!("[{}] | {} | ${} | Qty: {}",i,item.name,item.price,item.stock);
        i=i+1;
    }
}
fn print_commands() {
    println!("commands:\n a: add money \n b: balance \n #: Buy Item \n c: command list \n i: Inventory \n r: Restock \n q: Quit");
}

fn restock(v: &mut VendingMachine) {
    println!("<( Restock Mode, [e] to exit )>");


    loop {
        println!("*** [n]ew item, [#] edit item, [e]xit ***");
        print_inventory(&v.contents);
        let r = read_line();
        let nmbr = r.parse::<u32>();
        //exit command:
        if r == "e" {
            println!("exiting restock mode");
            return;
        } // exit restock mode
        else if r == "n" {
            println!("what is the items name?");
            let nme = read_line();
            println!("what does it cost?");
            let c = read_line();
            let cost = c.parse::<f32>();
            println!("how many are you stocking?");
            let q = read_line();
            let qty = q.parse::<u32>();
            if cost.is_ok() && qty.is_ok() {
                v.contents.push(Item { name: nme, price: cost.unwrap(), stock: qty.unwrap() });
            } else {
                println!("cost or qty weren't valid numbers.");
            }
        }
        else if nmbr.is_ok() {
            let nmbr = nmbr.unwrap();
            if nmbr + 1 > v.contents.len() as u32 {
                println!("not a valid selection");
                continue;
            }
            let item: &mut Item = &mut v.contents.swap_remove(nmbr as usize);

            println!("#What is the new item's name? Currently it is {}. press enter for no change", item.name);
            let mut nme = read_line();
            if nme.len() < 1 { nme = item.name.to_string(); }
            println!("#what does it cost? Currently it costs ${}. press enter for no change", item.price);
            let mut c = read_line();

            let cost = c.parse::<f32>();
            if cost.is_ok() { item.price = cost.unwrap(); }
            println!("#how many are you stocking? Currently there are {} in stock. enter for no change", item.stock);
            let mut q = read_line();

            let qty = q.parse::<u32>();
            if qty.is_ok() { item.stock = qty.unwrap(); }
            v.contents.push(Item { name: nme, price: item.price, stock: item.stock });

        }
    }
}
