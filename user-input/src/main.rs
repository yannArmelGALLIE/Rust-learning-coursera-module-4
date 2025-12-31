use std::io;

fn main() {
    let mut input = String::new();
    while input.trim() != "Stop" {
        println!("Entrez un mot ('Stop' pour sortir) : ");
        io::stdin().read_line(&mut input).expect("Erreur de lecture");
        println!("Vous avez entrez : {}", input);
        input.clear();
    }

    println!("Au revoir")
}