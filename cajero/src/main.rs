fn main() {
    println!("Introduce el dinero que quieres sacar: ");
    let mut file_path = String::new();
    
    std::io::stdin().read_line(&mut file_path).expect("Failed to read line");
    println!("Dinero a sacar: {}", file_path);
}
