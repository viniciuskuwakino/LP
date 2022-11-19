use std::fs::File;
use std::io;
// use std::path::Path;
use std::io::{stdin, stdout, Read, Write};

mod cena;
mod objeto;
mod inventario;

use cena::Cena;
use objeto::Objeto;
use inventario::Inventario;
use std::cmp::max;
use std::borrow::Borrow;
use std::f32::consts::E;

fn load_game() -> Vec<Cena> {
    let mut v_cenas = Vec::new();
    let mut file = File::open("dados.txt").expect("Algo deu errado!");
    let mut content = String::new();

    // Carrega o arquivo na variável content
    file.read_to_string(&mut content).expect("Erro");

    // v contém vetor de cenas<vetor de objetos<string>>
    let v: Vec<Vec<Vec<&str>>> = content
        .split(";END ROOM")
        .map(|x| x.trim().lines().map(|y| y.split('=').collect()).collect())
        .collect();

    // Carrega as cenas e os objetos da cena, e retorna o vetor de cenas
    for i in 0..v.len() - 1 {
        let mut cena = Cena::new(
            String::from(v[i][0][1]),
            String::from(v[i][1][1]),
            String::from(v[i][2][1]),
        );

        let mut j = 3;

        while j < v[i].len() {
            let obj = Objeto::new(
                v[i][j][1].to_string(),
                v[i][j + 1][1].to_string(),
                v[i][j + 2][1].to_string(),
                v[i][j + 3][1].to_string(),
                v[i][j + 4][1].to_string(),
                v[i][j + 5][1].to_string(),
                v[i][j + 6][1].to_string(),
                v[i][j + 7][1].to_string(),
            );
            cena.append_obj(obj);
            j = j + 8;
        }
        v_cenas.push(cena);
    }
    v_cenas
}

fn comando_use(
    v_cenas: &[Cena],
    mut cena_atual: usize,
    comando: Vec<&str>,
    input_uppercase: String,
    mut inv: Inventario
) -> usize {

    // Verifica se o objeto existe na cena
    let mut verifica_objeto = 0;

    // Clona os objetos da cena atual para a variável "cena", pois apenas uma variável pode possuir o dado
    let objeto_cena_atual = v_cenas[cena_atual].objetos.clone();

    // Se o tamanho do comando for igual a 2
    if comando.len() == 2 {

        // Percorre o vetor de objetos da cena atual
        for i in 0..objeto_cena_atual.len() {

            // Se o comando[1] for igual ao nome do objeto, então cena_atual recebe a cena_alvo
            if comando[1] == objeto_cena_atual[i].nome {
                cena_atual = nova_cena(cena_atual, input_uppercase.clone(), &objeto_cena_atual, i);
                verifica_objeto = 1;  // Objeto existe na cena
            }
        }

        if verifica_objeto == 0 {
            println!("\n - Objeto não existe na cena! \n");
        }
    } else if comando.len() == 4 {

        // Confere se o objeto estiver no inventário
        if inv.conferir(comando[1].to_string()) {

            // Percorre o vetor de objetos da cena atual
            for i in 0..objeto_cena_atual.len() {

                // Se o comando[3] for igual ao nome do objeto, então cena_atual recebe a cena_alvo
                if comando[3] == objeto_cena_atual[i].nome {
                    cena_atual = nova_cena(cena_atual, input_uppercase.clone(), &objeto_cena_atual, i);
                    verifica_objeto = 1;  // Objeto existe na cena
                }
            }
            if verifica_objeto == 0 {
                println!("\n - Objeto não existe na cena! \n");
            }

        } else {
            println!("\n - Objeto não está no inventário.\n")
        }

    } else {
        println!("\n - Comando incompleto\n");
    }

    cena_atual
}

fn nova_cena(
    mut cena_atual: usize,
    input_uppercase: String,
    cena: &Vec<Objeto>,
    i: usize
) -> usize {

    // Se o input_uppercase for igual ao comando do objeto da cena atual
    if input_uppercase == cena[i].comando {

        // Imprime o obj_positivo
        println!("\n- {}", cena[i].positivo);

        // cena_atual recebe a cena_alvo, parse e unwrap para converter string para inteiro
        if cena[i].cena_alvo != "-1" {
            cena_atual = cena[i].cena_alvo.parse().unwrap();
        }

        // Press Enter e limpa o terminal
        pause();
        clear_shell();
    } else {
        println!("\n- {}\n", cena[i].negativo)
    }

    cena_atual
}

fn comando_check(
    v_cenas: &[Cena],
    mut cena_atual: usize,
    comando: Vec<&str>,
) {

    // Verifica se o objeto existe na cena
    let mut verifica_objeto = 0;

    // Clona os objetos da cena atual para a variável "cena", pois apenas uma variável pode possuir o dado
    let objeto_cena_atual = v_cenas[cena_atual].objetos.clone();

    // Se o tamanho do comando for igual a 2
    if comando.len() == 2 {

        // Percorre o vetor de objetos
        for i in 0..objeto_cena_atual.len() {

            // Se o comando[1] for igual ao nome do objeto
            if comando[1] == objeto_cena_atual[i].nome {
                println!("\n - {}\n", objeto_cena_atual[i].texto);
                verifica_objeto = 1;  // Objeto existe na cena
            }
        }

        if verifica_objeto == 0 {
            println!("\n - Objeto não existe na cena! \n");
        }

    } else {
        println!("\n - Comando incompleto\n");
    }
}

fn comando_get(
    v_cenas: &[Cena],
    cena_atual: usize,
    comando: Vec<&str>,
    mut inv: Inventario
) -> Objeto {

    // Gera um objeto vazio
    let mut obj = Objeto::objeto_vazio();

    // Clona os objetos da cena atual para a variável "cena", pois apenas uma variável pode possuir o dado
    let objeto_cena_atual = v_cenas[cena_atual].objetos.clone();

    // Se o tamanho do comando for igual a 2
    if comando.len() == 2 {

        // Percorre o vetor de objetos
        for i in 0..objeto_cena_atual.len() {

            // Se o comando[1] for igual ao nome do objeto
            if comando[1] == objeto_cena_atual[i].nome {

                // Se o tipo do objeto for do tipo "INVENTORY_OBJECT"
                if objeto_cena_atual[i].tipo.eq("INVENTORY_OBJECT") {

                    // Confere se o objeto já está no inventário
                    if inv.conferir(objeto_cena_atual[i].clone().nome) {
                        println!("\n - Objeto já obtido!\n");
                    } else {
                        println!("\n - Objeto: {:?}, obtido com sucesso! \n", objeto_cena_atual[i].nome);
                        obj = objeto_cena_atual[i].clone();
                    }

                } else {
                    println!("\n - Nao posso pegar este objeto\n");
                }
            }
        }

        obj
    } else {
        println!("\n - Comando incompleto\n");
        obj
    }
}

fn comando_map(winner: i32) {
    println!("  ┌─────┐               ┌────────┐ ┌──────────┐
  │Cofre│               │Banheiro│ │Escritório│
  └──┬──┘               └───┬────┘ └─────┬────┘
     │                      │            │
  ┌──┴──┐      ┌────────┐   │ ┌──────┐   │   ┌──────┐    ┌─────┐
  │Porão│      │Passagem│   └─┤ Sala ├───┘   │Jardim├────┤Tunel│
  └──┬──┘      └───┬────┘     └───┬──┘       └───┬──┘    └─────┘
     │             │              │              │
 ┌───┴───┐     ┌───┴───┐          │          ┌───┴──┐    ┌─────────┐
 │Garagem│     │Armazem│          │          │Quarto│    │Quarto de│
 └───┬───┘     └───┬───┘          │          └───┬──┘    │Hospedes │
     │             │         ┌────┴────┐         │       └────┬────┘
┌────┴─────┐   ┌───┴───┐     │ A Casa  │    ┌────┴───┐        │
│Lavanderia├───┤Cozinha├─────┤ Monstro ├────┤Corredor├────────┘
└──────────┘   └───────┘     └─────────┘    └────┬───┘
                                                 │        ┌───────┐
                                                 │        │Sala de│
                                                 └────────┤ Jogos │
                                                          └───────┘");
    println!("Corações coletados: {}/4", winner);
    pause();
    clear_shell()
}

fn comando_help() {
    println!("Comandos do jogo:");
    println!("\t- inventory -> mostra o inventario.");
    println!("\t- use OBJETO -> interagir com objeto da cena (abrir, usar, pressionar, etc).");
    println!("\t- use ITEM with OBJETO -> usar item do inventario em objeto da cena.");
    println!("\t- check OBJETO -> descreve objeto da cena.");
    println!("\t- get OBJETO -> obtem objeto e guarda no inventario.");
    println!("\t- newgame -> aparece as opções de \"s\" ou \"n\" para reiniciar o jogo.");
    println!("\t- save NOME_SAVE -> salva o estado atual do jogo e os itens do inventário.");
    println!("\t- load NOME_SAVE -> carrega a cena e os objetos do inventário.");
    println!("\t- map -> mostra o mapa da casa monstro e o número de corações coletados.");
    println!("\t- exit -> sai do jogo.\n");
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"\nPress Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read_line(&mut String::new()).unwrap();
}

fn clear_shell() {
    print!("\x1B[2J\x1B[1;1H");
}

fn win_condition(
    mut inventory: Inventario
) -> i32 {

    let mut contador = 0;

    if inventory.conferir(String::from("CORACAO_AZUL")) {
        contador = contador + 1;
    }

    if inventory.conferir(String::from("CORACAO_VERMELHO")) {
        contador = contador + 1;
    }

    if inventory.conferir(String::from("CORACAO_VERDE")) {
        contador = contador + 1;
    }

    if inventory.conferir(String::from("CORACAO_AMARELO")) {
        contador = contador + 1;
    }

    contador
}

fn main() {

    let mut condicao_saida = 0;
    let mut cena_atual = 0;
    let mut v_cenas = load_game();
    let mut inventario = Inventario::new();
    let mut input = String::new();

    clear_shell();

    loop {

        // Condicao de vitoria do jogador
        let mut winner = 0;
        let inventory = inventario.clone();
        winner = win_condition(inventory);
        if winner == 4 {
            // v_cenas.len() retorna o numero de cenas, logo temos a ultima cena
            v_cenas[v_cenas.len()-1].print_info();
            break;
        }

        // Imprime o texto da cena
        v_cenas[cena_atual].print_info();
        println!("*** Use o comando \"help\" para as instruções...\n");

        io::stdin().read_line(&mut input).expect("Erro");

        // input em maiúsculo
        let input_uppercase = input.trim().to_uppercase();

        // input em maiúsculo para o split do comando
        let input_uppercase_command = input.trim().to_uppercase();

        let comando: Vec<&str> = input_uppercase_command.split(" ").collect();

        match comando[0] {
            "USE" => {
                let inv = inventario.clone();

                // cena_atual -> recebe valor da cena_alvo
                cena_atual = comando_use(&v_cenas, cena_atual, comando, input_uppercase, inv);
            }
            "GET" => {
                let inv = inventario.clone();
                let obj = comando_get(&v_cenas, cena_atual, comando, inv);

                if !obj.nome.eq("") {
                    inventario.obter_objeto(obj);
                } else {
                    println!("\n - Objeto não existe na cena! \n");
                }

            }
            "INVENTORY" => inventario.mostrar_inventario(),
            "CHECK" => comando_check(&v_cenas, cena_atual, comando),
            "HELP" => comando_help(),
            "NEWGAME" => {
                let mut opcao = String::new();
                println!("\n - Deseja reiniciar o jogo? (S/N)");
                io::stdin().read_line(&mut opcao).expect("Erro");

                if opcao.trim().eq("s") || opcao.trim().eq("S") {
                    condicao_saida = 0;
                    cena_atual = 0;
                    v_cenas = load_game();
                    inventario = Inventario::new();
                    input = String::new();

                    println!("\n - O jogo será reiniciado!\n");

                } else if opcao.trim().eq("n") || opcao.trim().eq("N") {
                    println!("\n - O jogo continua...\n");
                } else {
                    println!("\n - Esta opção não existe!\n");
                }

                pause();
                clear_shell();
            }
            "SAVE" => {

                if comando.len() == 2 {
                    let arquivo = comando[1].to_string() + ".txt";
                    let mut save = File::create(arquivo);
                    let mut texto = String::new();
                    let inv = inventario.clone();

                    let var = vec![
                        "obj_id=".to_string(),
                        "obj_tipo=".to_string(),
                        "obj_nome=".to_string(),
                        "obj_texto=".to_string(),
                        "obj_positivo=".to_string(),
                        "obj_negativo=".to_string(),
                        "obj_comando=".to_string(),
                        "obj_cena_alvo=".to_string(),
                    ];

                    // Salva a cena atual do jogo
                    let mut save_cena = String::from("cena_id=");
                    save_cena.push_str(cena_atual.to_string().as_ref());
                    texto.push_str(save_cena.as_ref());
                    texto.push_str("\n");

                    // Salva os objetos que estão no inventário
                    for i in inv.obj_inventory {
                        let mut save_obj = String::from("");

                        for j in var.clone() {
                            save_obj = j;

                            if save_obj.eq("obj_id=") {
                                save_obj.push_str(i.id.to_string().as_ref());
                                texto.push_str(save_obj.as_ref());
                                texto.push_str("\n");

                            } else if save_obj.eq("obj_tipo=") {
                                save_obj.push_str(i.tipo.to_string().as_ref());
                                texto.push_str(save_obj.as_ref());
                                texto.push_str("\n");

                            } else if save_obj.eq("obj_nome=") {
                                save_obj.push_str(i.nome.to_string().as_ref());
                                texto.push_str(save_obj.as_ref());
                                texto.push_str("\n");

                            } else if save_obj.eq("obj_texto=") {
                                save_obj.push_str(i.texto.to_string().as_ref());
                                texto.push_str(save_obj.as_ref());
                                texto.push_str("\n");

                            } else if save_obj.eq("obj_positivo=") {
                                save_obj.push_str(i.positivo.to_string().as_ref());
                                texto.push_str(save_obj.as_ref());
                                texto.push_str("\n");

                            } else if save_obj.eq("obj_negativo=") {
                                save_obj.push_str(i.negativo.to_string().as_ref());
                                texto.push_str(save_obj.as_ref());
                                texto.push_str("\n");

                            } else if save_obj.eq("obj_comando=") {
                                save_obj.push_str(i.comando.to_string().as_ref());
                                texto.push_str(save_obj.as_ref());
                                texto.push_str("\n");

                            } else if save_obj.eq("obj_cena_alvo=") {
                                save_obj.push_str(i.cena_alvo.to_string().as_ref());
                                texto.push_str(save_obj.as_ref());
                                texto.push_str("\n");

                            } else {
                                println!("Erro");
                            }
                        }
                    }

                    save.unwrap().write(texto.as_ref());

                    println!("\n - Jogo salvo com sucesso no arquivo:\"{}.txt\" \n", comando[1]);

                } else {
                    println!("\n - Comando save incorreto.\n");
                }

            }
            "LOAD" => {

                if comando.len() == 2 {

                    // inventario está vazio
                    inventario = Inventario::new();

                    // variável load terá o nome do "arquivo.txt", para carregar a cena e os objetos do inventário
                    let mut load = comando[1].to_string();
                    load.push_str(".txt".to_string().as_ref());

                    // Carrega o "arquivo.txt"
                    let mut file = File::open(load).expect("\n\n - O arquivo não existe!\n\n");

                    // Cria variavel content para receber os dados do arquivo
                    let mut content = String::new();

                    // Carrega o arquivo na variável content
                    file.read_to_string(&mut content).expect("Erro");

                    // v contém vetor de cenas<vetor de objetos<string>>
                    let v: Vec<Vec<&str>> = content
                        .split('\n').map(|x: &str| x.split('=').collect())
                        .collect();

                    cena_atual = v[0][1].parse().unwrap();

                    println!("CENA ATUAL:{}",cena_atual);

                    let mut k = 1;

                    while k < v.len() - 1 {
                        let load_obj = Objeto::new(
                            v[k][1].to_string(),
                            v[k + 1][1].to_string(),
                            v[k + 2][1].to_string(),
                            v[k + 3][1].to_string(),
                            v[k + 4][1].to_string(),
                            v[k + 5][1].to_string(),
                            v[k + 6][1].to_string(),
                            v[k + 7][1].to_string(),
                        );
                        inventario.obter_objeto(load_obj);
                        k = k + 8;
                    }

                    clear_shell();

                } else {
                    println!("\n - Comando load incorreto.\n");
                }

            }
            "MAP" => comando_map(winner),
            "EXIT" => {
                println!("Saindo do jogo...\n");
                condicao_saida = 1;
            }
            _ => println!("Nao existe o comando: {}\n", input.trim()),
        }

        if condicao_saida == 1 {
            break;
        }

        input = String::from("");
    }
}

