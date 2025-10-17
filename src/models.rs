use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
#[derive(Debug, Serialize, Deserialize)]
pub struct Tarefa {
    pub titulo: String,
    pub descricao: String,
    pub categoria: String,
    pub data: NaiveDate,
    pub prioridade: Prioridade,
    pub status: Status
}
#[derive(Debug, Serialize, Deserialize)]
pub enum Prioridade {
    Alta,
    Media,
    Baixa
}
#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Pendente,
    Concluida { date: NaiveDate }
}

impl Tarefa {
    pub fn new(
        titulo: String,
        descricao: String,
        categoria: String,
        data: NaiveDate,
        prioridade: Prioridade) -> Self {
        Self {
            titulo,
            descricao,
            categoria,
            data,
            prioridade,
            status: Status::Pendente
        }
    }
    // pub fn exibir (&self) -> String {

    //     let icone_status = match self.status {
    //         Status::Pendente => "[⏳]",
    //         Status::Concluida { .. } => "[✅]",
    //     };

    //     format!("{} {} - Data Prevista: {}", icone_status, self.titulo, self.data.format("%d-%m-%Y"))
    // }
    //Vamos substituir o exibir por uma implementacao do fmt
    pub fn finalizar(&mut self){
        self.status = Status::Concluida { date: Utc::now().date_naive() }
    }
}
impl fmt::Display for Tarefa {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let icone_status = match self.status {
            Status::Pendente => "[⏳]",
            Status::Concluida { .. } => "[✅]",
        };
        write!(
            f, 
            "{}{} (Data Prevista: {}", 
            icone_status, 
            self.titulo, 
            self.data.format("%d-%m-%Y")
        )
    }
}