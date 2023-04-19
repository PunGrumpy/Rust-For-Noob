fn main() {
    println!("Data types in Rust");

    println!("========== Integer types ==========");
    int();

    println!("========== Floating point types ==========");
    float();

    println!("========== Boolean type ==========");
    boolean();

    println!("========== Character type ==========");
    char();

    println!("========== Array type ==========");
    array();

    println!("========== Tuple type ==========");
    tuple();

    println!("========== String types ==========");
    string();
}

fn int() {
    let int: i8 = -20; // i8 is a signed integer type
    let uint: u8 = 20; // u8 is an unsigned integer type
    println!("int: {}, uint: {}", int, uint);
}

fn float() {
    let f32: f32 = 8.2; // f32 is a 32-bit floating point type
    let f64: f64 = 8.2; // f64 is a 64-bit floating point type
    println!("f32: {}, f64: {}", f32, f64);
}

fn boolean() {
    let bool: bool = true;
    println!("bool: {}", bool);
}

fn char() {
    let char: char = 'A';
    println!("char: {}", char);
}

fn array() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let big_array: [i32; 100] = [0; 100]; // 100 zeros
    println!("array: {:?}", array);
    println!("array length: {}", array.len());
    println!("array first element: {}", array[0]);
    println!("big_array: {:?}", big_array);
}

fn tuple() {
    let tuple: (&str, &str, i32) = ("John", "Doe", 32);
    let (first_name, last_name, age) = tuple; // also called "deconstruction"
    println!("{} {} is {} years old", tuple.0, tuple.1, tuple.2);
    println!("{} {} is {} years old", first_name, last_name, age);

    let (country, city, pop) = ("USA", "New York", 1000);
    println!("{} {} has a population of {}", country, city, pop);
}

fn string() {
    // &str or String called "String slice"
    let slice: &str = "Hello, world!";
    let str: String = slice.to_string();
    let str2: String = "Hello, world!".to_string();
    let str3: String = String::from("Hello, world!");
    println!("slice: {}", slice);
    println!("str: {}", str);
    println!("str2: {}", str2);
    println!("str3: {}", str3);

    let slice2: &str = &str;
    let slice3: &str = str.as_str();
    println!("slice2: {}", slice2);
    println!("slice3: {}", slice3);

    let first_name: &str = "John";
    let last_name: &str = "Doe";
    let full_name: String = format!("{} {}", first_name, last_name);
    let full_name2: String = [first_name, " ", last_name].concat();
    println!("full_name: {}", full_name);
    println!("full_name2: {}", full_name2);

    let mut name: String = String::from("John"); // Mut means mutable (can be changed)
    name.push_str(" Doe");
    println!("name: {}", name);
}