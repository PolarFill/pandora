//Responsavel por mostrar o cabeçalho que aparece quando o programa é iniciado
//além de executar certos comandos

pub fn initrc() { //Função responsavel por executar comandos durante o inicio da maid

    #[path = "runner.rs"] mod runner;
    use std::path::Path;
    use std::fs;
    use std::io::{self, BufRead};

    if Path::new("./Maid/MaidFiles/Info/Init/initial_commands").exists() {   //Checa se um arquivo de comandos iniciais existe 
                                                                                                                                
        let selected_file = fs::File::open("./Maid/MaidFiles/Info/Init/initial_commands").unwrap();  //Se existir, lê ele e transcreve as linhas em um iterator
        let lines = io::BufReader::new(selected_file).lines();
        
        for i in lines {
            let _ = runner::run(&mut i.unwrap());
        }

    }
}