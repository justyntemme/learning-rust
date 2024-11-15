//Expressions and statements
// 5 = expression
// true = expression
// add(3,4) -> expression
// if else -> expressions
// Expression == anything that returns a value / including {} aka blocks
fn human_id(name: &str, age: u32, height: f32) {
    println!(
        "My name is {}, I am {}, years old, and my height is {} cm",
        name, age, height
    )
}
fn hello_world() {
    println!("hello rustcians");
}

fn tell_height(height: i32) {
    println!("My height is {}", height)
}

fn add_int(a: i32, b: i32) -> i32 {
    a + b
}

//Statement == anything that DOESNT return a value
// let x = 5; === statements
// fn foo() {} Statement
fn calc_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}
//Ownership, Borowing and References
//
//Ownership
//
//C, C++ -> Memory management control Issue
// Garbage Collector solved this issue but created new issue
//
//
// Ownership
// 1. every value has a single Owner
// 2. only one owner at a time
// 3. when the owner goes out of scope, the value will be dropped
//

fn main() {
    let bmi: f64 = calc_bmi(70.80, 123.34);
    println!("Your BMI is {}", bmi);
    let z = add_int(32, 23);
    println!("Z equals {}", z);
    let _X: i32 = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty
    };
    let x = _X;
    let y: u64 = 1000000;
    println!("Hello, world! and fuck you! {}", x ^ 1);
    println!("Hello, world! and fuck you! {}", y ^ 2);

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number Array: {:?}", numbers);
    let fruits: [&str; 2] = ["apple", "bananana"];
    println!("Fruits array 2rd element {:?}", fruits[1]);

    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human tuple {:?}", human);
    let mix = ("Kratos".to_string(), 23, true, [1, 2, 3, 4, 5]);
    println!("Mix tuple: {:?}", mix);

    // Slices: dynamically sized [1,2,3,4,5]
    //
    let gator: &[&String] = &[&"IT".to_string(), &"Harry potter".to_string()];
    println!("Slices {:?}", gator);

    //Strings vs String Slices (&str)
    //
    //
    let mut stone_cold: String = String::from("Hell, ");

    stone_cold.push_str("Yeah!");

    println!("stone_cold says: {}", stone_cold);

    //B- &str *String slice)
    let upper: String = String::from("hello darkness my old friend").to_uppercase();
    let name: String = String::from("Hello, World!");
    let slice: &str = &name[0..5];
    println!("Slice Value {}", slice);
    println!("Slice Len {}", slice.len());
    println!("Uppercase: {}", upper);
    //functions
    hello_world();
    tell_height(32);
    human_id("Justyn", 55, 182.2);
    let s1 = String::from("RUST");
    let len = calc_len(&s1);
    println!("len is {}", len);
    let s2 = s1;
    println!("{}", s2);

    let mut _x: i32 = 4;
    let _r: &mut i32 = &mut _x;

    *_r += 1;
    //BIG lesson
    // One mut reference to a value or many immutable references
    println!("Value of _x : {}", _x);
    let mut account: BankAccount = BankAccount {
        owner: "Alice".to_string(),
        balance: 150.55,
    };

    account.check_balance();
    account.withdraw(10.00);

    //immutable borrow to check balance
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!(
            "withdrawing {} from account owned by {}",
            amount, self.owner
        );
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!(
            "ACcount owned by {} has a blanace of {}",
            self.owner, self.balance,
        );
    }
}

fn calc_len(s: &String) -> usize {
    s.len()
}
