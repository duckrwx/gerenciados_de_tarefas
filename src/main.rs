use crate::{io_utils::read_string, task_managager::TaskManager};
mod models;
mod io_utils;
mod task_managager;

fn main(){
    let mut manager = TaskManager::new();
    loop{
        println!("======================");
        println!("Gerenciador de Tarefas");
        println!("======================");
        println!("1. Adicionar Tarefa");
        println!("2. Listar Tarefa");
        println!("3. Marcar Tarefa Finalizada");
        println!("4. Importar arquivo JSON");
        println!("5. Sair");

        let opcao = io_utils::read_string("Escolha uma opcao: ");


        match opcao.trim(){
            "1" => manager.adicionar_tarefa(),
            "2" => manager.listar_tarefas(),
            "3" => manager.finalizar_tarefa(),
            "4" => manager.ler_tarefas(),
            "5" => {
                if manager.tarefas.is_empty(){
                println!("Lista Vazia, Adeus!");
                break;
                } else {
                    let save = read_string("Salvar arquivo em JSON (Y,n)?: ");
                    match save.trim() {
                        "n" | "N" => {
                            println!("Arquivo nao foi salvo.")
                        },
                        _ => {
                            manager.salvar_tarefas();
                            println!("Arquivo salvo em {}", save);
                            break;
                        }
                    }
                }
            }
            _ => println!("Opcao invalida."),
        }
    }
}

