use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn leer_input() -> Vec<String> {
    let mut vector: Vec<String> = Vec::new();

    //quiero introducir file_path por teclado:
    /* println!("Introduce el path del fichero: ");
    let mut file_path = String::new();
    //lo mismo pero con .trim()
    
    std::io::stdin().read_line(&mut file_path).expect("Failed to read line");
    let file_path = file_path.trim(); */
    
    let file_path = "Input/input.txt";
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    };

    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(error) => panic!("There was a problem reading a line: {:?}", error),
        };

        vector.push(line.clone());
    }

    vector
}
