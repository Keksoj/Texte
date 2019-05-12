// 2019-05-07

// Chaque lettre est un struct de six lines, des

use std::fmt;

pub struct Lettre {
    pub line1: &'static str,
    pub line2: &'static str,
    pub line3: &'static str,
    pub line4: &'static str,
    pub line5: &'static str,
    pub line6: &'static str,
}

pub struct Mot {
    pub line1: String,
    pub line2: String,
    pub line3: String,
    pub line4: String,
    pub line5: String,
    pub line6: String,
}


impl Mot {
    pub fn new() -> Self {
        Self {
            line1: String::new(),
            line2: String::new(),
            line3: String::new(),
            line4: String::new(),
            line5: String::new(),
            line6: String::new(),
        }
    }
}
impl Mot {
    pub fn clone(&self) -> Self {
        Self {
            line1: self.line1.clone(),
            line2: self.line2.clone(),
            line3: self.line3.clone(),
            line4: self.line4.clone(),
            line5: self.line5.clone(),
            line6: self.line6.clone(),
        }
    }
}

impl fmt::Display for Lettre {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}\n{}\n{}\n{}\n{}\n{}",
            &self.line1,
            &self.line2,
            &self.line3,
            &self.line4,
            &self.line5,
            &self.line6,
        )
    }
}

impl fmt::Display for Mot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}\n{}\n{}\n{}\n{}\n{}",
            &self.line1,
            &self.line2,
            &self.line3,
            &self.line4,
            &self.line5,
            &self.line6,
        )
    }
}
//
// impl fmt::Display for Vec<Mot> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let mut chaine: String = String::new();
//         let iterateur = self.iter();
//         for mot in iterateur {
//             chaine.push_str(&mot.line1);
//         }
//         write!(f, "{}", chaine)
//     }
// }

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

pub fn split_into_lines(vecteur_de_caracteres: Vec<char>) -> Vec<Vec<char>> {

    // cloner le vecteur pour le splitter comme on veut
    let mut vecteur_a_splitter: Vec<char> = vecteur_de_caracteres.clone();

    // créer le vecteur de vecteur
    let mut lignes: Vec<Vec<char>> = Vec::new();

    // Faire des lignes de dix caractères
    while vecteur_a_splitter.len() > 20 {
        let ligne: Vec<char> = vecteur_a_splitter.drain(0..20).collect();
        println!("On rajoute la ligne {:?}", ligne);
        lignes.push(ligne);
    }

    // rajouter le reste
    println!("Et il reste ça : {:?}", vecteur_a_splitter);
    lignes.push(vecteur_a_splitter);

    println!("{:?}", lignes);

    lignes
}

pub fn decorer(lignes: Vec<Vec<char>>) -> Vec<Mot> {

    let mut jolies_lignes: Vec<Mot> = Vec::new();

    let iterer_les_lignes = lignes.iter();

    for ligne in iterer_les_lignes {

        let mut ligne_a_completer = Mot::new() ;

        let iterer_les_lettres = ligne.iter();

        for lettre in iterer_les_lettres {
            ligne_a_completer.line1.push_str(char_to_struct(*lettre).line1);
            ligne_a_completer.line2.push_str(char_to_struct(*lettre).line2);
            ligne_a_completer.line3.push_str(char_to_struct(*lettre).line3);
            ligne_a_completer.line4.push_str(char_to_struct(*lettre).line4);
            ligne_a_completer.line5.push_str(char_to_struct(*lettre).line5);
            ligne_a_completer.line6.push_str(char_to_struct(*lettre).line6);
        }
        println!("On a ajouté la ligne : \n{}", ligne_a_completer);

        jolies_lignes.push(ligne_a_completer);
    }
    // println!("Ce qui nous donne : {}", jolies_lignes);
    jolies_lignes
}

pub fn rassembler(jolies_lignes: Vec<Mot>) -> String {
    let mut paragraphe = String::new();
    let iterateur = jolies_lignes.iter();
    for ligne in iterateur {
        paragraphe.push_str(&ligne.line1); paragraphe.push('\n');
        paragraphe.push_str(&ligne.line2); paragraphe.push('\n');
        paragraphe.push_str(&ligne.line3); paragraphe.push('\n');
        paragraphe.push_str(&ligne.line4); paragraphe.push('\n');
        paragraphe.push_str(&ligne.line5); paragraphe.push('\n');
        paragraphe.push_str(&ligne.line6); paragraphe.push('\n');
    }
    paragraphe
}

// ATTENTION TRÈS LONG SI DÉROULÉ
// convertir un caractère individuel en struct-lettre
pub fn char_to_struct(caractere: char) -> Lettre {
    match caractere {
        // the printable ASCII caracters in the official order

        // we won't really deal with newlines
        '\n' => Lettre {
            line1: r#" "#,
            line2: r#" "#,
            line3: r#" "#,
            line4: r#" "#,
            line5: r#" "#,
            line6: r#" "#,
        },

        // space ! " ~ $ % & ' ( ) * + , - . /
        ' ' => Lettre {
            line1: r#"    "#,
            line2: r#"    "#,
            line3: r#"    "#,
            line4: r#"    "#,
            line5: r#"    "#,
            line6: r#"    "#,
        },
        '!' => Lettre {
            line1: r#" _  "#,
            line2: r#"| | "#,
            line3: r#"| | "#,
            line4: r#"|_| "#,
            line5: r#"(_) "#,
            line6: r#"    "#,
        },
        '"' => Lettre {
            line1: r#" _ _  "#,
            line2: r#"( | ) "#,
            line3: r#" V V  "#,
            line4: r#"      "#,
            line5: r#"      "#,
            line6: r#"      "#,
        },
        '#' => Lettre {
            line1: r#"   _  _    "#,
            line2: r#" _| || |_  "#,
            line3: r#"|_  ..  _| "#,
            line4: r#"|_      _| "#,
            line5: r#"  |_||_|   "#,
            line6: r#"           "#,
        },
        '$' => Lettre {
            line1: r#"  _   "#,
            line2: r#" | |  "#,
            line3: r#"/ __) "#,
            line4: r#"\__ \ "#,
            line5: r#"(   / "#,
            line6: r#" |_|  "#,
        },
        '%' => Lettre {
            line1: r#" _  __ "#,
            line2: r#"(_)/ / "#,
            line3: r#"  / /  "#,
            line4: r#" / /_  "#,
            line5: r#"/_/(_) "#,
            line6: r#"       "#,
        },
        '&' => Lettre {
            line1: r#"  ___    "#,
            line2: r#" ( _ )   "#,
            line3: r#" / _ \/\ "#,
            line4: r#"| (_>  < "#,
            line5: r#" \___/\/ "#,
            line6: r#"         "#,
        },
        '\'' => Lettre {
            line1: r#" _  "#,
            line2: r#"( ) "#,
            line3: r#"|/  "#,
            line4: r#"    "#,
            line5: r#"    "#,
            line6: r#"    "#,
        },
        '(' => Lettre {
            line1: r#"  __  "#,
            line2: r#" / /  "#,
            line3: r#"| |   "#,
            line4: r#"| |   "#,
            line5: r#"| |   "#,
            line6: r#" \_\  "#,
        },
        ')' => Lettre {
            line1: r#"__   "#,
            line2: r#"\ \  "#,
            line3: r#" | | "#,
            line4: r#" | | "#,
            line5: r#" | | "#,
            line6: r#"/_/  "#,
        },
        '*' => Lettre {
            line1: r#"       "#,
            line2: r#"__/\__ "#,
            line3: r#"\    / "#,
            line4: r#"/_  _\ "#,
            line5: r#"  \/   "#,
            line6: r#"       "#,
        },
        '+' => Lettre {
            line1: r#"        "#,
            line2: r#"   _    "#,
            line3: r#" _| |_  "#,
            line4: r#"|_   _| "#,
            line5: r#"  |_|   "#,
            line6: r#"        "#,
        },
        ',' => Lettre {
            line1: "      ",
            line2: "      ",
            line3: "      ",
            line4: " _    ",
            line5: "( )   ",
            line6: "|/    ",
        },
        '-' => Lettre {
            line1: r#"        "#,
            line2: r#"        "#,
            line3: r#" _____  "#,
            line4: r#"|_____| "#,
            line5: r#"        "#,
            line6: r#"        "#,
        },
        '.' => Lettre {
            line1: r#"    "#,
            line2: r#"    "#,
            line3: r#"    "#,
            line4: r#" _  "#,
            line5: r#"(_) "#,
            line6: r#"    "#,
        },
        '/' => Lettre {
            line1: r#"    __ "#,
            line2: r#"   / / "#,
            line3: r#"  / /  "#,
            line4: r#" / /   "#,
            line5: r#"/_/    "#,
            line6: r#"       "#,
        },

        // numbers and  : ; < = > ? @
        '0' => Lettre {
            line1: r#"  ___   "#,
            line2: r#" / _ \  "#,
            line3: r#"| | | | "#,
            line4: r#"| |_| | "#,
            line5: r#" \___/  "#,
            line6: r#"        "#,
        },
        '1' => Lettre {
            line1: r#" __  "#,
            line2: r#"/_ | "#,
            line3: r#" | | "#,
            line4: r#" | | "#,
            line5: r#" |_| "#,
            line6: r#"     "#,
        },
        '2' => Lettre {
            line1: r#" ____   "#,
            line2: r#"|___ \  "#,
            line3: r#"  __) | "#,
            line4: r#" / __/  "#,
            line5: r#"|_____| "#,
            line6: r#"        "#,
        },
        '3' => Lettre {
            line1: r#" _____  "#,
            line2: r#"|___ /  "#,
            line3: r#"  |_ \  "#,
            line4: r#" ___) | "#,
            line5: r#"|____/  "#,
            line6: r#"        "#,
        },
        '4' => Lettre {
            line1: r#" _  _    "#,
            line2: r#"| || |   "#,
            line3: r#"| || |_  "#,
            line4: r#"|__   _| "#,
            line5: r#"   |_|   "#,
            line6: r#"         "#,
        },
        '5' => Lettre {
            line1: r#" ____   "#,
            line2: r#"| ___|  "#,
            line3: r#"|___ \  "#,
            line4: r#" ___) | "#,
            line5: r#"|____/  "#,
            line6: r#"        "#,
        },
        '6' => Lettre {
            line1: r#"  __    "#,
            line2: r#" / /_   "#,
            line3: r#"| '_ \  "#,
            line4: r#"| (_) | "#,
            line5: r#" \___/  "#,
            line6: r#"        "#,
        },
        '7' => Lettre {
            line1: r#" _____  "#,
            line2: r#"|___  | "#,
            line3: r#"   / /  "#,
            line4: r#"  / /   "#,
            line5: r#" /_/    "#,
            line6: r#"        "#,
        },
        '8' => Lettre {
            line1: r#"  ___   "#,
            line2: r#" ( _ )  "#,
            line3: r#" / _ \  "#,
            line4: r#"| (_) | "#,
            line5: r#" \___/  "#,
            line6: r#"        "#,
        },
        '9' => Lettre {
            line1: r#"  ___   "#,
            line2: r#" / _ \  "#,
            line3: r#"| (_) | "#,
            line4: r#" \__, | "#,
            line5: r#"   /_/  "#,
            line6: r#"        "#,
        },
        ':' => Lettre {
            line1: r#"    "#,
            line2: r#" _  "#,
            line3: r#"(_) "#,
            line4: r#" _  "#,
            line5: r#"(_) "#,
            line6: r#"    "#,
        },
        ';' => Lettre {
            line1: r#"    "#,
            line2: r#" _  "#,
            line3: r#"(_) "#,
            line4: r#" _  "#,
            line5: r#"( ) "#,
            line6: r#"|/  "#,
        },
        '<' => Lettre {
            line1: r#"  __ "#,
            line2: r#" / / "#,
            line3: r#"/ /  "#,
            line4: r#"\ \  "#,
            line5: r#" \_\ "#,
            line6: r#"     "#,
        },
        '=' => Lettre {
            line1: r#"        "#,
            line2: r#" _____  "#,
            line3: r#"|_____| "#,
            line4: r#"|_____| "#,
            line5: r#"        "#,
            line6: r#"        "#,
        },
        '>' => Lettre {
            line1: r#"__   "#,
            line2: r#"\ \  "#,
            line3: r#" \ \ "#,
            line4: r#" / / "#,
            line5: r#"/_/  "#,
            line6: r#"     "#,
        },
        '?' => Lettre {
            line1: r#" ___  "#,
            line2: r#"|__ \ "#,
            line3: r#"  / / "#,
            line4: r#" |_|  "#,
            line5: r#" (_)  "#,
            line6: r#"      "#,
        },
        '@' => Lettre {
            line1: r#"   ____   "#,
            line2: r#"  / __ \  "#,
            line3: r#" / / _` | "#,
            line4: r#"| | (_| | "#,
            line5: r#" \ \__,_| "#,
            line6: r#"  \____/  "#,
        },


        // big case letters
        'A' => Lettre {
            line1: r#"    _     "#,
            line2: r#"   / \    "#,
            line3: r#"  / _ \   "#,
            line4: r#" / ___ \  "#,
            line5: r#"/_/   \_\ "#,
            line6: r#"          "#,
        },
        'B' => Lettre {
            line1: r#" ____   "#,
            line2: r#"| __ )  "#,
            line3: r#"|  _ \  "#,
            line4: r#"| |_) | "#,
            line5: r#"|____/  "#,
            line6: r#"        "#,
        },
        'C' => Lettre {
            line1: r#"  ____  "#,
            line2: r#" / ___| "#,
            line3: r#"| |     "#,
            line4: r#"| |___  "#,
            line5: r#" \____| "#,
            line6: r#"        "#,
        },
        'D' => Lettre {
            line1: r#" ____   "#,
            line2: r#"|  _ \  "#,
            line3: r#"| | | | "#,
            line4: r#"| |_| | "#,
            line5: r#"|____/  "#,
            line6: r#"        "#,
        },
        'E' => Lettre {
            line1: r#" _____  "#,
            line2: r#"| ____| "#,
            line3: r#"|  _|   "#,
            line4: r#"| |___  "#,
            line5: r#"|_____| "#,
            line6: r#"        "#,
        },
        'F' => Lettre {
            line1: r#" _____  "#,
            line2: r#"|  ___| "#,
            line3: r#"| |_    "#,
            line4: r#"|  _|   "#,
            line5: r#"|_|     "#,
            line6: r#"        "#,
        },
        'G' => Lettre {
            line1: r#"  ____  "#,
            line2: r#" / ___| "#,
            line3: r#"| |  _  "#,
            line4: r#"| |_| | "#,
            line5: r#" \____| "#,
            line6: r#"        "#,
        },
        'H' => Lettre {
            line1: r#" _   _  "#,
            line2: r#"| | | | "#,
            line3: r#"| |_| | "#,
            line4: r#"|  _  | "#,
            line5: r#"|_| |_| "#,
            line6: r#"        "#,
        },
        'I' => Lettre {
            line1: r#" ___  "#,
            line2: r#"|_ _| "#,
            line3: r#" | |  "#,
            line4: r#" | |  "#,
            line5: r#"|___| "#,
            line6: r#"      "#,
        },
        'J' => Lettre {
            line1: r#"     _  "#,
            line2: r#"    | | "#,
            line3: r#" _  | | "#,
            line4: r#"| |_| | "#,
            line5: r#" \___/  "#,
            line6: r#"        "#,
        },
        'K' => Lettre {
            line1: r#" _  __ "#,
            line2: r#"| |/ / "#,
            line3: r#"| ' /  "#,
            line4: r#"| . \  "#,
            line5: r#"|_|\_\ "#,
            line6: r#"       "#,
        },
        'L' => Lettre {
            line1: r#" _      "#,
            line2: r#"| |     "#,
            line3: r#"| |     "#,
            line4: r#"| |___  "#,
            line5: r#"|_____| "#,
            line6: r#"        "#,
        },
        'M' => Lettre {
            line1: r#" __  __  "#,
            line2: r#"|  \/  | "#,
            line3: r#"| |\/| | "#,
            line4: r#"| |  | | "#,
            line5: r#"|_|  |_| "#,
            line6: r#"         "#,
        },
        'N' => Lettre {
            line1: r#" _   _  "#,
            line2: r#"| \ | | "#,
            line3: r#"|  \| | "#,
            line4: r#"| |\  | "#,
            line5: r#"|_| \_| "#,
            line6: r#"        "#,
        },
        'O' => Lettre {
            line1: r#"  ____   "#,
            line2: r#" / __ \  "#,
            line3: r#"| |  | | "#,
            line4: r#"| |__| | "#,
            line5: r#" \____/  "#,
            line6: r#"         "#,
        },
        'P' => Lettre {
            line1: r#" ____   "#,
            line2: r#"|  _ \  "#,
            line3: r#"| |_) | "#,
            line4: r#"|  __/  "#,
            line5: r#"|_|     "#,
            line6: r#"        "#,
        },
        'Q' => Lettre {
            line1: r#"  ___   "#,
            line2: r#" / _ \  "#,
            line3: r#"| | | | "#,
            line4: r#"| |_| | "#,
            line5: r#" \__\_\ "#,
            line6: r#"        "#,
        },
        'R' => Lettre {
            line1: r#" ____   "#,
            line2: r#"|  _ \  "#,
            line3: r#"| |_) | "#,
            line4: r#"|  _ <  "#,
            line5: r#"|_| \_\ "#,
            line6: r#"        "#,
        },
        'S' => Lettre {
            line1: r#" ____   "#,
            line2: r#"/ ___|  "#,
            line3: r#"\___ \  "#,
            line4: r#" ___) | "#,
            line5: r#"|____/  "#,
            line6: r#"        "#,
        },
        'T' => Lettre {
            line1: r#" _____  "#,
            line2: r#"|_   _| "#,
            line3: r#"  | |   "#,
            line4: r#"  | |   "#,
            line5: r#"  |_|   "#,
            line6: r#"        "#,
        },
        'U' => Lettre {
            line1: r#" _   _  "#,
            line2: r#"| | | | "#,
            line3: r#"| | | | "#,
            line4: r#"| |_| | "#,
            line5: r#" \___/  "#,
            line6: r#"        "#,
        },
        'V' => Lettre {
            line1: r#"__     __ "#,
            line2: r#"\ \   / / "#,
            line3: r#" \ \ / /  "#,
            line4: r#"  \ V /   "#,
            line5: r#"   \_/    "#,
            line6: r#"          "#,
        },
        'W' => Lettre {
            line1: r#"__        __ "#,
            line2: r#"\ \      / / "#,
            line3: r#" \ \ /\ / /  "#,
            line4: r#"  \ V  V /   "#,
            line5: r#"   \_/\_/    "#,
            line6: r#"             "#,
        },
        'X' => Lettre {
            line1: r#"__  __ "#,
            line2: r#"\ \/ / "#,
            line3: r#" \  /  "#,
            line4: r#" /  \  "#,
            line5: r#"/_/\_\ "#,
            line6: r#"       "#,
        },
        'Y' => Lettre {
            line1: r#"__   __ "#,
            line2: r#"\ \ / / "#,
            line3: r#" \ V /  "#,
            line4: r#"  | |   "#,
            line5: r#"  |_|   "#,
            line6: r#"        "#,
        },
        'Z' => Lettre {
            line1: r#" _____ "#,
            line2: r#"|__  / "#,
            line3: r#"  / /  "#,
            line4: r#" / /_  "#,
            line5: r#"/____| "#,
            line6: r#"       "#,
        },

        // [ \ ] ^ _ `
        '[' => Lettre {
            line1: r#" __  "#,
            line2: r#"| _| "#,
            line3: r#"| |  "#,
            line4: r#"| |  "#,
            line5: r#"| |  "#,
            line6: r#"|__| "#,
        },
        '\\' => Lettre {
            line1: r#"__     "#,
            line2: r#"\ \    "#,
            line3: r#" \ \   "#,
            line4: r#"  \ \  "#,
            line5: r#"   \_\ "#,
            line6: r#"       "#,
        },
        ']' => Lettre {
            line1: r#" __  "#,
            line2: r#"|_ | "#,
            line3: r#" | | "#,
            line4: r#" | | "#,
            line5: r#" | | "#,
            line6: r#"|__| "#,
        },
        '^' => Lettre {
            line1: r#" /\  "#,
            line2: r#"|/\| "#,
            line3: r#"     "#,
            line4: r#"     "#,
            line5: r#"     "#,
            line6: r#"     "#,
        },
        '_' => Lettre {
            line1: r#"        "#,
            line2: r#"        "#,
            line3: r#"        "#,
            line4: r#"        "#,
            line5: r#" _____  "#,
            line6: r#"|_____| "#,
        },
        '`' => Lettre {
            line1: r#" _  "#,
            line2: r#"( ) "#,
            line3: r#" \| "#,
            line4: r#"    "#,
            line5: r#"    "#,
            line6: r#"    "#,
        },

        // small case letters
        'a' => Lettre {
            line1: r#"        "#,
            line2: r#"  __ _  "#,
            line3: r#" / _` | "#,
            line4: r#"| (_| | "#,
            line5: r#" \__,_| "#,
            line6: r#"        "#,
        },
        'b' => Lettre {
            line1: r#" _      "#,
            line2: r#"| |__   "#,
            line3: r#"| '_ \  "#,
            line4: r#"| |_) | "#,
            line5: r#"|_.__/  "#,
            line6: r#"        "#,
        },
        'c' => Lettre {
            line1: r#"        "#,
            line2: r#"  ____  "#,
            line3: r#" / ___| "#,
            line4: r#"| (___  "#,
            line5: r#" \____| "#,
            line6: r#"        "#,
        },
        'd' => Lettre {
            line1: r#"     _  "#,
            line2: r#"  __| | "#,
            line3: r#" / _  | "#,
            line4: r#"| (_| | "#,
            line5: r#" \__,_| "#,
            line6: r#"        "#,
        },
        'e' => Lettre {
            line1: r#"        "#,
            line2: r#"  ___   "#,
            line3: r#" / _  \ "#,
            line4: r#"|  ___/ "#,
            line5: r#" \____| "#,
            line6: r#"        "#,
        },
        'f' => Lettre {
            line1: r#"  __  "#,
            line2: r#" / _| "#,
            line3: r#"| |_  "#,
            line4: r#"|  _| "#,
            line5: r#"|_|   "#,
            line6: r#"      "#,
        },
        'g' => Lettre {
            line1: r#"        "#,
            line2: r#"  __ _  "#,
            line3: r#" / _` | "#,
            line4: r#"| (_| | "#,
            line5: r#" \__, | "#,
            line6: r#" |___/  "#,
        },
        'h' => Lettre {
            line1: r#" _      "#,
            line2: r#"| |__   "#,
            line3: r#"| '_ \  "#,
            line4: r#"| | | | "#,
            line5: r#"|_| |_| "#,
            line6: r#"        "#,
        },
        'i' => Lettre {
            line1: r#" _  "#,
            line2: r#"(_) "#,
            line3: r#"| | "#,
            line4: r#"| | "#,
            line5: r#"|_| "#,
            line6: r#"    "#,
        },
        'j' => Lettre {
            line1: r#"   _  "#,
            line2: r#"  (_) "#,
            line3: r#"  | | "#,
            line4: r#"  | | "#,
            line5: r#" _/ | "#,
            line6: r#"|__/  "#,
        },
        'k' => Lettre {
            line1: r#" _     "#,
            line2: r#"| | __ "#,
            line3: r#"| |/ / "#,
            line4: r#"|   <  "#,
            line5: r#"|_|\_\ "#,
            line6: r#"       "#,
        },
        'l' => Lettre {
            line1: r#" _  "#,
            line2: r#"| | "#,
            line3: r#"| | "#,
            line4: r#"| | "#,
            line5: r#"|_| "#,
            line6: r#"    "#,
        },
        'm' => Lettre {
            line1: r#"            "#,
            line2: r#" _ __ ___   "#,
            line3: r#"| '_ ` _ \  "#,
            line4: r#"| | | | | | "#,
            line5: r#"|_| |_| |_| "#,
            line6: r#"            "#,
        },
        'n' => Lettre {
            line1: r#"        "#,
            line2: r#" _ __   "#,
            line3: r#"| '_ \  "#,
            line4: r#"| | | | "#,
            line5: r#"|_| |_| "#,
            line6: r#"        "#,
        },
        'o' => Lettre {
            line1: r#"        "#,
            line2: r#"  ___   "#,
            line3: r#" / _ \  "#,
            line4: r#"| (_) | "#,
            line5: r#" \___/  "#,
            line6: r#"        "#,
        },
        'p' => Lettre {
            line1: r#"        "#,
            line2: r#" _ __   "#,
            line3: r#"| '_ \  "#,
            line4: r#"| |_) | "#,
            line5: r#"| .__/  "#,
            line6: r#"|_|     "#,
        },
        'q' => Lettre {
            line1: r#"        "#,
            line2: r#"  __ _  "#,
            line3: r#" / _` | "#,
            line4: r#"| (_| | "#,
            line5: r#" \__, | "#,
            line6: r#"    |_| "#,
        },
        'r' => Lettre {
            line1: r#"       "#,
            line2: r#" _ __  "#,
            line3: r#"| '__| "#,
            line4: r#"| |    "#,
            line5: r#"|_|    "#,
            line6: r#"       "#,
        },
        's' => Lettre {
            line1: r#"      "#,
            line2: r#"  __  "#,
            line3: r#"/ __| "#,
            line4: r#"\__ \ "#,
            line5: r#"|___/ "#,
            line6: r#"      "#,
        },
        't' => Lettre {
            line1: r#" _    "#,
            line2: r#"| |_  "#,
            line3: r#"| __| "#,
            line4: r#"| |_  "#,
            line5: r#" \__| "#,
            line6: r#"      "#,
        },
        'u' => Lettre {
            line1: r#"        "#,
            line2: r#" _   _  "#,
            line3: r#"| | | | "#,
            line4: r#"| |_| | "#,
            line5: r#" \__,_| "#,
            line6: r#"        "#,
        },
        'v' => Lettre {
            line1: r#"        "#,
            line2: r#"__   __ "#,
            line3: r#"\ \ / / "#,
            line4: r#" \ V /  "#,
            line5: r#"  \_/   "#,
            line6: r#"        "#,
        },
        'w' => Lettre {
            line1: r#"           "#,
            line2: r#"__      __ "#,
            line3: r#"\ \ /\ / / "#,
            line4: r#" \ V  V /  "#,
            line5: r#"  \_/\_/   "#,
            line6: r#"           "#,
        },
        'x' => Lettre {
            line1: r#"       "#,
            line2: r#"__  __ "#,
            line3: r#"\ \/ / "#,
            line4: r#" >  <  "#,
            line5: r#"/_/\_\ "#,
            line6: r#"       "#,
        },
        'y' => Lettre {
            line1: r#"        "#,
            line2: r#" _   _  "#,
            line3: r#"| | | | "#,
            line4: r#"| |_| | "#,
            line5: r#" \__, | "#,
            line6: r#" |___/  "#,
        },
        'z' => Lettre {
            line1: r#"      "#,
            line2: r#" ____ "#,
            line3: r#"|_  / "#,
            line4: r#" / /  "#,
            line5: r#"/___| "#,
            line6: r#"      "#,
        },

        // { | } ~ and that's about it
        '{' => Lettre {
            line1: r#"    __ "#,
            line2: r#"   / / "#,
            line3: r#"  | |  "#,
            line4: r#" < <   "#,
            line5: r#"  | |  "#,
            line6: r#"   \_\ "#,
        },
        '|' => Lettre {
            line1: r#" _  "#,
            line2: r#"| | "#,
            line3: r#"| | "#,
            line4: r#"| | "#,
            line5: r#"| | "#,
            line6: r#"|_| "#,
        },
        '}' => Lettre {
            line1: r#"__    "#,
            line2: r#"\ \   "#,
            line3: r#" | |  "#,
            line4: r#"  > > "#,
            line5: r#" | |  "#,
            line6: r#"/_/   "#,
        },
        '~' => Lettre {
            line1: r#"      "#,
            line2: r#"      "#,
            line3: r#" /\/| "#,
            line4: r#"|/\/  "#,
            line5: r#"      "#,
            line6: r#"      "#,
        },

        // some fancy french letters
        'à' => Lettre {
            line1: r#"   \_\  "#,
            line2: r#"  __ _  "#,
            line3: r#" / _` | "#,
            line4: r#"| (_| | "#,
            line5: r#" \__,_| "#,
            line6: r#"        "#,
        },
        'ç' => Lettre {
            line1: r#"        "#,
            line2: r#"  ____  "#,
            line3: r#" / ___| "#,
            line4: r#"| (___  "#,
            line5: r#" \_  _| "#,
            line6: r#"   |/   "#,
        },
        'é' => Lettre {
            line1: r#"    __  "#,
            line2: r#"  _/_/  "#,
            line3: r#" / _  \ "#,
            line4: r#"|  ___/ "#,
            line5: r#" \____| "#,
            line6: r#"        "#,
        },
        'è' => Lettre {
            line1: r#"   __   "#,
            line2: r#"  _\ \  "#,
            line3: r#" / _  \ "#,
            line4: r#"|  ___/ "#,
            line5: r#" \____| "#,
            line6: r#"        "#,
        },
        'ê' => Lettre {
            line1: r#"  /_\   "#,
            line2: r#"  ___   "#,
            line3: r#" / _  \ "#,
            line4: r#"|  ___/ "#,
            line5: r#" \____| "#,
            line6: r#"        "#,
        },
        // En cas d'erreur, on renvoie un gros X
        _ => Lettre {
                line1: "OO    OO",
                line2: " OO  OO ",
                line3: "  OOOO  ",
                line4: "  OOOO  ",
                line5: " OO  OO ",
                line6: "OO    OO",
            },
    }
}
