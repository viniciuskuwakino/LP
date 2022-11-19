#[derive(Debug)]
pub struct Objeto {
    pub id: String,
    pub tipo: String,
    pub nome: String,
    pub texto: String,
    pub positivo: String,
    pub negativo: String,
    pub comando: String,
    pub cena_alvo: String,
}

impl Objeto {
    pub fn new(
        id: String,
        tipo: String,
        nome: String,
        texto: String,
        positivo: String,
        negativo: String,
        comando: String,
        cena_alvo: String,
    ) -> Self {
        Objeto {
            id,
            tipo,
            nome,
            texto,
            positivo,
            negativo,
            comando,
            cena_alvo,
        }
    }
    
    pub fn objeto_vazio() -> Self {
        Objeto {
            id: "".to_string(),
            tipo: "".to_string(),
            nome: "".to_string(),
            texto: "".to_string(),
            positivo: "".to_string(),
            negativo: "".to_string(),
            comando: "".to_string(),
            cena_alvo: "".to_string()
        }
    }

}

impl Clone for Objeto {
    fn clone(&self) -> Self {
        Objeto::new(
            self.id.clone(),
            self.tipo.clone(),
            self.nome.clone(),
            self.texto.clone(),
            self.positivo.clone(),
            self.negativo.clone(),
            self.comando.clone(),
            self.cena_alvo.clone()
        )
    }
}
