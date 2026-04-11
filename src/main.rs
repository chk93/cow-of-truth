use std::io;

fn main() {

    let version = "alpha 0.1.0";
    println!("{}", version);
    println!("Введите имя друна:");

    let faggot = "пидорас";
    let fat = "жирный";

    let mut inputmenu = String::new();
    io::stdin().read_line(&mut inputmenu).expect("Иди нахуй");

    println!("Ваш друн жирни? Да/Нет");
    let mut fatmenu = String::new();
    io::stdin().read_line(&mut fatmenu).expect("Иди нахуй");
    let fatmenu = fatmenu.trim();

    if fatmenu == "Да" {
         println!("Ваш друн жирни? Да/Нет");
         let inputmenu = inputmenu.trim();
         println!("Думаю...");
         println!("{} {} {}", inputmenu, fat, faggot); 
    } else if fatmenu == "Нет"{
         let inputmenu = inputmenu.trim();
         println!("Думаю...");
         println!("{} {}", inputmenu, faggot); 
    } else {
        println!("неправильное написание");
    }

}
