// 2019-05-07

// Le but du jeu est d'afficher du gros texte en ascii, par exemple

// Comme input : une string, et comme output : une string aussi.
// Entre deux, plusieurs transformations
    // De String à Vec<char>
    // de Vec<char> à Vec<Lettre> où Lettre est un struct de String
    // de Vec<Lettre> à un seul struct de String
    // Compiler toutes les String du struct en une seule

mod lib;
use lib::Lettre;
use lib::Mot;
use lib::string_to_char_vector;
use lib::chars_to_structs;


fn main() {


    // Utiliser clap pour récupérer l'entrée utilisateur
    // hardcodons l'entrée utilisateur pour le moment
    let entree: String = String::from("acb");
    println!("Voici l'entrée utilisateur : {}\n", entree);

    // Décomposer l'entrée en une collection de caractères de type char
    let vecteur_caracteres: Vec<char> = string_to_char_vector(entree);
    println!("\nVoici l'entrée sous forme de vecteur : {:?}", vecteur_caracteres);

    // transformer le vecteur de caractère en vecteur de structs, l'afficher
    let vecteur_structs = chars_to_structs(vecteur_caracteres);
    println!("Nous avons converti ce vecteur de caractères en vecteur de structs,
que voici:");
    let iterat = vecteur_structs.iter();
    for lettre in iterat {
        println!("{}", lettre);
    }


    // Créer le struct final, qui contiendra les lignes concaténées
    let mut struct_final = Mot::new();
    
    // Six fois de suite, on ajoute les lignes de la lettre aux lignes du mot
    // Et on passe à la lettre suivante grâce à l'itérateur.
    let nouveliterateur = vecteur_structs.iter();
    for lettre in nouveliterateur {
        println!("l'itérateur montre:\n{}", lettre);
        struct_final.ligne1.push_str(&lettre.ligne1);
        struct_final.ligne2.push_str(&lettre.ligne2);
        struct_final.ligne3.push_str(&lettre.ligne3);
        struct_final.ligne4.push_str(&lettre.ligne4);
        struct_final.ligne5.push_str(&lettre.ligne5);
        struct_final.ligne6.push_str(&lettre.ligne6);
    };
    println!("{}", struct_final);

}
