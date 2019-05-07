// 2019-05-07

// Chaque lettre est un struct de six lignes, des

use std::fmt;

pub struct Lettre {
    pub ligne1: String,
    pub ligne2: String,
    pub ligne3: String,
    pub ligne4: String,
    pub ligne5: String,
    pub ligne6: String,
}
//
// let lettrea = Lettre {
//      ligne1: String::from("        "),
//      ligne2: String::from("  __ _  "),
//      ligne3: String::from(" / _` | "),
//      ligne4: String::from("| ( | | "),
//      ligne5: String::from(" ___,_| "),
//      ligne6: String::from("        "),
// }
//
// let lettreb = Lettre {
//      ligne1: String::from(" _      "),
//      ligne2: String::from("| |_    "),
//      ligne3: String::from("| '_    "),
//      ligne4: String::from("| |_) | "),
//      ligne5: String::from("|_.__/  "),
//      ligne6: String::from("        "),
// }

impl Lettre {

    // Ici on compilera les instances de lettres pour faire un grand struct
    // fn concatener(&self) -> Self {
    //
    // }
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
