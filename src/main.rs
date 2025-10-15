use crate::task_managager::TaskManager;
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
        println!("4. Sair");

        let opcao = io_utils::read_string("Escolha uma opcao: ");


        match opcao.trim(){
            "1" => manager.adicionar_tarefa(),
            "2" => manager.listar_tarefas(),
            "3" => manager.finalizar_tarefa(),
            "4" => {
                println!("Saindo");
                break;
            }
            _ => println!("Opcao invalida."),
        }
    }
}

