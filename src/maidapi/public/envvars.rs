//Estas funções servem para interagir com o sistema de enviroment variables da maid
//Note que, caso esteja usando outro sistema, estas funções podem não funcionar

pub fn get_entry(entry: String) -> String {
    use std::io::Read;
    use std::path::Path;
    use std::env;
    use std::fs;

    let current_user = get_current_user();

    let user_format = format!("./Maid/Users/{current_user}/.vars/{entry}");
    let system_format = format!("./Maid/MaidFiles/Info/Vars/{entry}");



    if Path::new(&system_format).exists() == true {

        let mut content = String::new();

        let mut selected_file = fs::File::open(system_format).unwrap();

        selected_file.read_to_string(&mut content).expect("Um erro ocorreu enquanto o arquivo 1 era lido");

        return content.trim().to_owned();

    } 
    
    else if Path::new(&user_format).exists() == true {

        let mut content = String::new();

        let mut selected_file = fs::File::open(user_format).unwrap();

        selected_file.read_to_string(&mut content).expect("Um erro ocorreu enquanto o arquivo 2 era lido");

        return content.trim().to_owned();

    }

    else if env::var(&entry).is_ok() {

        return env::var(&entry).unwrap()

    }

    else {

        return "Not found".to_owned();

    }

}

pub fn create_entry(entry: &String, env: &String) {

    //Há 4 valores possíveis para o env
    //User - Cria uma var na home do usuário
    //Maid - Cria uma var global da maid
    //Os_user - Cria uma var para o usuário logado no sistema operacional
    //Os_system - Cria uma var global no sistema operacional

    

}

fn get_current_user() -> String {

    //O usuário logado tem uma prioridade maior do que o usuário padrão.
    //ou seja, se um usuário está definido como o padrão, porém outro usuário está logado
    //o usuário logado será definido como o usuário atual
    
    use std::path::Path;
    use std::fs;
    use std::io::Read;

    if Path::new("./Maid/MaidFiles/Info/Users/current").exists() {  
        let mut selected_file = fs::File::open("./Maid/MaidFiles/Info/Users/current").unwrap();

        let mut content = String::new();
        selected_file.read_to_string(&mut content).expect("Um erro ocorreu enquanto o arquivo era lido");
        return content.trim().to_owned();
    } 

    else if Path::new("./Maid/MaidFiles/Info/Users/default").exists() {
        let mut selected_file = fs::File::open("./Maid/MaidFiles/Info/Users/default").unwrap();

        let mut content = String::new();
        selected_file.read_to_string(&mut content).expect("Um erro ocorreu enquanto o arquivo era lido");
        return content.trim().to_owned();
    }

    else {
        let return_value = String::from("None");
        return return_value;
    };
}





/*
pub fn get_entry(entry: &String, mode: &String) {

    let mut return_value = String::new();

    if mode == "system" {

        maid

    } 
    
    else if mode == "user" {


    } 
    
    else if mode == "default" {

    }


}
*/