// 2019-05-07

// Le but du jeu est d'afficher du gros texte en ascii, par exemple

// Comme input : une string, et comme output : une string aussi.

mod lib;
use lib::Lettre;


fn main() {


    // Utiliser clap pour récupérer l'entrée utilisateur

    println!("Hello, world!");

    // Introduire une lettre, ici le a
    let lettrea = Lettre {
        ligne1: String::from("        "),
        ligne2: String::from("  __ _  "),
        ligne3: String::from(" / _` | "),
        ligne4: String::from("| ( | | "),
        ligne5: String::from(" ___,_| "),
        ligne6: String::from("        "),
    };
    // L'afficher
    println!("voici la lettre a:\n{}", lettrea);

    let lettreb = Lettre {
        ligne1: String::from(" _      "),
        ligne2: String::from("| |_    "),
        ligne3: String::from("| '_    "),
        ligne4: String::from("| |_) | "),
        ligne5: String::from("|_.__/  "),
        ligne6: String::from("        "),
    };
    println!("voici la lettre b:\n{}", lettreb);

    let lettrec = Lettre {
        ligne1: String::from("        "),
        ligne2: String::from("  ____  "),
        ligne3: String::from(" / ___| "),
        ligne4: String::from("| (___  "),
        ligne5: String::from(" _____| "),
        ligne6: String::from("        "),
    };

    // Décomposer l'entrée en une collection de caractères de type char
    let mut mot_vecteur: Vec<&Lettre> = Vec::new();
    mot_vecteur.push(&lettrea);
    mot_vecteur.push(&lettreb);
    mot_vecteur.push(&lettrec);

    // compiler les contenus de a et b dans un struct appelé total
    // Créer le struct vide
    let mut mot_struct: Lettre = Lettre {
            ligne1: String::new(),
            ligne2: String::new(),
            ligne3: String::new(),
            ligne4: String::new(),
            ligne5: String::new(),
            ligne6: String::new(),
    };


    // créer un itérateur qui parcourra le vecteur mot
    let mot_iter = mot_vecteur.iter();
    for lettre in mot_iter {
        println!("l'itérateur montre:\n{}", lettre);
        mot_struct.ligne1.push_str(&lettre.ligne1);
        mot_struct.ligne2.push_str(&lettre.ligne2);
        mot_struct.ligne3.push_str(&lettre.ligne3);
        mot_struct.ligne4.push_str(&lettre.ligne4);
        mot_struct.ligne5.push_str(&lettre.ligne5);
        mot_struct.ligne6.push_str(&lettre.ligne6);
    };
    println!("{}", mot_struct);

}
