use std::fs::{File, remove_file, OpenOptions};
use std::io::{self, Read, Write};
use chrono::Local;

struct Gestionnaire;

impl Gestionnaire {
    fn creer_fichier(nom: &str, contenu: &str) -> io::Result<()> {
        let mut file = File::create(nom)?;
        file.write_all(contenu.as_bytes())?;
        println!("Fichier '{}' créé avec succès à {}", nom, Local::now());
        Ok(())
    }

    fn lire_fichier(nom: &str) -> io::Result<()> {
        let mut contenu = String::new();
        File::open(nom)?.read_to_string(&mut contenu)?;
        println!("Contenu de '{}':\n{}", nom, contenu);
        Ok(())
    }

    fn modifier_fichier(nom: &str, nouveau_contenu: &str) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(nom)?;
        file.write_all(nouveau_contenu.as_bytes())?;
        println!("Fichier '{}' modifié à {}", nom, Local::now());
        Ok(())
    }

    fn supprimer_fichier(nom: &str) -> io::Result<()> {
        remove_file(nom)?;
        println!("Fichier '{}' supprimé à {}", nom, Local::now());
        Ok(())
    }
}

fn main() {
    loop {
        println!("\n=== GESTIONNAIRE DE FICHIERS ===");
        println!("1. Créer un fichier");
        println!("2. Lire un fichier");
        println!("3. Modifier un fichier");
        println!("4. Supprimer un fichier");
        println!("5. Quitter");

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur lecture");
        let choix = choix.trim();

        match choix {
            "1" => {
                let (nom, contenu) = demander_infos();
                if let Err(e) = Gestionnaire::creer_fichier(&nom, &contenu) {
                    println!("Erreur: {}", e);
                }
            }
            "2" => {
                let nom = demander_nom();
                if let Err(e) = Gestionnaire::lire_fichier(&nom) {
                    println!("Erreur: {}", e);
                }
            }
            "3" => {
                let (nom, nouveau) = demander_infos();
                if let Err(e) = Gestionnaire::modifier_fichier(&nom, &nouveau) {
                    println!("Erreur: {}", e);
                }
            }
            "4" => {
                let nom = demander_nom();
                if let Err(e) = Gestionnaire::supprimer_fichier(&nom) {
                    println!("Erreur: {}", e);
                }
            }
            "5" => {
                println!("Au revoir !");
                break;
            }
            _ => println!("Choix invalide"),
        }
    }
}

fn demander_nom() -> String {
    println!("Entrez le nom du fichier :");
    let mut nom = String::new();
    io::stdin().read_line(&mut nom).expect("Erreur");
    nom.trim().to_string()
}

fn demander_infos() -> (String, String) {
    let nom = demander_nom();
    println!("Entrez le contenu :");
    let mut contenu = String::new();
    io::stdin().read_line(&mut contenu).expect("Erreur");
    (nom, contenu.trim().to_string())
}
