use std::fmt;

pub struct Pessoa {
    nome: String,
    idade: u32,
    casado: bool,
}

impl Pessoa {
    pub fn new(nome: String, idade: u32, casado: bool) -> Pessoa {
        Pessoa {
            nome,
            idade,
            casado,
        }
    }

    pub fn nome(&self) -> &str {
        &self.nome
    }

    pub fn idade(&self) -> u32 {
        self.idade
    }

    pub fn casado(&self) -> bool {
        self.casado
    }

    pub fn set_nome(&mut self, nome: String) {
       self.nome = nome;
    }

    pub fn set_idade(&mut self, idade: u32) {
        self.idade = idade;
    }

    pub fn set_casado(&mut self, casado: bool) {
        self.casado = casado;
    }
}

impl fmt::Display for Pessoa {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\nPessoa {{
            nome: {},
            idade: {},
            casado: {},
        }}",
        self.nome, self.idade, self.casado)
    }
}
