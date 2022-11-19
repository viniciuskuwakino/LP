use crate::objeto::Objeto;

#[derive(Debug)]
pub struct Inventario {
    pub obj_inventory: Vec<Objeto>,
}

impl Inventario {
    pub fn new() -> Self {
        Inventario { obj_inventory: Vec::new() }
    }

    pub fn obter_objeto(&mut self, objeto: Objeto) {
        self.obj_inventory.push(objeto);
    }

    // Confere se o objeto está no inventario
    pub fn conferir(&mut self, nome_obj: String) -> bool {
        let mut boolean = false;
        for i in &self.obj_inventory {
            if i.nome == nome_obj { boolean = true }
        }

        boolean
    }

    pub fn mostrar_inventario(&mut self) {
        if self.obj_inventory.is_empty() {
            println!("\n - Inventario está vazio!\n");
        } else {
            println!();
            for i in &self.obj_inventory {
                println!("- {}", i.nome);
            }
            println!();
        }
    }
}

impl Clone for Inventario {
    fn clone(&self) -> Self {
        Inventario {
            obj_inventory: self.obj_inventory.clone()
        }
    }
}
