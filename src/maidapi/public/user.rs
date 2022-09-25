//Estas funções servem para interagir com o sistema de usuários da maid
//Alguns exemplos de funções são: pegar usuário atual, checar se usuário existe, chegar diretório de alguma pasta, etc
//Note que essas funções são para o sistema padrão de usuários da maid, portanto, caso você estejá usando outro sistema
//estas funções talvez não funcionem

pub fn get_current_user() -> String {

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

pub fn get_user_folder(selected_folder: &str, mode: i8) -> String {
    
    //Esta função tem dois modos:
    //O primeiro modo (1) é mais leve, porém, tem menos acurácia, enquanto o segundo modo (2) é o oposto
    //O primeiro modo retorna o suposto diretório do usuário de acordo com padrões (exemplo: ./Maid/polar)
    //já o segundo modo lê o arquivo .info do usuário e retorna os diretórios presentes lá

    let username = get_current_user();

    if mode == 1 {
        let return_value =
        match selected_folder {
            "downloads" | "download" => format!("./Maid/Users/{}/Downloads", username),
            "documents" | "document" => format!("./Maid/Users/{}/Documentos", username),
            "medias" | "media"  => format!("./Maid/Users/{}/Mídia", username),
            "configs" | "config" => format!("./Maid/Users/{}/Configurações", username),
            _ => format!("./Maid/Users/{}", username)
        };

        return return_value;
    } else {
        String::from("None")
    }

    /*
    else {

        use configparser::ini::Ini;

        let mut config = Ini::new();

        let info_file = format!("./Maid/MaidFiles/Info/Users/{}.info", username);

        let map = config.load(info_file);

        let return_value =
        match selected_folder {
            "downloads" | "download" => config.get("folder_structure", "downloads").unwrap(),
            "documents" | "document" => config.get("folder_structure", "documentos").unwrap(),
            "medias" | "media"  => config.get("folder_structure", "midia").unwrap(),
            "configs" | "config" => config.get("folder_structure", "configs").unwrap(),
            _ => config.get("folder_structure", "folder_path").unwrap()
        };

        return return_value;

    }
    */
}