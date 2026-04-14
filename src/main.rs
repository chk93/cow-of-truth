use std::io;

fn main() {
    let version = "alpha 0.2.0";
    println!("{}", version);
    println!("Введите имя друна:");

    let faggot = "пидорас";
    let fat = "жирный";

    let inputmenu = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Иди нахуй");
        input.trim().to_string()
    };

    let fatm = loop {
        let answer = fatmenu();
        if answer == "Да" || answer == "Нет" {
            break answer;
        } else {
            println!("Ошибка: ответ только Да/Нет, соблюдая регистр");
        }
    };

    if fatm == "Да" {
        println!("Думаю...");
        println!("{} {} {}", inputmenu, fat, faggot); 
    } else {
        println!("Думаю...");
        println!("{} {}", inputmenu, faggot);
    }
}

fn fatmenu() -> String {
    println!("Ваш друн жирни? Да/Нет");

    let mut fatm = String::new();
    io::stdin().read_line(&mut fatm).expect("Иди нахуй");
    fatm.trim().to_string()
}