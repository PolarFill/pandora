//Responsavel por rodar programas a partir da maid
//Basicament&e, é uma das partes mais criticas

use shell_words::split;
use which::which;

#[path = "builtins.rs"] mod maid_bultins; //Builtins da maid

pub fn run(command: &mut String) -> bool { //Roda comandos e 
    use std::process::{Command, Stdio, exit};

    if command.contains(" ") == false { //Checa se o comando tem um espaço (ou seja, se é composto de "binario + argumentos" ou só "binario")
        command.push(' ');              //Caso seja só binario, insere um "", para poder dividir o comando em 2 a partir do espaço e coletar tudo em um vec
    }

    println!("{}", &command);

    let user_command = command.splitn(2, ' ');  //Dividindo em duas partes: o módulo que deve ser executado, e os argumentos para o binario
    let user_command = user_command.collect::<Vec<&str>>(); //Coletando as 2 partes do iterator gerado no comando anterior em um vector

    let module = user_command[0].trim(); //Removendo newlines da parte do módulo pra não zoar tudo

    let command_args = user_command.get(1).unwrap().trim();         //Pegando o elemento 1 do vector (os argumentos) e dando um trim nele só por precaução
    let command_args = shell_words::split(&command_args).unwrap();  //Passa isso pro shell_worlds, pra ele transformar em algo aceito pelo Command::new().args()

    let mut is_bultin = false;

    match module { //Match para checar se o módulo fornecido é um builtin da maid

        "exit" => exit(0),
        "restart" => { maid_bultins::restart(); is_bultin = true; },
        "clear" | "cls" => {print!("\x1B[2J\x1B[1;1H"); is_bultin = true},
        "echo" => {println!("{}", user_command[1]); is_bultin = true},
        _ => {}
    }

    if is_bultin == false {

        let module_path = check_path(&module.to_string());  //Pega o diretório do executavel a partir do path
        if module_path == "Not found" {                     //Checa se o check_path retornou "Not found" (ou seja, o executavel não existe)
            println!("{}: Comando ou arquivo inválido", module);             //Avisa que o comando não foi encontrado
            return false;                                                    //Retorna false (ou seja, retorna que o processo não foi finalizado corretamente)
        } else {                                            //Caso o comando tenha sido encontrado

            let mut child = Command::new(&module_path)      //Inicia o binario em um novo processo a partir do path dele
                            .args(command_args)             //Fornece argumentos pro processo, caso aja algum
                            .stdout(Stdio::inherit())       //redireciona o stdout e o stdin do programa para o console sendo executado
                            .stdin(Stdio::inherit())
                            .spawn()                        //Spawna o processo
                            .unwrap();
            
            let _result = child.wait().unwrap();    //Espera o processo ser finalizado
            return _result.success();                           //Retorna se o processo foi finalizado com sucesso ou não
        }
    } else { 
        return true //Se o comando for um bultin, só assume que ele foi finalizado corretamente e retorna true
    }
}

pub fn run_nonmodule(module: &mut String) {
    use std::path::Path;
    use std::process::{Command, Stdio};
    use is_executable::IsExecutable;

    let module = module.trim(); //dando trim no comando
    let module = module.replacen("\"", "",1); //Remove as aspas do começo

    let splited_module = module.splitn(2, "\"");    //Divide o comando fornecido em 2 partes a partir das aspas (")
    let splited_module = splited_module.collect::<Vec<&str>>(); //Coletando as 2 partes do iterator gerado no comando anterior em um vector

    let module = splited_module[0].replace("\"", "");  //Remove as aspas do módulo, pois elas não são necessárias

    if module.contains("/") || module.contains("\\") || module.contains("\\\\") { //Checa se o módulo é um caminho especificando um binario
        if Path::new(&module).exists() && Path::new(&module).is_executable() {      //Se o módulo for um caminho, e esse caminho existir, e se o módulo for um executavel
            if splited_module[1].is_empty() {                                     //Se o comando do usuario não tiver argumentos
                                                                                    //Executa o processo sem argumentos
                let child = Command::new(&module)      //Inicia o binario em um novo processo a partir do path dele
                .stdout(Stdio::inherit())       //redireciona o stdout e o stdin do programa para o console sendo executado
                .stdin(Stdio::inherit())
                .spawn()                        //Spawna o processo
                .unwrap();

                let _result = child.wait_with_output().unwrap(); //Espera o processo encerrar para continuar com a execução normal da maid

            } else {                                                        //Caso o contrario (comando fornecido com argumentos)

                let command_args = shell_words::split(&splited_module[1]).unwrap();  //Passa o argumento pro shell_worlds, pra ele transformar em algo aceito pelo Command::new().args()

                let child = Command::new(&module)      //Inicia o binario em um novo processo a partir do path dele
                .args(command_args)             //Fornece argumentos pro processo, caso aja algum
                .stdout(Stdio::inherit())       //redireciona o stdout e o stdin do programa para o console sendo executado
                .stdin(Stdio::inherit())
                .spawn()                        //Spawna o processo
                .unwrap();

                let _result = child.wait_with_output().unwrap(); //Espera o processo encerrar para continuar com a execução normal da maid

            }

        } else {
            println!("{}: Comando ou arquivo inválido", module);
        }

    } else {
        println!("{}: Comando ou arquivo inválido", module);
    }
}

//##################################################################
//Aqui se encontram as funções para checar o path
//Talvez seja uma boa idéia transformar isso em uma api '-

fn check_path(module: &String) -> String { //Checa se o comando está em algum diretório do path, e algumas outras coisas
    use std::path::Path;
    use is_executable::IsExecutable;
    
    let mut binary_directory = String::from("Not found"); //Cria uma variavel que irá armazenar o caminho final do binario (iniciada com o valor de não encontrado)

    if module.contains("/") || module.contains("\\") || module.contains("\\\\") { //Checa se o módulo é um caminho especificando um binario
        if Path::new(module).exists() && Path::new(module).is_executable() {      //Se o módulo for um caminho, e esse caminho existir, e se o módulo for um executavel
                binary_directory = module.to_string()                             //Define o caminho do binario como o módulo fornecido
            } 

    } else {  //Caso o módulo não seja um caminho

        let path = get_path();  //Pegando o path inserido pelo usuario com o get_path()

        for i in path {                                                 //Para cada diretório inserido no path
            let search_path1 = format!("{i}/{module}/{module}.exe");    //Caminho onde o binario é contido em uma pasta (path1)
            let search_path2 = format!("{i}/{module}.exe");             //Caminho onde o binario está jogado no meio da pasta (path2)

            if Path::new(&search_path1).exists()  {                     //Se o path1 existir
                binary_directory = search_path1.to_string();            //Armazena isso como o caminho final do binario
                break;                                                  //Quebra o for loop
            } else if Path::new(&search_path2).exists() {               //Mesma coisa do anterior porém com o path2
                binary_directory = search_path2.to_string();
                break;
            }
        }
    }

    if binary_directory == "Not found" {    //Caso o binario não tenha sido encontrado

        let result = which(module);                               //Usa o which para encontrar o caminho do binário, como uma ultíma solução

        match result {                                                                                       
            Ok(result) => binary_directory = result.into_os_string().into_string().unwrap(),  //Caso o which tenha encontrado o caminho do binário especificado, define ele como o valor de retorno da função
            _ => ()                                                                                    //Caso o contrário, não faz nada (comando não encontrado)
        }
    }

    return binary_directory;    //Retorna o diretório final do binario
}


fn get_path() -> Vec<String> { //Pega o path do arquivo onde ele está definido
    use std::io::Read;
    use std::fs;

//	if use_path == false {
//		return vec!["./Maid/MaidFiles/Modules".to_string()]
//	} else {

    let mut selected_file = fs::File::open("./Maid/MaidFiles/Info/Vars/path").unwrap();   //Abrindo arquivo do path

    let mut content = String::new(); //Variavel onde o path será armazenado
    selected_file.read_to_string(&mut content).expect("Um erro ocorreu enquanto o arquivo era lido"); //Lê o arquivo do path e joga o conteudo na string anterior
    
    let environment_path = content.trim().split(";").map(|s| s.to_string()); //Separa os diretórios por ;
    let environment_path = environment_path.collect::<Vec<String>>(); //Coleta tudo em um vector

    return environment_path; //Retorna o vector
//	};
}