use crate::party;
use colored::*;
use party::file as F;
use party::random_game as R_G;
use party::Crash as C;
use party::Robot as R;
use party::Terrain as T;
use std::io;
use std::{thread, time};

pub fn choice(mut robot: &mut Vec<R>) -> T {
    //La fonction permettra à l'utilisateur de choisir entre sélectionner le monde aléatoire
    //ou le monde généré par le fichier "two_robots.txt"
    //La variable ten_millis va permettre de définir le temps en ms qu'on voudrais que le programme
    //s'arrete
    let ten_millis = time::Duration::from_millis(2000);
    //Cette boucle ne s'arretera uniquement lorsque l'utilisateur aura entré une réponse valide
    loop {
        //Nous permet de lire ce qu'on écrit dans le terminal
        //On n'oublie pas le trim qui permettre de retirer le retour chariot
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.as_str().trim() {
                "Y" | "y" => {
                    println!(
                        "{}\n         {}{}{}",
                        "Generation du monde aléatoire en cours 🌍 ...\n".green(),
                        "└[∵┌]".cyan().bold(),
                        "└[ ∵ ]┘".yellow().bold(),
                        "[┐∵]┘".magenta().bold()
                    );
                    //On met en pause le programme 2000ms (valeur entré dans la variable "ten_millis")
                    thread::sleep(ten_millis);
                    //On return en executant la fonction "random_world"
                    return R_G::random_world(&mut robot);
                }
                "N" | "n" => {
                    println!(
                        "{}\n         {}{}{}\n{}",
                        "Generation du monde en cours 🌍 ...\n".green(),
                        "└[∵┌]".red().bold(),
                        "└[ ∵ ]┘".blue().bold(),
                        "[┐∵]┘".green().bold(),
                        "Lecture du fichier : 'two_robots.txt' en cours..."
                    );
                    //On met en pause le programme 2000ms (valeur entré dans la variable "ten_millis")
                    thread::sleep(ten_millis);
                    //On return en executant la fonction "file"
                    return F::file(&mut robot);
                }
                _ => println!("Y/N ?"),
            },
            Err(error) => println!("????: {}", error),
        }
    }
}

pub fn initial_final(robot: &mut Vec<R>, position: String) {
    //Cette fonction va afficher tout les robots contenu dans le vecteur un par un
    for i in 0..robot.len() {
        let id = robot[i].id;
        //On le parse pour mettre des jolies couleurs
        let s = id.to_string();
        if robot[i].instruction.is_empty() {
            println!(
                "Position {} du Robot<{}> -> chez lui (s'est pris une cuite)",
                position,
                s.green()
            )
        } else {
            println!(
                "Position {} du Robot<{}> -> x : {} y : {}",
                position,
                s.magenta().bold(),
                robot[i].x,
                robot[i].y
            );
        }
    }
}

pub fn display(robot: &mut Vec<R>, terrain: &mut T) {
    //Cette fonction va afficher le contenu des variables grâce à l'implémentation
    //du trait Display
    println!("Musique : 'HATARI - ENGIN MISKUNN' 🎶");
    println!("Piste de danse {{ {} }}", terrain);
    println!("Robots [");
    let len = robot.len();
    //Iterator is beautiful
    for bot in robot {
        println!("  {{ {}, }}", bot);
    }
    println!("]");
    if len == 1 {
        println!("Wouhou...")
    }
}

pub fn display_crash(crash: Vec<C>) {
    //Cette fonction va afficher le nombres de crash eu durant la soirée
    //ou pas
    if crash.is_empty() {
        println!(
            "La soirée s'est bien passé, aucun incident à déplorer\n{}",
            "signé : La Direction 🕶".black().bold()
        );
    } else {
        for aie in crash.iter() {
            println!("{}", aie);
        }
    }
}
