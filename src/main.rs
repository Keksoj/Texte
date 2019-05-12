// 2019-05-07

// Le but du jeu est d'afficher du gros texte en ascii, par exemple

// Comme input : une string du standard input
// comme output : une string aussi au standard output
// Entre deux, plusieurs transformations
    // De String à Vec<char>
    // de Vec<char> à Vec<Lettre> où Lettre est un struct de String
    // de Vec<Lettre> à un seul struct de String
    // Compiler toutes les String du struct en une seule

use std::io;
mod lib;
use lib::string_to_char_vector;
use lib::split_into_lines;
use lib::decorer;
use lib::rassembler;


fn main() {

    // Récupérer l'entrée standard
    let mut entree: String = String::new();
    match io::stdin().read_line(&mut entree) {
        Ok(n) => {
            // println!("On va convertir ce texte : {}", entree);
        }
        Err(error) => println!("erreur: {}", error),
    }

    // Décomposer l'entrée en une collection de caractères de type char
    let vecteur_caracteres = string_to_char_vector(entree);
    // println!("\nVoici l'entrée sous forme de vecteur : {:?}", vecteur_caracteres);

    let lignes_de_caracteres = split_into_lines(vecteur_caracteres);

    let paragraphe = decorer(lignes_de_caracteres);

    println!("Voilà le tout:\n{}", rassembler(paragraphe));
    //
    // // transformer le vecteur de caractère en vecteur de structs
    // let vecteur_structs = chars_to_structs(vecteur_caracteres);
    //
    // // transformer le vecteur de structs en un seul vecteur-mot
    // let paragraphe = structs_to_one(vecteur_structs);
    //
    // // transformer le paragraphe en une seule chaine de caractères
    // let chaine = paragraph_to_string(paragraphe);
    // println!("{}", chaine);
}
