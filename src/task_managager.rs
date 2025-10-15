use crate::models::{ Tarefa, Prioridade};
use crate::io_utils::read_string;
use chrono::NaiveDate;
pub struct TaskManager {
    pub tarefas: Vec<Tarefa>,
}
impl TaskManager {
    pub fn new() -> Self {
        Self {
            tarefas: Vec::new(),
        }
    }
    // serde_json, std::fs, std::path
    pub fn _ler_tarefas() -> Vec<Tarefa> {
        todo!()
    }
    pub fn adicionar_tarefa (&mut self){
        let titulo = read_string("Titulo: ");
        let descricao = read_string("Descriçao: ");
        let categoria = read_string("Categoria: ");
        let pri_str = read_string("Prioridade (Baixa, Média, Alta): ");
        let prioridade = match pri_str.to_lowercase().as_str() {
            "baixa" => Prioridade::Baixa,                
            "média" => Prioridade::Media,
            "alta" => Prioridade::Alta,
            _ => { 
                Prioridade::Baixa
            }
        };
        let data_str = read_string("Vencimento: (DD-MM-AAAA): ");
        let data = match NaiveDate::parse_from_str(&data_str, "%d-%m-%Y") {
            Ok(dt) => dt,
            Err(_) => {
                println!("Data inválida, usando data atual.");
                chrono::Utc::now().date_naive()
                }
            };
        let tarefa = Tarefa::new(titulo, descricao, categoria, data, prioridade);
        println!("Tarefa {} criada.", tarefa.titulo);
    
        self.tarefas.push(tarefa);
        println!("Tarefa adicionada na Lista");
    }
    pub fn listar_tarefas(&self) {
        if self.tarefas.is_empty() {
            println!("Nenhuma tarefa encontrada.");
        } else {
            for (i, tarefa) in self.tarefas.iter().enumerate() {
                println!("{}. {}", i + 1, tarefa.exibir());
            }
        }
    }
    pub fn finalizar_tarefa(&mut self) {
        if  self.tarefas.is_empty() {
            println!("Nenhuma tarefa para finalizar.");
            return;
        }
        let titulo = read_string("Título da tarefa a ser finalizada: ");
        if let Some(tarefa) = self.tarefas.iter_mut().find(|t| t.titulo == titulo) {
            tarefa.finalizar();
            println!("Tarefa {} completada!", tarefa.titulo);
        } else {
            println!("Tarefa não encontrada.");
        }
    }
}


