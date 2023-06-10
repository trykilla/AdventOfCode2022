mod modules;

//puzzle input: encrypted strategy guide
//La primera columna es la que tu oponente va a jugar: A es piedra, B es papel y C es tijera.+
//Lo que tu debes jugar es la segunda columna: X es piedra, Y es papel y Z es tijera.
//Tu score es la suma de los scores de cada ronda.

/*

POR EJEMPLO

    A Y
    B X
    C Z


*/

/* EN LA PRIMERA RONDA, TU OPONENTE ELIGE PIEDRA (A) Y TU DEBES ELEGIR PAPEL (Y). TE DA UNA VICTORIA DE 8 PUNTOS
EN LA SEGUNDA RONDA, TU OPONENTE ELIGE PAPEL (B) Y TU DEBES ELEGIR PIEDRA (X). TE DA UNA DERROTA DE 1 PUNTO
LA TERCERA RONDA ES UN EMPATA A TIJERAS Y TE DA 6 PUNTOS

SI HUBIERAS SEGUIDO LA ESTRATEGIA, HABRÍAS OBTENIDO 15 PUNTOS (8 + 1 + 6)

 */

/*SEGUNDA PARTE! */

/*

   X -> PERDER
   Y -> EMPATAR
   Z -> GANAR

*/

const PIEDRA: i32 = 1;
const PAPEL: i32 = 2;
const TIJERA: i32 = 3;

const GANAR: i32 = 6;
const EMPATAR: i32 = 3;
const PERDER: i32 = 0;

//a eso se le suma el score de la ronda en la que estás
//Tienes que calcular el score que sacarías si juegas la guía

fn main() {
    let score = 0;
    let mut _vector: Vec<String> = Vec::new();
    _vector = crate::modules::leer_input();
    //calc_total_score(_vector, score);

    calc_total_score_final(_vector, score);
    /* crate::modules::new_func(); */
}

fn calc_total_score_final(vector: Vec<String>, mut score: i32) {
    for i in 0..vector.len() {
        let divided_vector: Vec<Vec<char>> = vector
            .iter()
            .map(|element| element.chars().filter(|&c| c != ' ').collect())
            .collect();

        match divided_vector[i].as_slice() {
            ['A', 'Y'] => score += EMPATAR + PIEDRA,          //
            ['A', 'Z'] => score += GANAR + PAPEL,             //
            ['A', 'X'] => score += PERDER + TIJERA,           //
            ['B', 'Y'] => score += EMPATAR + PAPEL,           //
            ['B', 'X'] => score += PERDER + PIEDRA,           //
            ['B', 'Z'] => score += GANAR + TIJERA,            //
            ['C', 'X'] => score += PERDER + PAPEL,            //
            ['C', 'Y'] => score += EMPATAR + TIJERA,          //
            ['C', 'Z'] => score += GANAR + PIEDRA,            //
            _ => println!("No se cumple ninguna condición!"), // Caso por defecto: no se cumple ninguna de las condiciones anteriores
        }
    }
    println!("El score total final con la guía definitiva es: {} ", score);
}

fn calc_total_score(vector: Vec<String>, mut score: i32) {
    for i in 0..vector.len() {
        let divided_vector: Vec<Vec<char>> = vector
            .iter()
            .map(|element| element.chars().filter(|&c| c != ' ').collect())
            .collect();

        match divided_vector[i].as_slice() {
            ['A', 'Y'] => score += GANAR + PAPEL,
            ['A', 'Z'] => score += PERDER + TIJERA,
            ['B', 'Y'] => score += EMPATAR + PAPEL,
            ['B', 'X'] => score += PERDER + PIEDRA,
            ['B', 'Z'] => score += GANAR + TIJERA,
            ['C', 'X'] => score += GANAR + PIEDRA,
            ['C', 'Y'] => score += PERDER + PAPEL,
            ['A', 'X'] => score += EMPATAR + PIEDRA,
            ['C', 'Z'] => score += EMPATAR + TIJERA,
            _ => println!("No se cumple ninguna condición!"), // Caso por defecto: no se cumple ninguna de las condiciones anteriores
        }
    }
    println!("El score total es: {} ", score);
}
