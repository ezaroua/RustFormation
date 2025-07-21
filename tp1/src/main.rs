use std::io;
/*
struct Compte {
    nom: String,
    solde: f64,
}


fn afficher_solde(compte: &Compte) {
    println!("Solde du compte {}: {:.2}€", compte.nom, compte.solde);
}


fn retrait(compte: &mut Compte, montant: f64) {
    if montant <= compte.solde {
        compte.solde -= montant;
        println!("Retrait de {:.2}€ effectué. Nouveau solde: {:.2}€", montant, compte.solde);
    } else {
        println!("Fonds insuffisants !");
    }
}

fn liste_comptes(comptes: &Vec<Compte>) {
    println!("\nListe des comptes :");
    for c in comptes {
        println!("- {}", c.nom);
    }
}

fn selectionner_compte<'a>(comptes: &'a mut Vec<Compte>) -> Option<&'a mut Compte> {
    println!("Entrez le nom du compte : ");
    let mut nom = String::new();
    io::stdin().read_line(&mut nom).expect("Erreur");
    let nom = nom.trim();

    for c in comptes.iter_mut() {
        if c.nom == nom {
            return Some(c);
        }
    }

    println!("Compte introuvable !");
    None
}

fn main() {
    let mut comptes = vec![
        Compte { nom: "Ilias".to_string(), solde: 1000.0 },
        Compte { nom: "Amin".to_string(), solde: 500.0 },
    ];

    let options = ["Afficher solde", "Retrait", "Liste comptes", "Quitter"];

    loop {
        println!("\n=== Menu ===");
        for (i, option) in options.iter().enumerate() {
            println!("{} - {}", i + 1, option);
        }

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur de lecture");
        let choix = choix.trim().parse::<u32>();

        match choix {
            Ok(1) => {
                if let Some(c) = selectionner_compte(&mut comptes) {
                    afficher_solde(c);
                }
            }
            Ok(2) => {
                if let Some(c) = selectionner_compte(&mut comptes) {
                    println!("Montant à retirer : ");
                    let mut montant = String::new();
                    io::stdin().read_line(&mut montant).expect("Erreur");
                    if let Ok(montant_f) = montant.trim().parse::<f64>() {
                        retrait(c, montant_f);
                    } else {
                        println!("Montant invalide");
                    }
                }
            }
            Ok(3) => {
                liste_comptes(&comptes);
            }
            Ok(4) => {
                println!("Au revoir !");
                break;
            }
            _ => println!("Choix invalide."),
        }
    }
}
*/
struct Compte {
    nom: String,
    solde: f64,
}

impl Compte {
    fn afficher_solde(&self) {
        println!("Solde du compte {}: {:.2}€", self.nom, self.solde);
    }

    fn retrait(&mut self, montant: f64) {
        if montant <= self.solde {
            self.solde -= montant;
            println!("Retrait de {:.2}€ effectué. Nouveau solde: {:.2}€", montant, self.solde);
        } else {
            println!("Fonds insuffisants pour le retrait.");
        }
    }
}

fn main() {
    let mut comptes = vec![
        Compte { nom: String::from("Ilias"), solde: 1000.0 },
        Compte { nom: String::from("Amin"), solde: 500.0 },
    ];

    let options = ["Afficher solde", "Retrait", "Liste comptes", "Quitter"];

    loop {
        println!("\n=== Menu ===");
        for (i, option) in options.iter().enumerate() {
            println!("{} - {}", i + 1, option);
        }

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur de lecture");
        let choix = choix.trim().parse::<u32>();

        match choix {
            Ok(1) => {
                let compte = selectionner_compte(&mut comptes);
                if let Some(c) = compte {
                    c.afficher_solde();
                }
            }
            Ok(2) => {
                let compte = selectionner_compte(&mut comptes);
                if let Some(c) = compte {
                    println!("Montant à retirer : ");
                    let mut montant = String::new();
                    io::stdin().read_line(&mut montant).expect("Erreur");
                    if let Ok(montant_f) = montant.trim().parse::<f64>() {
                        c.retrait(montant_f);
                    } else {
                        println!("Montant invalide");
                    }
                }
            }
            Ok(3) => {
                println!("\nListe des comptes :");
                for c in &comptes {
                    println!("- {}", c.nom);
                }
            }
            Ok(4) => {
                println!("Au revoir !");
                break;
            }
            _ => println!("Choix invalide"),
        }
    }
}

fn selectionner_compte<'a>(comptes: &'a mut Vec<Compte>) -> Option<&'a mut Compte> {
    println!("Entrez le nom du compte : ");
    let mut nom = String::new();
    io::stdin().read_line(&mut nom).expect("Erreur");
    let nom = nom.trim();

    for c in comptes.iter_mut() {
        if c.nom == nom {
            return Some(c);
        }
    }
    println!("Compte introuvable !");
    None
}




