use crate::objeto::Objeto;

#[derive(Debug)]
pub struct Cena {
    pub id: String,
    pub nome: String,
    pub texto: String,
    pub objetos: Vec<Objeto>,
}

impl Cena {

    pub fn new(id: String, nome: String, texto: String) -> Self {
        Cena {
            id,
            nome,
            texto,
            objetos: Vec::new(),
        }
    }

    pub fn print_info(&self) {
        println!("Cena {}: {}", self.id, self.nome);
        println!("{}\n", self.texto);
    }

    pub fn append_obj(&mut self, objeto: Objeto) {
        self.objetos.push(objeto);
    }
}