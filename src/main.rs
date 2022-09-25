#[path = "maidapi/private/dir.rs"] mod maid_dir;
#[path = "maidapi/private/init_funcs.rs"] mod maid_initrc;
#[path = "maidapi/private/repair.rs"] mod maid_repair;
#[path = "maidapi/private/header.rs"] mod maid_header;
#[path = "maidapi/private/runner.rs"] mod maid_runner;
#[path = "maidapi/private/runner_special.rs"] mod maid_runner_special;
#[path = "maidapi/public/user.rs"] mod maid_user;
#[path = "maidapi/public/envvars.rs"] mod maid_vars;

use ini::Ini;
use rustyline::error::ReadlineError;
use rustyline::{Editor};

fn main() {

    maid_dir::changecwd_binpath(); //Muda o cwd atual pro diretório onde o binario da maid está localizado
    maid_repair::check(); //Checa se algo precisa de reparo ou se essa é a primeira inicialização
    maid_repair::try_fix_startup(); //Checa a integridade dos arquivos (ou seja, chama o try_fix), caso o usuario tenha ativado isso     
    maid_initrc::initrc(); //Executa comandos especificados pelo usuario na inicialização da maid
    maid_header::print_header(); //Exibe o header da maid (ou não)
    
    let ps1 = get_ps1();    //Pegando ps1 (prefixo que será exibido quando o input for pedido)
    let ps1 = format!("{} ", ps1); //Adicionando um espaço no final do ps1

    let current_user = maid_user::get_current_user();

    let mut rl = Editor::<()>::new();                                            //Criando editor do rustyline (objeto que será utilizado para outras funções do rustyline)

    let history_path = format!("./Maid/Users/{}/.userfiles/history.txt", current_user);
    if rl.load_history(&history_path).is_err() {                                            //Carregando histórico
        println!("Histórico não encontrado")
    }

    loop {

        let user_command = rl.readline(ps1.as_str());   //Pegando input com rustyline

        match user_command {                                        //Checando resultado do input
            Ok(mut user_command) => {                           //Resultado ok:

                let use_alias = check_alias_enabled();              //Checa se o alias está ativado
                // let interpret_vars = check_vars_enabled();

                if use_alias == true {                                    //Se o alias tiver ativado
                    user_command = parse_alias(&mut user_command);        //Checa o módulo por alias
                }

                // if interpret_vars == true {
                //     user_command = parse_vars(&mut user_command);
                // }

                if user_command.contains("&&") {

                    rl.add_history_entry(&user_command);                      //Adiciona o comando no histórico
                    maid_runner_special::run_uppersand(&mut user_command);

                }

                if user_command.starts_with("\"") { //Se o comando conter aspas no começo e fim (diretório com espaço)

                    rl.add_history_entry(&user_command);                      //Adiciona o comando no histórico
                    maid_runner::run_nonmodule(&mut user_command);          //Roda o comando como um diretório especificado para um binario
                                                                                //AVISO: BOTAR SUPORTE PARA ARGUMENTOS (por enquanto só o executavel especificado pode ser executado, sem argumentos)
                } else {                                                   //Caso o contrario

                    rl.add_history_entry(&user_command);              //Adiciona o comando no histórico
                    maid_runner::run(&mut user_command);                   //Roda o comando

                }
            },

            Err(ReadlineError::Interrupted) => {                       //Interrompido (ctrl + c)
                println!("CTRL-C");                                    //Printa ctrl + c e encerra o loop
                break                                                  
            },
            Err(ReadlineError::Eof) => {                               //End of file
                println!("CTRL-D");                                    //Printa ctrl + d e encerra o loop
                break
            },
            Err(err) => {                               //Outro erro
                println!("Erro: {:?}", err);                           //Printa o erro e encerra o loop
                break
            }
        }

        let _ = rl.save_history(&history_path);   //Salva o histórico

    }
}


fn get_ps1() -> String { //Pega o ps1 (separado em outra função pra main() ficar mais bonitinha)
    let current_user = maid_user::get_current_user();
    let customization_path = format!("./Maid/Users/{current_user}/Configurações/Customização.ini");
    
    let conf = Ini::load_from_file(customization_path).unwrap();
    let section = conf.section(Some("Prefixos")).unwrap();
    
    let ps1 = section.get("prefixo_input").unwrap_or("Input >>");

    return ps1.to_string();
}

//##################################################################
//Aqui se encontram as funções para checar alias
//Talvez seja uma boa idéia transformar isso em uma api '-


fn check_alias_enabled() -> bool {

    #[path = "E:/programas/rust/maid-plus-renewed/src/maidapi/public/user.rs"] mod maid_user;

    let current_user = maid_user::get_current_user();
    let conffile = format!("./Maid/Users/{current_user}/Configurações/Performance.ini");
    
    let conf = Ini::load_from_file(conffile).unwrap();
    let section = conf.section(Some("Geral")).unwrap();
    
    let alias = section.get("alias").unwrap_or("false");

    if alias == "false" {
        return false
    } else {
        return true
    }
}

fn parse_alias(command: &mut String) -> String {

    let alias_value = maid_vars::get_entry("alias".to_owned());

    let alias_list = alias_value.trim().split(";").map(|s| s.to_string()); //Separa os diretórios por ;
    let alias_list = alias_list.collect::<Vec<String>>(); //Coleta tudo em um vector

    let mut return_value = String::from(&*command);

    for i in alias_list {

        if i.starts_with(&*command) {

            let i = i.trim();

            let mut split_list = i.splitn(2, "=");

            let _ = split_list.next().unwrap();
            let alias_final = split_list.next().unwrap();

            return_value = alias_final.to_owned();
        }

    }

    return return_value.trim().to_owned();

}

//##################################################################
//Aqui se encontram as funções para checar vars

fn check_vars_enabled() -> bool {

    return true;
}

fn parse_vars(command: &mut String) -> String {

    return String::new();
}\