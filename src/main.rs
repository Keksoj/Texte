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


fn main() {


    // Utiliser clap pour récupérer l'entrée utilisateur
    // hardcodons l'entrée utilisateur pour le moment
    let entree: String = String::from("acb");
    println!("Voici l'entrée utilisateur : {}\n", entree);

    // Décomposer l'entrée en une collection de caractères de type char
    // D'abord, créer un itérateur qui parcourt l'entrée
    let iterateur = entree.chars();
    // Créer un vecteur de caractère
    let mut vecteur_caracteres: Vec<char> = Vec::new();
    // itérer pour passer les caractères de l'un à l'autre
    for caractere in iterateur {
        println!("Ajoutons {} au vecteur de caractères", caractere);
        vecteur_caracteres.push(caractere)
    }
    println!("\nVoici l'entrée sous forme de vecteur : {:?}", vecteur_caracteres);



    // Introduire une lettre, ici le a
    let lettrea = Lettre {
        ligne1: String::from("        "),
        ligne2: String::from("  __ _  "),
        ligne3: String::from(" / _` | "),
        ligne4: String::from("| ( | | "),
        ligne5: String::from(" ___,_| "),
        ligne6: String::from("        "),
    };

    let lettreb = Lettre {
        ligne1: String::from(" _      "),
        ligne2: String::from("| |_    "),
        ligne3: String::from("| '_    "),
        ligne4: String::from("| |_) | "),
        ligne5: String::from("|_.__/  "),
        ligne6: String::from("        "),
    };

    let lettrec = Lettre {
        ligne1: String::from("        "),
        ligne2: String::from("  ____  "),
        ligne3: String::from(" / ___| "),
        ligne4: String::from("| (___  "),
        ligne5: String::from(" _____| "),
        ligne6: String::from("        "),
    };



    // Créer un itérateur sur le vecteur de caractère
    let vecteur_caracteres_iter = vecteur_caracteres.iter();

    // Créer le vecteur de structs, il est vide pour le moment
    let mut vecteur_structs = Vec::new();

    // pour chaque caractère, ajouter le struct correspondant au vecteur de struct
    for car in vecteur_caracteres_iter {
        match car {
            'a' => vecteur_structs.push(&lettrea),
            'b' => vecteur_structs.push(&lettreb),
            'c' => vecteur_structs.push(&lettrec),
            _ => break,
        }
    }

    // Pour débugueur, vérifions que le vecteur de structs est bien
    let iterat = vecteur_structs.iter();
    println!("Nous avons converti ce vecteur de caractères en vecteur de structs,
que voici:");
    for lettre in iterat {
        println!("{}", lettre);
    }


    // Créer le struct final, qui contiendra les lignes concaténées
    let mut struct_final: Lettre = Lettre::new();

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
