// 2019-05-07

// Chaque lettre est un struct de six lignes, des

use std::fmt;
use std::char;

pub struct Lettre {
    pub ligne1: &'static str,
    pub ligne2: &'static str,
    pub ligne3: &'static str,
    pub ligne4: &'static str,
    pub ligne5: &'static str,
    pub ligne6: &'static str,
}

pub struct Mot {
    pub ligne1: String,
    pub ligne2: String,
    pub ligne3: String,
    pub ligne4: String,
    pub ligne5: String,
    pub ligne6: String,
}

impl Mot {
    pub fn new() -> Self {
        Self {
            ligne1: String::new(),
            ligne2: String::new(),
            ligne3: String::new(),
            ligne4: String::new(),
            ligne5: String::new(),
            ligne6: String::new(),
        }
    }
}

impl Lettre {
    pub fn new() -> Self {
        Self {
            ligne1: "",
            ligne2: "",
            ligne3: "",
            ligne4: "",
            ligne5: "",
            ligne6: "",
        }
    }
    pub fn clone(&self) -> Self {
        Self {
            ligne1: self.ligne1.clone(),
            ligne2: self.ligne2.clone(),
            ligne3: self.ligne3.clone(),
            ligne4: self.ligne4.clone(),
            ligne5: self.ligne5.clone(),
            ligne6: self.ligne6.clone(),
        }
    }
}

impl fmt::Display for Lettre {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}\n{}\n{}\n{}\n{}\n{}",
            &self.ligne1,
            &self.ligne2,
            &self.ligne3,
            &self.ligne4,
            &self.ligne5,
            &self.ligne6,
        )
    }
}

impl fmt::Display for Mot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}\n{}\n{}\n{}\n{}\n{}",
            &self.ligne1,
            &self.ligne2,
            &self.ligne3,
            &self.ligne4,
            &self.ligne5,
            &self.ligne6,
        )
    }
}
// convertir l'entrée utilisateur en vecteur de caractères
pub fn string_to_char_vector(entree: String) -> Vec<char> {

    // itérons
    let iterateur = entree.chars();
    // Créer un vecteur de caractère
    let mut vecteur_caracteres: Vec<char> = Vec::new();
    // itérer pour passer les caractères de l'un à l'autre
    for caractere in iterateur {
        println!("Ajoutons {} au vecteur de caractères", caractere);
        vecteur_caracteres.push(caractere)
    }
    vecteur_caracteres
}


pub fn chars_to_structs(vecteur_caracteres: Vec<char>) -> Vec<Lettre> {

    let lettrea = Lettre {
        ligne1: "        ",
        ligne2: "  __ _  ",
        ligne3: " / _` | ",
        ligne4: "| ( | | ",
        ligne5: " ___,_| ",
        ligne6: "        ",
    };
    println!("la lettre a : {}", lettrea.ligne3);
    let lettreb = Lettre {
        ligne1: " _      ",
        ligne2: "| |_    ",
        ligne3: "| '_    ",
        ligne4: "| |_) | ",
        ligne5: "|_.__/  ",
        ligne6: "        ",
    };

    let lettrec = Lettre {
        ligne1: "        ",
        ligne2: "  ____  ",
        ligne3: " / ___| ",
        ligne4: "| (___  ",
        ligne5: " _____| ",
        ligne6: "        ",
    };

    // Créer un itérateur sur le vecteur de caractère
    let iterateur = vecteur_caracteres.iter();

    // Créer le vecteur de structs, il est vide pour le moment
    let mut vecteur_de_structs = Vec::new();

    // pour chaque caractère, ajouter le struct correspondant au vecteur de struct
    for car in iterateur {
        vecteur_de_structs.push(char_to_struct(*car))
        // match car {
        //     'a' => { vecteur_de_structs.push(lettrea.clone()); println!("rajouté a") }
        //     'b' => { vecteur_de_structs.push(lettreb.clone()); println!("rajouté b") }
        //     'c' => { vecteur_de_structs.push(lettrec.clone()); println!("rajouté c") }
        //     _ => { println!("on panique !"); break }
        // }
    }

    vecteur_de_structs
}

pub fn char_to_struct(caractere: char) -> Lettre {
    match caractere {
        'a' => Lettre {
                    ligne1: "        ",
                    ligne2: "  __ _  ",
                    ligne3: " / _` | ",
                    ligne4: "| ( | | ",
                    ligne5: " ___,_| ",
                    ligne6: "        ",
                },
        'b' => Lettre {
                    ligne1: " _      ",
                    ligne2: "| |_    ",
                    ligne3: "| '_    ",
                    ligne4: "| |_) | ",
                    ligne5: "|_.__/  ",
                    ligne6: "        ",
                },
        'c' => Lettre {
                    ligne1: "        ",
                    ligne2: "  ____  ",
                    ligne3: " / ___| ",
                    ligne4: "| (___  ",
                    ligne5: " _____| ",
                    ligne6: "        ",
                },
        // En cas d'erreur, on renvoie un gros X
        _ => Lettre {
                ligne1: "OO    OO",
                ligne2: " OO  OO ",
                ligne3: "  OOOO  ",
                ligne4: "  OOOO  ",
                ligne5: " OO  OO ",
                ligne6: "OO    OO",
            },
    }
}
