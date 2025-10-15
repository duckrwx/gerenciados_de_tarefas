use std::io::Write;

pub fn read_string (prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();//Temos que limpar o buffer e jogar para fora o que tem dentro.
    let mut buffer = String::new();
    match std::io::stdin().read_line(&mut buffer){
        Ok(_) => buffer.trim().to_string(), //precisamos de uma variavel para retornar o read_line
        Err(_) => {
            println!("Erro de leitura");
            std::process::exit(1);
        }
    }
    //Metodo de leitura do teclado, que pode falhar! Por isso o match.
}