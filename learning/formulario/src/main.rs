use std::io;
use std::fmt;

fn main() {
    println!("Bem-vindo ao formulário digital!");
    println!("Digite seu nome: ");
    
    let nome: String = entrada();

    let idade: u32 = loop {
        println!("\nDigite sua idade: ");

        let entrada = entrada();

        match entrada.parse::<u32>() {
            Ok(valor) => break valor,
            Err(_) => {
                println!("Digite apenas números.");
            }
        }
    };
    
    let cpf = ler_cpf();

    print_form(&nome, &cpf, &idade);
}


//|-=-=-=-=-=-=-=-=-=|Functions|=-=-=-=-=-=-=-=-=-=-=|
fn entrada() -> String {
    let mut texto = String::new();

    io::stdin()
        .read_line(&mut texto)
        .expect("Erro ao receber a informação.");
    
    texto.trim().to_string()
}

fn ler_cpf() -> Cpf {
    loop {
        println!("\nDigite seu CPF: ");

        let entrada = entrada();

        let cpf = limpar_cpf(&entrada);

        if cpf.len() == 11 {
            break Cpf(cpf);
        }
    
        println!("CPF inválido! Digte todos os 11 digitos.");
    }
}

fn limpar_cpf(cpf: &str) -> String {
    cpf.chars().filter(|c| c.is_numeric()).collect()
}

fn print_form(nome: &String, cpf: &Cpf, idade: &u32) {
    println!("|-=-=-=-=-=-=-=-=-=-=|Formulário|-=-=-=-=-=-=-=-=-=-=|");

    println!("\nNome: {nome}");
    println!("\nidade: {idade}");
    println!("\nCpf: {cpf}");

    println!("\n|-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=|");
}


//|-=-=-=-=-=-=-=-=-|Structs/Types|=-=-=-=-=-=-=--=-=-|
#[derive(Debug)]
struct Cpf(String);

impl fmt::Display for Cpf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let cpf = limpar_cpf(&self.0);

        if cpf.len() != 11 {
            return write!(f, "CPF inválido.");
        }

        write!(
            f, "{}.{}.{}-{}",
            &cpf[0..3],
            &cpf[3..6],
            &cpf[6..9],
            &cpf[9..11]
        )
    }
}
