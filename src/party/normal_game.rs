use crate::party;
use rand::Rng;

pub fn game(mut robot: &mut Vec<party::Robot>, mut terrain: party::Terrain) {
    //Le vecteur crash va contenir touts les accidents sous forme de String
    //La variable tmp va nous permettre de garder les coordonnées avant execution de l'instruction
    //La variable obstacle va contenir un vecteur contenant les coordonnées des obstacles généré par la fonction "create_barrier"
    let mut crash = Vec::new();
    let mut tmp: (i32, i32);
    let mut obstacle = create_barrier(&mut terrain);
    let mut size = taille(robot);
    let mut x = 0;
    //On crée une boucle qui s'arrete jusqu'à que la plus grande liste d'instructions n'ai plus d'instructions
    loop {
        //Dans cette boucle, on va envoyer notre liste d'instruction un par un dans la fonction instruction
        //qui va nous réaliser les mouvements du robot
        for i in 0..robot.len() {
            if x < robot[i].instruction.len() {
                tmp = (robot[i].x, robot[i].y);
                party::instructions::instruction(robot[i].instruction[x], &mut robot[i]);
                //Si il y a qu'un seul robot présent dans le vecteur
                //On aura pas besoin de l'envoyer à la fonction crash
                if robot.len() > 1 {
                    party::rules::crash(tmp, &mut robot, i, &mut crash);
                }
                //Les fonctions limit et obstacles vont contenir tout les
                //accidents durant la soirée
                party::rules::limit(tmp, &mut robot, i, &mut crash, &mut terrain);
                party::rules::obstacle(&mut obstacle, &mut robot, i, &mut crash, &mut terrain);
            }
        }

        //On regarde si la taille de la plus grande liste d'instruction n'a pas changé
        //Si elle a changé (à cause de l'obstacle en feu) on change cette taille
        if size != taille(robot) {
            size = taille(robot);
        }
        //On incrémente jusqu'à que x soit égale à la taille du robot
        if x != size {
            x += 1;
        } else {
            break;
        }
    }
    //Nous affiche ou pas les accidents survenu durant la nuit
    party::display::display_crash(crash);
}

pub fn taille(robot: &mut Vec<party::Robot>) -> usize {
    //Simple programme qui nous renvoie la taille de la plus grande liste
    //d'instruction d'un robot
    let mut taille = robot[0].instruction.len();
    for i in 0..robot.len() - 1 {
        if taille > robot[i + 1].instruction.len() {
            taille = robot[i].instruction.len();
        } else {
            taille = robot[i + 1].instruction.len()
        }
    }
    return taille;
}

pub fn create_robot(robot: &mut Vec<party::Robot>, c: &mut Vec<&str>, id: i32, vide: char) {
    //Le programme va parser tout les chaines de caracteres qu'il a reçu dans la variable c
    //On commence par initialiser toutes les variables que va recevoir un robot
    let mut robot_instruction: Vec<&party::Instruction> = Vec::new();
    let mut position: Vec<i32> = Vec::new();
    let mut orientation = party::Orientation::North;

    //On sait que les 2 premiers strings envoyé par file sont les positions initiaux du robot
    for i in 0..2 {
        let _number = match c[i].parse::<i32>() {
            Ok(number) => position.push(number),
            Err(_) => (),
        };
    }
    //Le 3e string sera bien evidemment l'orientation du robot
    //En fonction du string, le programme va envoyé au robot l'équivalent du string en variable
    match c[2] {
        "N" => orientation = party::Orientation::North,
        "E" => orientation = party::Orientation::Est,
        "W" => orientation = party::Orientation::West,
        "S" => orientation = party::Orientation::South,
        _ => println!("C'est une Orientation ???? {}", c[2]),
    }
    //Dans cette partie du programme, on compare si le char reçu par la fonction "file"
    //dans la variable "vide" est 'O' ou 'N' ce qui va nous permettre de savoir si le
    //vecteur c contient des instructions ou pas, si on a reçu le char "O" le vecteur c en contient
    //si 'O' alors il en a pas donc on commence à générer des instructions aléatoires
    //si 'N' alors il commence à push dans le vecteur robot_instruction l'équivalent du string en variable
    if vide == 'N' {
        let instruction: Vec<char> = c[3].chars().collect();
        for i in 0..instruction.len() {
            match instruction[i] {
                'F' => robot_instruction.push(&party::Instruction::F),
                'R' => robot_instruction.push(&party::Instruction::R),
                'L' => robot_instruction.push(&party::Instruction::L),
                _ => {
                    println!("C'est une instruction ???? {} ", instruction[i]);
                    break;
                }
            }
        }
    } else if vide == 'O' {
        let mut rng = rand::thread_rng();
        let nbre_instruction = rng.gen_range(1, 10);
        for _ in 0..nbre_instruction {
            let aleatoire = rng.gen_range(0, 3);
            match aleatoire {
                0 => robot_instruction.push(&party::Instruction::F),
                1 => robot_instruction.push(&party::Instruction::R),
                2 => robot_instruction.push(&party::Instruction::L),
                _ => (),
            }
        }
    }
    //On push dans le vecteur robot tout les informations parsé
    //Ce qui fait que à chaque utilisation de la fonction "create_robot"
    //on a un robot creé
    robot.push(party::Robot {
        id: id,
        x: position[1],
        y: position[0],
        orientation: orientation,
        instruction: robot_instruction,
    });
}

fn create_barrier(terrain: &mut party::Terrain) -> Vec<party::Obstacle> {
    //Cette fonction va crée des obstacles aléatoirement sur la map
    let mut rng = rand::thread_rng();
    //Le nombre d'obstacle sera proportionel à la taille max de case du terrain
    let mut max = (terrain.x * terrain.y) / 3;
    if max == 0 {
        max = 1;
    }
    let mut obstacle = Vec::new();
    //Il n'y aura que 3 type d'obstacle faute d'idée
    let mut i = 0;
    for _ in 0..rng.gen_range(0, max) {
        obstacle.push(party::Obstacle {
            x: rng.gen_range(0, terrain.x),
            y: rng.gen_range(0, terrain.y),
            id: i,
        });

        i += 1;
        if i > 2 {
            i = 0;
        }
    }
    return obstacle;
}