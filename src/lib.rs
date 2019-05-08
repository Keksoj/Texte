// 2019-05-07

// Chaque lettre est un struct de six lignes, des

use std::fmt;

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

// convertir le vecteur de caractères en vecteur de structs-lettres
pub fn chars_to_structs(vecteur_caracteres: Vec<char>) -> Vec<Lettre> {

    // Créer un itérateur sur le vecteur de caractère
    let iterateur = vecteur_caracteres.iter();

    // Créer le vecteur de structs, il est vide pour le moment
    let mut vecteur_de_structs = Vec::new();

    // pour chaque caractère, compléter le vecteur de struct avec un struct-lettre
    for character in iterateur {
        vecteur_de_structs.push(char_to_struct(*character))
    }
    vecteur_de_structs
}

// convertir un caractère individuel en struct-lettre
pub fn char_to_struct(caractere: char) -> Lettre {
    match caractere {
        'a' => Lettre {
                    ligne1: r#"        "#,
                    ligne2: r#"  __ _  "#,
                    ligne3: r#" / _` | "#,
                    ligne4: r#"| (_| | "#,
                    ligne5: r#" \__,_| "#,
                    ligne6: r#"        "#,
                },
        'b' => Lettre {
                    ligne1: r#" _      "#,
                    ligne2: r#"| |__   "#,
                    ligne3: r#"| '_ \  "#,
                    ligne4: r#"| |_) | "#,
                    ligne5: r#"|_.__/  "#,
                    ligne6: r#"        "#,
                },
        'c' => Lettre {
                    ligne1: r#"        "#,
                    ligne2: r#"  ____  "#,
                    ligne3: r#" / ___| "#,
                    ligne4: r#"| (___  "#,
                    ligne5: r#" \____| "#,
                    ligne6: r#"        "#,
                },
        'd' => Lettre {
                    ligne1: r#"     _  "#,
                    ligne2: r#"  __| | "#,
                    ligne3: r#" / _  | "#,
                    ligne4: r#"| (_| | "#,
                    ligne5: r#" \__,_| "#,
                    ligne6: r#"        "#,
                },
        'e' => Lettre {
                    ligne1: r#"        "#,
                    ligne2: r#"  ___   "#,
                    ligne3: r#" / _  \ "#,
                    ligne4: r#"|  ___/ "#,
                    ligne5: r#" \____| "#,
                    ligne6: r#"        "#,
                },
        'f' => Lettre {
                    ligne1: r#"  __  "#,
                    ligne2: r#" / _| "#,
                    ligne3: r#"| |_  "#,
                    ligne4: r#"|  _| "#,
                    ligne5: r#"|_|   "#,
                    ligne6: r#"      "#,
                },


        ' ' => Lettre {
                    ligne1: "      ",
                    ligne2: "      ",
                    ligne3: "      ",
                    ligne4: "      ",
                    ligne5: "      ",
                    ligne6: "      ",
                },
        ',' => Lettre {
                    ligne1: "      ",
                    ligne2: "      ",
                    ligne3: "      ",
                    ligne4: " _    ",
                    ligne5: "( )   ",
                    ligne6: "|/    ",
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
