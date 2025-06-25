use std::io;

struct Livre {
    titre: String,
    auteur: String,
    annee: u32,
    disponible: bool,
}

fn ajouter_livre(bibliotheque: &mut Vec<Livre>) {
    let mut titre = String::new();
    let mut auteur = String::new();
    let mut annee = String::new();

    println!("Titre :");
    io::stdin().read_line(&mut titre).unwrap();

    println!("Auteur :");
    io::stdin().read_line(&mut auteur).unwrap();

    println!("Année :");
    io::stdin().read_line(&mut annee).unwrap();

    let annee_num = annee.trim().parse().unwrap_or(0);

    let livre = Livre {
        titre: titre.trim().to_string(),
        auteur: auteur.trim().to_string(),
        annee: annee_num,
        disponible: true,
    };

    bibliotheque.push(livre);
}

fn emprunter_livre(bibliotheque: &mut Vec<Livre>) {
    let mut titre = String::new();
    println!("Titre à emprunter :");
    io::stdin().read_line(&mut titre).unwrap();
    let titre = titre.trim();

    for livre in bibliotheque.iter_mut() {
        if livre.titre == titre && livre.disponible {
            livre.disponible = false;
            println!("Livre emprunté !");
            return;
        }
    }

    println!("Ce livre n’est pas disponible.");
}

fn retourner_livre(bibliotheque: &mut Vec<Livre>) {
    let mut titre = String::new();
    println!("Titre à retourner :");
    io::stdin().read_line(&mut titre).unwrap();
    let titre = titre.trim();

    for livre in bibliotheque.iter_mut() {
        if livre.titre == titre && !livre.disponible {
            livre.disponible = true;
            println!("Livre retourné !");
            return;
        }
    }

    println!("Ce livre est déjà disponible ou introuvable.");
}

fn afficher_tous_les_livres(bibliotheque: &Vec<Livre>) {
    println!("Tous les livres :");
    for livre in bibliotheque {
        let etat = if livre.disponible { "Disponible" } else { "Emprunté" };
        println!("{} - {} ({}) [{}]", livre.titre, livre.auteur, livre.annee, etat);
    }
}

fn afficher_livres_disponibles(bibliotheque: &Vec<Livre>) {
    println!("Livres disponibles :");
    for livre in bibliotheque {
        if livre.disponible {
            println!("{} - {} ({})", livre.titre, livre.auteur, livre.annee);
        }
    }
}

fn main() {
    let mut bibliotheque: Vec<Livre> = Vec::new();

    loop {
        println!();
        println!("--- MENU ---");
        println!("1. Ajouter un livre");
        println!("2. Emprunter un livre");
        println!("3. Retourner un livre");
        println!("4. Afficher tous les livres");
        println!("5. Afficher les livres disponibles");
        println!("6. Quitter");

        let mut choix = String::new();
        println!("Votre choix :");
        io::stdin().read_line(&mut choix).unwrap();

        match choix.trim() {
            "1" => ajouter_livre(&mut bibliotheque),
            "2" => emprunter_livre(&mut bibliotheque),
            "3" => retourner_livre(&mut bibliotheque),
            "4" => afficher_tous_les_livres(&bibliotheque),
            "5" => afficher_livres_disponibles(&bibliotheque),
            "6" => {
                println!("À bientôt !");
                break;
            }
            _ => println!("Choix non valide."),
        }
    }
}
