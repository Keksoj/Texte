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
        // println!("Ajoutons {} au vecteur de caractères", caractere);
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

// convertir le vecteur de structs-lettres en un seul vecteur-mot
pub fn structs_to_one(vecteur_structs: Vec<Lettre>) -> Mot {
    // Créer le struct final, qui contiendra les lignes concaténées
    let mut struct_final = Mot::new();

    // Six fois de suite, on ajoute les lignes de la lettre aux lignes du mot
    // Et on passe à la lettre suivante grâce à l'itérateur.
    let nouveliterateur = vecteur_structs.iter();
    for lettre in nouveliterateur {
        struct_final.ligne1.push_str(&lettre.ligne1);
        struct_final.ligne2.push_str(&lettre.ligne2);
        struct_final.ligne3.push_str(&lettre.ligne3);
        struct_final.ligne4.push_str(&lettre.ligne4);
        struct_final.ligne5.push_str(&lettre.ligne5);
        struct_final.ligne6.push_str(&lettre.ligne6);
    };
    struct_final
}

// ATTENTION TRÈS LONG SI DÉROULÉ
// convertir un caractère individuel en struct-lettre
pub fn char_to_struct(caractere: char) -> Lettre {
    match caractere {
        // the printable ASCII caracters in the official order

        // we won't really deal with newlines
        '\n' => Lettre {
            ligne1: r#"      "#,
            ligne2: r#"      "#,
            ligne3: r#"  new "#,
            ligne4: r#" line "#,
            ligne5: r#"      "#,
            ligne6: r#"      "#,
        },

        // space ! " ~ $ % & ' ( ) * + , - . /
        ' ' => Lettre {
            ligne1: r#"    "#,
            ligne2: r#"    "#,
            ligne3: r#"    "#,
            ligne4: r#"    "#,
            ligne5: r#"    "#,
            ligne6: r#"    "#,
        },
        '!' => Lettre {
            ligne1: r#" _  "#,
            ligne2: r#"| | "#,
            ligne3: r#"| | "#,
            ligne4: r#"|_| "#,
            ligne5: r#"(_) "#,
            ligne6: r#"    "#,
        },
        '"' => Lettre {
            ligne1: r#" _ _  "#,
            ligne2: r#"( | ) "#,
            ligne3: r#" V V  "#,
            ligne4: r#"      "#,
            ligne5: r#"      "#,
            ligne6: r#"      "#,
        },
        '#' => Lettre {
            ligne1: r#"   _  _    "#,
            ligne2: r#" _| || |_  "#,
            ligne3: r#"|_  ..  _| "#,
            ligne4: r#"|_      _| "#,
            ligne5: r#"  |_||_|   "#,
            ligne6: r#"           "#,
        },
        '$' => Lettre {
            ligne1: r#"  _   "#,
            ligne2: r#" | |  "#,
            ligne3: r#"/ __) "#,
            ligne4: r#"\__ \ "#,
            ligne5: r#"(   / "#,
            ligne6: r#" |_|  "#,
        },
        '%' => Lettre {
            ligne1: r#" _  __ "#,
            ligne2: r#"(_)/ / "#,
            ligne3: r#"  / /  "#,
            ligne4: r#" / /_  "#,
            ligne5: r#"/_/(_) "#,
            ligne6: r#"       "#,
        },
        '&' => Lettre {
            ligne1: r#"  ___    "#,
            ligne2: r#" ( _ )   "#,
            ligne3: r#" / _ \/\ "#,
            ligne4: r#"| (_>  < "#,
            ligne5: r#" \___/\/ "#,
            ligne6: r#"         "#,
        },
        '\'' => Lettre {
            ligne1: r#" _  "#,
            ligne2: r#"( ) "#,
            ligne3: r#"|/  "#,
            ligne4: r#"    "#,
            ligne5: r#"    "#,
            ligne6: r#"    "#,
        },
        '(' => Lettre {
            ligne1: r#"  __  "#,
            ligne2: r#" / /  "#,
            ligne3: r#"| |   "#,
            ligne4: r#"| |   "#,
            ligne5: r#"| |   "#,
            ligne6: r#" \_\  "#,
        },
        ')' => Lettre {
            ligne1: r#"__   "#,
            ligne2: r#"\ \  "#,
            ligne3: r#" | | "#,
            ligne4: r#" | | "#,
            ligne5: r#" | | "#,
            ligne6: r#"/_/  "#,
        },
        '*' => Lettre {
            ligne1: r#"       "#,
            ligne2: r#"__/\__ "#,
            ligne3: r#"\    / "#,
            ligne4: r#"/_  _\ "#,
            ligne5: r#"  \/   "#,
            ligne6: r#"       "#,
        },
        '+' => Lettre {
            ligne1: r#"        "#,
            ligne2: r#"   _    "#,
            ligne3: r#" _| |_  "#,
            ligne4: r#"|_   _| "#,
            ligne5: r#"  |_|   "#,
            ligne6: r#"        "#,
        },
        ',' => Lettre {
            ligne1: "      ",
            ligne2: "      ",
            ligne3: "      ",
            ligne4: " _    ",
            ligne5: "( )   ",
            ligne6: "|/    ",
        },
        '-' => Lettre {
            ligne1: r#"        "#,
            ligne2: r#"        "#,
            ligne3: r#" _____  "#,
            ligne4: r#"|_____| "#,
            ligne5: r#"        "#,
            ligne6: r#"        "#,
        },
        '.' => Lettre {
            ligne1: r#"    "#,
            ligne2: r#"    "#,
            ligne3: r#"    "#,
            ligne4: r#" _  "#,
            ligne5: r#"(_) "#,
            ligne6: r#"    "#,
        },
        '/' => Lettre {
            ligne1: r#"    __ "#,
            ligne2: r#"   / / "#,
            ligne3: r#"  / /  "#,
            ligne4: r#" / /   "#,
            ligne5: r#"/_/    "#,
            ligne6: r#"       "#,
        },

        // numbers and  : ; < = > ? @
        '0' => Lettre {
            ligne1: r#"  ___   "#,
            ligne2: r#" / _ \  "#,
            ligne3: r#"| | | | "#,
            ligne4: r#"| |_| | "#,
            ligne5: r#" \___/  "#,
            ligne6: r#"        "#,
        },
        '1' => Lettre {
            ligne1: r#" __  "#,
            ligne2: r#"/_ | "#,
            ligne3: r#" | | "#,
            ligne4: r#" | | "#,
            ligne5: r#" |_| "#,
            ligne6: r#"     "#,
        },
        '2' => Lettre {
            ligne1: r#" ____   "#,
            ligne2: r#"|___ \  "#,
            ligne3: r#"  __) | "#,
            ligne4: r#" / __/  "#,
            ligne5: r#"|_____| "#,
            ligne6: r#"        "#,
        },
        '3' => Lettre {
            ligne1: r#" _____  "#,
            ligne2: r#"|___ /  "#,
            ligne3: r#"  |_ \  "#,
            ligne4: r#" ___) | "#,
            ligne5: r#"|____/  "#,
            ligne6: r#"        "#,
        },
        '4' => Lettre {
            ligne1: r#" _  _    "#,
            ligne2: r#"| || |   "#,
            ligne3: r#"| || |_  "#,
            ligne4: r#"|__   _| "#,
            ligne5: r#"   |_|   "#,
            ligne6: r#"         "#,
        },
        '5' => Lettre {
            ligne1: r#" ____   "#,
            ligne2: r#"| ___|  "#,
            ligne3: r#"|___ \  "#,
            ligne4: r#" ___) | "#,
            ligne5: r#"|____/  "#,
            ligne6: r#"        "#,
        },
        '6' => Lettre {
            ligne1: r#"  __    "#,
            ligne2: r#" / /_   "#,
            ligne3: r#"| '_ \  "#,
            ligne4: r#"| (_) | "#,
            ligne5: r#" \___/  "#,
            ligne6: r#"        "#,
        },
        '7' => Lettre {
            ligne1: r#" _____  "#,
            ligne2: r#"|___  | "#,
            ligne3: r#"   / /  "#,
            ligne4: r#"  / /   "#,
            ligne5: r#" /_/    "#,
            ligne6: r#"        "#,
        },
        '8' => Lettre {
            ligne1: r#"  ___   "#,
            ligne2: r#" ( _ )  "#,
            ligne3: r#" / _ \  "#,
            ligne4: r#"| (_) | "#,
            ligne5: r#" \___/  "#,
            ligne6: r#"        "#,
        },
        '9' => Lettre {
            ligne1: r#"  ___   "#,
            ligne2: r#" / _ \  "#,
            ligne3: r#"| (_) | "#,
            ligne4: r#" \__, | "#,
            ligne5: r#"   /_/  "#,
            ligne6: r#"        "#,
        },
        ':' => Lettre {
            ligne1: r#"    "#,
            ligne2: r#" _  "#,
            ligne3: r#"(_) "#,
            ligne4: r#" _  "#,
            ligne5: r#"(_) "#,
            ligne6: r#"    "#,
        },
        ';' => Lettre {
            ligne1: r#"    "#,
            ligne2: r#" _  "#,
            ligne3: r#"(_) "#,
            ligne4: r#" _  "#,
            ligne5: r#"( ) "#,
            ligne6: r#"|/  "#,
        },
        '<' => Lettre {
            ligne1: r#"  __ "#,
            ligne2: r#" / / "#,
            ligne3: r#"/ /  "#,
            ligne4: r#"\ \  "#,
            ligne5: r#" \_\ "#,
            ligne6: r#"     "#,
        },
        '=' => Lettre {
            ligne1: r#"        "#,
            ligne2: r#" _____  "#,
            ligne3: r#"|_____| "#,
            ligne4: r#"|_____| "#,
            ligne5: r#"        "#,
            ligne6: r#"        "#,
        },
        '>' => Lettre {
            ligne1: r#"__   "#,
            ligne2: r#"\ \  "#,
            ligne3: r#" \ \ "#,
            ligne4: r#" / / "#,
            ligne5: r#"/_/  "#,
            ligne6: r#"     "#,
        },
        '?' => Lettre {
            ligne1: r#" ___  "#,
            ligne2: r#"|__ \ "#,
            ligne3: r#"  / / "#,
            ligne4: r#" |_|  "#,
            ligne5: r#" (_)  "#,
            ligne6: r#"      "#,
        },
        '@' => Lettre {
            ligne1: r#"   ____   "#,
            ligne2: r#"  / __ \  "#,
            ligne3: r#" / / _` | "#,
            ligne4: r#"| | (_| | "#,
            ligne5: r#" \ \__,_| "#,
            ligne6: r#"  \____/  "#,
        },


        // big case letters
        'A' => Lettre {
            ligne1: r#"    _     "#,
            ligne2: r#"   / \    "#,
            ligne3: r#"  / _ \   "#,
            ligne4: r#" / ___ \  "#,
            ligne5: r#"/_/   \_\ "#,
            ligne6: r#"          "#,
        },
        'B' => Lettre {
            ligne1: r#" ____   "#,
            ligne2: r#"| __ )  "#,
            ligne3: r#"|  _ \  "#,
            ligne4: r#"| |_) | "#,
            ligne5: r#"|____/  "#,
            ligne6: r#"        "#,
        },
        'C' => Lettre {
            ligne1: r#"  ____  "#,
            ligne2: r#" / ___| "#,
            ligne3: r#"| |     "#,
            ligne4: r#"| |___  "#,
            ligne5: r#" \____| "#,
            ligne6: r#"        "#,
        },
        'D' => Lettre {
            ligne1: r#" ____   "#,
            ligne2: r#"|  _ \  "#,
            ligne3: r#"| | | | "#,
            ligne4: r#"| |_| | "#,
            ligne5: r#"|____/  "#,
            ligne6: r#"        "#,
        },
        'E' => Lettre {
            ligne1: r#" _____  "#,
            ligne2: r#"| ____| "#,
            ligne3: r#"|  _|   "#,
            ligne4: r#"| |___  "#,
            ligne5: r#"|_____| "#,
            ligne6: r#"        "#,
        },
        'F' => Lettre {
            ligne1: r#" _____  "#,
            ligne2: r#"|  ___| "#,
            ligne3: r#"| |_    "#,
            ligne4: r#"|  _|   "#,
            ligne5: r#"|_|     "#,
            ligne6: r#"        "#,
        },
        'G' => Lettre {
            ligne1: r#"  ____  "#,
            ligne2: r#" / ___| "#,
            ligne3: r#"| |  _  "#,
            ligne4: r#"| |_| | "#,
            ligne5: r#" \____| "#,
            ligne6: r#"        "#,
        },
        'H' => Lettre {
            ligne1: r#" _   _  "#,
            ligne2: r#"| | | | "#,
            ligne3: r#"| |_| | "#,
            ligne4: r#"|  _  | "#,
            ligne5: r#"|_| |_| "#,
            ligne6: r#"        "#,
        },
        'I' => Lettre {
            ligne1: r#" ___  "#,
            ligne2: r#"|_ _| "#,
            ligne3: r#" | |  "#,
            ligne4: r#" | |  "#,
            ligne5: r#"|___| "#,
            ligne6: r#"      "#,
        },
        'J' => Lettre {
            ligne1: r#"     _  "#,
            ligne2: r#"    | | "#,
            ligne3: r#" _  | | "#,
            ligne4: r#"| |_| | "#,
            ligne5: r#" \___/  "#,
            ligne6: r#"        "#,
        },
        'K' => Lettre {
            ligne1: r#" _  __ "#,
            ligne2: r#"| |/ / "#,
            ligne3: r#"| ' /  "#,
            ligne4: r#"| . \  "#,
            ligne5: r#"|_|\_\ "#,
            ligne6: r#"       "#,
        },
        'L' => Lettre {
            ligne1: r#" _      "#,
            ligne2: r#"| |     "#,
            ligne3: r#"| |     "#,
            ligne4: r#"| |___  "#,
            ligne5: r#"|_____| "#,
            ligne6: r#"        "#,
        },
        'M' => Lettre {
            ligne1: r#" __  __  "#,
            ligne2: r#"|  \/  | "#,
            ligne3: r#"| |\/| | "#,
            ligne4: r#"| |  | | "#,
            ligne5: r#"|_|  |_| "#,
            ligne6: r#"         "#,
        },
        'N' => Lettre {
            ligne1: r#" _   _  "#,
            ligne2: r#"| \ | | "#,
            ligne3: r#"|  \| | "#,
            ligne4: r#"| |\  | "#,
            ligne5: r#"|_| \_| "#,
            ligne6: r#"        "#,
        },
        'O' => Lettre {
            ligne1: r#"  ____   "#,
            ligne2: r#" / __ \  "#,
            ligne3: r#"| |  | | "#,
            ligne4: r#"| |__| | "#,
            ligne5: r#" \____/  "#,
            ligne6: r#"         "#,
        },
        'P' => Lettre {
            ligne1: r#" ____   "#,
            ligne2: r#"|  _ \  "#,
            ligne3: r#"| |_) | "#,
            ligne4: r#"|  __/  "#,
            ligne5: r#"|_|     "#,
            ligne6: r#"        "#,
        },
        'Q' => Lettre {
            ligne1: r#"  ___   "#,
            ligne2: r#" / _ \  "#,
            ligne3: r#"| | | | "#,
            ligne4: r#"| |_| | "#,
            ligne5: r#" \__\_\ "#,
            ligne6: r#"        "#,
        },
        'R' => Lettre {
            ligne1: r#" ____   "#,
            ligne2: r#"|  _ \  "#,
            ligne3: r#"| |_) | "#,
            ligne4: r#"|  _ <  "#,
            ligne5: r#"|_| \_\ "#,
            ligne6: r#"        "#,
        },
        'S' => Lettre {
            ligne1: r#" ____   "#,
            ligne2: r#"/ ___|  "#,
            ligne3: r#"\___ \  "#,
            ligne4: r#" ___) | "#,
            ligne5: r#"|____/  "#,
            ligne6: r#"        "#,
        },
        'T' => Lettre {
            ligne1: r#" _____  "#,
            ligne2: r#"|_   _| "#,
            ligne3: r#"  | |   "#,
            ligne4: r#"  | |   "#,
            ligne5: r#"  |_|   "#,
            ligne6: r#"        "#,
        },
        'U' => Lettre {
            ligne1: r#" _   _  "#,
            ligne2: r#"| | | | "#,
            ligne3: r#"| | | | "#,
            ligne4: r#"| |_| | "#,
            ligne5: r#" \___/  "#,
            ligne6: r#"        "#,
        },
        'V' => Lettre {
            ligne1: r#"__     __ "#,
            ligne2: r#"\ \   / / "#,
            ligne3: r#" \ \ / /  "#,
            ligne4: r#"  \ V /   "#,
            ligne5: r#"   \_/    "#,
            ligne6: r#"          "#,
        },
        'W' => Lettre {
            ligne1: r#"__        __ "#,
            ligne2: r#"\ \      / / "#,
            ligne3: r#" \ \ /\ / /  "#,
            ligne4: r#"  \ V  V /   "#,
            ligne5: r#"   \_/\_/    "#,
            ligne6: r#"             "#,
        },
        'X' => Lettre {
            ligne1: r#"__  __ "#,
            ligne2: r#"\ \/ / "#,
            ligne3: r#" \  /  "#,
            ligne4: r#" /  \  "#,
            ligne5: r#"/_/\_\ "#,
            ligne6: r#"       "#,
        },
        'Y' => Lettre {
            ligne1: r#"__   __ "#,
            ligne2: r#"\ \ / / "#,
            ligne3: r#" \ V /  "#,
            ligne4: r#"  | |   "#,
            ligne5: r#"  |_|   "#,
            ligne6: r#"        "#,
        },
        'Z' => Lettre {
            ligne1: r#" _____ "#,
            ligne2: r#"|__  / "#,
            ligne3: r#"  / /  "#,
            ligne4: r#" / /_  "#,
            ligne5: r#"/____| "#,
            ligne6: r#"       "#,
        },

        // [ \ ] ^ _ `
        '[' => Lettre {
            ligne1: r#" __  "#,
            ligne2: r#"| _| "#,
            ligne3: r#"| |  "#,
            ligne4: r#"| |  "#,
            ligne5: r#"| |  "#,
            ligne6: r#"|__| "#,
        },
        '\\' => Lettre {
            ligne1: r#"__     "#,
            ligne2: r#"\ \    "#,
            ligne3: r#" \ \   "#,
            ligne4: r#"  \ \  "#,
            ligne5: r#"   \_\ "#,
            ligne6: r#"       "#,
        },
        ']' => Lettre {
            ligne1: r#" __  "#,
            ligne2: r#"|_ | "#,
            ligne3: r#" | | "#,
            ligne4: r#" | | "#,
            ligne5: r#" | | "#,
            ligne6: r#"|__| "#,
        },
        '^' => Lettre {
            ligne1: r#" /\  "#,
            ligne2: r#"|/\| "#,
            ligne3: r#"     "#,
            ligne4: r#"     "#,
            ligne5: r#"     "#,
            ligne6: r#"     "#,
        },
        '_' => Lettre {
            ligne1: r#"        "#,
            ligne2: r#"        "#,
            ligne3: r#"        "#,
            ligne4: r#"        "#,
            ligne5: r#" _____  "#,
            ligne6: r#"|_____| "#,
        },
        '`' => Lettre {
            ligne1: r#" _  "#,
            ligne2: r#"( ) "#,
            ligne3: r#" \| "#,
            ligne4: r#"    "#,
            ligne5: r#"    "#,
            ligne6: r#"    "#,
        },

        // small case letters
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
        'g' => Lettre {
            ligne1: r#"        "#,
            ligne2: r#"  __ _  "#,
            ligne3: r#" / _` | "#,
            ligne4: r#"| (_| | "#,
            ligne5: r#" \__, | "#,
            ligne6: r#" |___/  "#,
        },
        'h' => Lettre {
            ligne1: r#" _      "#,
            ligne2: r#"| |__   "#,
            ligne3: r#"| '_ \  "#,
            ligne4: r#"| | | | "#,
            ligne5: r#"|_| |_| "#,
            ligne6: r#"        "#,
        },
        'i' => Lettre {
            ligne1: r#" _  "#,
            ligne2: r#"(_) "#,
            ligne3: r#"| | "#,
            ligne4: r#"| | "#,
            ligne5: r#"|_| "#,
            ligne6: r#"    "#,
        },
        'j' => Lettre {
            ligne1: r#"   _  "#,
            ligne2: r#"  (_) "#,
            ligne3: r#"  | | "#,
            ligne4: r#"  | | "#,
            ligne5: r#" _/ | "#,
            ligne6: r#"|__/  "#,
        },
        'k' => Lettre {
            ligne1: r#" _     "#,
            ligne2: r#"| | __ "#,
            ligne3: r#"| |/ / "#,
            ligne4: r#"|   <  "#,
            ligne5: r#"|_|\_\ "#,
            ligne6: r#"       "#,
        },
        'l' => Lettre {
            ligne1: r#" _  "#,
            ligne2: r#"| | "#,
            ligne3: r#"| | "#,
            ligne4: r#"| | "#,
            ligne5: r#"|_| "#,
            ligne6: r#"    "#,
        },
        'm' => Lettre {
            ligne1: r#"            "#,
            ligne2: r#" _ __ ___   "#,
            ligne3: r#"| '_ ` _ \  "#,
            ligne4: r#"| | | | | | "#,
            ligne5: r#"|_| |_| |_| "#,
            ligne6: r#"            "#,
        },
        'n' => Lettre {
            ligne1: r#"        "#,
            ligne2: r#" _ __   "#,
            ligne3: r#"| '_ \  "#,
            ligne4: r#"| | | | "#,
            ligne5: r#"|_| |_| "#,
            ligne6: r#"        "#,
        },
        'o' => Lettre {
            ligne1: r#"        "#,
            ligne2: r#"  ___   "#,
            ligne3: r#" / _ \  "#,
            ligne4: r#"| (_) | "#,
            ligne5: r#" \___/  "#,
            ligne6: r#"        "#,
        },
        'p' => Lettre {
            ligne1: r#"        "#,
            ligne2: r#" _ __   "#,
            ligne3: r#"| '_ \  "#,
            ligne4: r#"| |_) | "#,
            ligne5: r#"| .__/  "#,
            ligne6: r#"|_|     "#,
        },
        'q' => Lettre {
            ligne1: r#"        "#,
            ligne2: r#"  __ _  "#,
            ligne3: r#" / _` | "#,
            ligne4: r#"| (_| | "#,
            ligne5: r#" \__, | "#,
            ligne6: r#"    |_| "#,
        },
        'r' => Lettre {
            ligne1: r#"       "#,
            ligne2: r#" _ __  "#,
            ligne3: r#"| '__| "#,
            ligne4: r#"| |    "#,
            ligne5: r#"|_|    "#,
            ligne6: r#"       "#,
        },
        's' => Lettre {
            ligne1: r#"      "#,
            ligne2: r#"  __  "#,
            ligne3: r#"/ __| "#,
            ligne4: r#"\__ \ "#,
            ligne5: r#"|___/ "#,
            ligne6: r#"      "#,
        },
        't' => Lettre {
            ligne1: r#" _    "#,
            ligne2: r#"| |_  "#,
            ligne3: r#"| __| "#,
            ligne4: r#"| |_  "#,
            ligne5: r#" \__| "#,
            ligne6: r#"      "#,
        },
        'u' => Lettre {
            ligne1: r#"        "#,
            ligne2: r#" _   _  "#,
            ligne3: r#"| | | | "#,
            ligne4: r#"| |_| | "#,
            ligne5: r#" \__,_| "#,
            ligne6: r#"        "#,
        },
        'v' => Lettre {
            ligne1: r#"        "#,
            ligne2: r#"__   __ "#,
            ligne3: r#"\ \ / / "#,
            ligne4: r#" \ V /  "#,
            ligne5: r#"  \_/   "#,
            ligne6: r#"        "#,
        },
        'w' => Lettre {
            ligne1: r#"           "#,
            ligne2: r#"__      __ "#,
            ligne3: r#"\ \ /\ / / "#,
            ligne4: r#" \ V  V /  "#,
            ligne5: r#"  \_/\_/   "#,
            ligne6: r#"           "#,
        },
        'x' => Lettre {
            ligne1: r#"       "#,
            ligne2: r#"__  __ "#,
            ligne3: r#"\ \/ / "#,
            ligne4: r#" >  <  "#,
            ligne5: r#"/_/\_\ "#,
            ligne6: r#"       "#,
        },
        'y' => Lettre {
            ligne1: r#"        "#,
            ligne2: r#" _   _  "#,
            ligne3: r#"| | | | "#,
            ligne4: r#"| |_| | "#,
            ligne5: r#" \__, | "#,
            ligne6: r#" |___/  "#,
        },
        'z' => Lettre {
            ligne1: r#"      "#,
            ligne2: r#" ____ "#,
            ligne3: r#"|_  / "#,
            ligne4: r#" / /  "#,
            ligne5: r#"/___| "#,
            ligne6: r#"      "#,
        },

        // { | } ~ and that's about it
        '{' => Lettre {
            ligne1: r#"    __ "#,
            ligne2: r#"   / / "#,
            ligne3: r#"  | |  "#,
            ligne4: r#" < <   "#,
            ligne5: r#"  | |  "#,
            ligne6: r#"   \_\ "#,
        },
        '|' => Lettre {
            ligne1: r#" _  "#,
            ligne2: r#"| | "#,
            ligne3: r#"| | "#,
            ligne4: r#"| | "#,
            ligne5: r#"| | "#,
            ligne6: r#"|_| "#,
        },
        '}' => Lettre {
            ligne1: r#"__    "#,
            ligne2: r#"\ \   "#,
            ligne3: r#" | |  "#,
            ligne4: r#"  > > "#,
            ligne5: r#" | |  "#,
            ligne6: r#"/_/   "#,
        },
        '~' => Lettre {
            ligne1: r#"      "#,
            ligne2: r#"      "#,
            ligne3: r#" /\/| "#,
            ligne4: r#"|/\/  "#,
            ligne5: r#"      "#,
            ligne6: r#"      "#,
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
