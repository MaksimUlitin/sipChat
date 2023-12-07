use std::io;

struct Player {
    health: u32,
    energy: u32,
}

impl Player {
    fn new() -> Player {
        Player { health: 100, energy: 100 }
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }

    fn show_status(&self) {
        println!("Здоровье: {} Энергия: {}", self.health, self.energy);
    }
}

enum Location {
    Base,
    Canyon,
    Oasis,
}

fn main() {
    let mut player = Player::new();
    let mut current_location = Location::Base;

    println!("Добро пожаловать на Марс!");

    while player.is_alive() {
        player.show_status();

        println!("Выберите действие:");
        println!("1. Исследовать текущую локацию");
        println!("2. Переместиться в другую локацию");
        println!("3. Отдохнуть");
        println!("4. Выйти");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Не удалось прочитать строку");

        match input.trim() {
            "1" => explore_location(&mut player, &current_location),
            "2" => current_location = change_location(&current_location),
            "3" => rest(&mut player),
            "4" => break,
            _ => println!("Некорректный ввод."),
        }
    }

    println!("Игра завершена. Спасибо за игру!");
}

fn explore_location(player: &mut Player, location: &Location) {
    match location {
        Location::Base => {
            println!("Вы находитесь в базе. Ничего интересного здесь.");
        }
        Location::Canyon => {
            println!("Вы нашли каньон. У вас уходит немного энергии.");
            player.energy -= 10;
        }
        Location::Oasis => {
            println!("Вы нашли оазис. Ваше здоровье восстанавливается.");
            player.health += 20;
        }
    }
}

fn change_location(current_location: &Location) -> Location {
    match current_location {
        Location::Base => Location::Canyon,
        Location::Canyon => Location::Oasis,
        Location::Oasis => Location::Base,
    }
}

fn rest(player: &mut Player) {
    println!("Вы отдыхаете и восстанавливаете энергию.");
    player.energy += 20;
}
