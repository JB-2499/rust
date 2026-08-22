mod pessoa;

use pessoa::Pessoa;

fn main() {
    let mut person = Pessoa::new(String::from("João"), 19, true);

    println!("Informações da pessoa:");
    println!("> Nome: {}", person.nome());
    println!("> Idade: {} anos.", person.idade());
    println!(
        "> Estado civil: {}.",
        if person.casado() {"casado"} else {"solteiro"}
    );
}
