mod modules {
    include!("../include/modules.rs");
}
use modules::leer_input;

fn main() {
    let _vector = leer_input();
    //divide_strings(_vector);
    divide_strings_two(_vector);
}

fn divide_strings_two(vector: Vec<String>) {
    let mut mut_vector: Vec<&String> = Vec::new();
    let mut cont: i32 = 0;
    let mut _division: usize = 0;
    let mut g_sum: u32 = 0;
    for i in 0..vector.len() {
        mut_vector.push(&vector[i]);
        cont += 1;
        if cont >= 3 {
            cont = 0;
            let result: Option<char> = mut_vector[0]
                .chars()
                .find(|c: &char| mut_vector[1].contains(*c) && mut_vector[2].contains(*c));
            match result {
                Some(C) => {
                    if C.is_ascii_alphabetic() && C.is_lowercase() {
                        let position: u32 = ((C as u32) - ('a' as u32)) + 1;
                        println!("{}", position);
                        g_sum = g_sum.checked_add(position).expect("Overflow");
                        println!("Suma: {}", g_sum);
                    } else if C.is_ascii_alphabetic() && C.is_uppercase() {
                        let position: u32 = (((C as u32) - ('A' as u32)) + 1) + 26;
                        println!("{}", position);
                        g_sum = g_sum.checked_add(position).expect("Overflow");
                        println!("Suma: {}", g_sum);
                    }
                }
                None => println!("No hay coincidencias"),
            }
            mut_vector.clear();
        }
    }
    println!("{:?}", mut_vector);
    print!("Suma: {}", g_sum);
}

fn _divide_strings(vector: Vec<String>) {
    let mut _division = 0;
    let mut g_sum: u32 = 0;
    for i in 0..vector.len() {
        _division = vector[i].len() / 2;

        let _first_half = &vector[i][0.._division];
        let _second_half = &vector[i][_division..vector[i].len()];

        let result = _first_half.chars().find(|c| _second_half.contains(*c));

        match result {
            Some(c) => {
                if c.is_ascii_alphabetic() && c.is_lowercase() {
                    let position = ((c as u32) - ('a' as u32)) + 1;
                    println!("{}", position);
                    g_sum = g_sum.checked_add(position).expect("Overflow");
                    println!("Suma: {}", g_sum);
                } else if c.is_ascii_alphabetic() && c.is_uppercase() {
                    let position = (((c as u32) - ('A' as u32)) + 1) + 26;
                    println!("{}", position);
                    g_sum = g_sum.checked_add(position).expect("Overflow");
                    println!("Suma: {}", g_sum);
                }
            }
            None => println!("No hay coincidencias"),
        }

        /* println!("Result: {}", result); */
    }
}
