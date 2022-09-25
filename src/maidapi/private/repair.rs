//Este arquivo contém o código para geração e reparo de arquivos da maid
//AVISO: DOCUMENTAR ESSE CÓDIGO DEPOIS

pub fn check() {  //Checa se essa é a primeira inicialização
    use std::path::Path;

    if Path::new("Maid").exists() == false ||                         //Checa se a pasta principal da maid existe
    Path::new("Maid/MaidFiles/Info/Init/boot").exists() == false {         //Se existir, checa se o arquivo de boot existe
        first_boot()                                                  //Caso algum dos dois dê falso, inicia o preparo inicial da maid
    }
}

pub fn try_fix(username: &String) {
    create_file_structure();
    conf_create(&username);
}

pub fn force_regen() {
    first_boot()
}

pub fn try_fix_startup() {
    #[path = "E:/programas/rust/maid-plus-renewed/src/maidapi/public/user.rs"] mod maid_user;
    use std::path::Path;
    use ini::Ini;

    let username = maid_user::get_current_user(); //Pega o usuario atual

    if Path::new("./Maid/Users/{username}/Configurações/Performance.ini").exists() { //Checa se o arquivo de performance existe

        let conf = Ini::load_from_file("./Maid/Users/{username}/Configurações/Performance.ini").unwrap();  //Se existir, carrega o arquivo
        let section = conf.section(Some("Geral")).unwrap();
        if section.get("checar_integridade").unwrap_or("true") == "true" { //Checa se a checagem de integridade de arquivos está ligada (fallback = true)
            try_fix(&username)                                //Se tiver, chama o maid_repair
        }
        
    } else {try_fix(&username)} //Se o arquivo de performance não existir, chama o maid_repair (pq tem algo de errado de qualquer forma)
}

//#####################################################################################################################


fn first_boot() {
    use std::io::{self, Write};
    use std::fs::File;

    println!("Parece que esta é a primeira vez que você executa a maid!");
    println!("Realizando preparações...");

    create_file_structure();

    println!("Parece que você não tem um usuário");

    print!("Insira o seu nome de usuário (você pode mudar isso depois): ");            //Exibindo ps1

    let _ = io::stdout().flush();   //Limpando o stdout, para não causar problemas com newlines
    let mut user_command = String::new();   //Comando do usuarío

    io::stdin().read_line(&mut user_command)  //Pegando input do usuario
    .ok()
    .expect("Um erro ocorreu enquanto o input era pego.");

    user_command = user_command.trim().to_string();

    println!("Criando usuário...");
    create_user(&user_command);
    user_default(&user_command);
    
    println!("Gerando configurações...");
    conf_create(&user_command);

    println!("Finalizando...");
    
    let _ = File::create("./Maid/MaidFiles/Info/Init/boot");

    println!("Processo concluido!");
    println!("Feche a maid e abra novamente");
}

//#####################################################################################################################

fn create_file_structure() {
    use std::path::Path;
    use std::fs::File;
    use std::fs;
    use std::io::{Write};

    if Path::new("./Maid").exists() == false {let _ = fs::create_dir("./Maid");};
    if Path::new("./Maid/Users").exists() == false {let _ = fs::create_dir("./Maid/Users");};
    if Path::new("./Maid/MaidFiles").exists() == false {let _ = fs::create_dir("./Maid/MaidFiles");};
    if Path::new("./Maid/MaidFiles/Modules").exists() == false {let _ = fs::create_dir("./Maid/MaidFiles/Modules");};
    if Path::new("./Maid/MaidFiles/Info").exists() == false {let _ = fs::create_dir("./Maid/MaidFiles/Info");};
    if Path::new("./Maid/MaidFiles/Info/Init").exists() == false {let _ = fs::create_dir("./Maid/MaidFiles/Info/Init");};
    if Path::new("./Maid/MaidFiles/Info/Vars").exists() == false {let _ = fs::create_dir("./Maid/MaidFiles/Info/Vars");};
    if Path::new("./Maid/MaidFiles/Info/Users").exists() == false {let _ = fs::create_dir("./Maid/MaidFiles/Info/Users");};

    if Path::new("./Maid/MaidFiles/Info/Init/initial_commands").exists() == false {let _ = File::create("./Maid/MaidFiles/Info/Init/initial_commands");};

    if Path::new("./Maid/MaidFiles/Info/Vars/path").exists() == false {
        let mut file = File::create("./Maid/MaidFiles/Info/Vars/path").unwrap(); 
        let _ = write!(&mut file, "./Maid/MaidFiles/Modules/CoreModules;./Maid/MaidFiles/Modules");
    };
}

fn conf_create(username: &String) {
    use std::fs::File;
    use std::io::{Write};
    use std::path::Path;

    let path1 = format!("./Maid/Users/{username}/Configurações/Performance.ini");
    if Path::new(&path1).exists() == false {
        let config_template = 
"#######################################################################
# Para ativar uma configuração, coloque o valor dela como \"true\"      #
# Para desativar uma configuração, coloque o valor dela como \"false\"  #
#######################################################################

[ Geral ]

# Define se a integridade dos arquivos deve ser checada sempre na inicialização
checar_integridade = true

# Ativa/desativa o sistema de abreviações
alias = true

[ Busca ]

# Ativa o path.
# O path define outros diretórios onde a Maid deve buscar módulos e scripts.
# Por padrão, a Maid só buscará módulos em \"Maid/MaidFiles/Modules\".
# (o path pode ser configurado em \"Maid/MaidFiles/Info/Path\")
utilizar_path = false

# Checa por módulos dentro e fora de pastas.
# Por padrão, a Maid só checa módulos dentro de pastas.
modulos_sem_pasta = false

# Checa por um módulo dentro de todas as pastas
# em um diretório.
checar_varias_pastas = false";
        let config_template = String::from(config_template);
        let mut file = File::create(&path1).unwrap();
        let _ = write!(&mut file, "{}", config_template);
    };

    let path2 = format!("./Maid/Users/{}/Configurações/Customização.ini", &username);
    if Path::new(&path2).exists() == false {
        let config_template = 
"#######################################################################
# Para ativar uma configuração, coloque o valor dela como \"true\"      #
# Para desativar uma configuração, coloque o valor dela como \"false\"  #
#######################################################################

[ Titulo ]

# Define se um titulo será exibido
exibir_titulo = true

# Define o que será escrito no titulo
titulo = Maid

# Define se o titulo será escrito com fontes alternativas
# Caso ativo, é possível especificar a fonte desejada
# (as fontes se encontram em \"Maid/MaidFiles/FigletFonts/\")
fontes_alternativas = true
fonte = default

[ Prefixos ]

# Define o prefixo para input
prefixo_input = Input >> ";
        let config_template = String::from(config_template);
        let mut file = File::create(&path2).unwrap();
        let _ = write!(&mut file, "{}", config_template);
    };
}

//#####################################################################################################################

fn create_user(username: &String) {

    //Essa função adiciona um usuário

    use std::path::Path;
    use std::fs;
    use std::io::{Write};
    use std::fs::File;

    let folder_path = format!("./Maid/Users/{}", username);
    let midia = format!("{}/Mídia", folder_path);
    let documentos = format!("{}/Documentos", folder_path);
    let downloads = format!("{}/Downloads", folder_path);
    let configs = format!("{}/Configurações", folder_path);
    let userfiles = format!("{}/.userfiles", folder_path);
    let uservars = format!("{}./.vars", folder_path);

    let user_filepath = format!("./Maid/MaidFiles/Info/Users/{}.info", username);

    if Path::new(&folder_path).exists() {
        println!("Um usuário com este nome já existe.");
    } else {
        println!("Criando usuário...");
        fs::create_dir(&folder_path).expect("Falha criando home");
        fs::create_dir(&midia).expect("Falha criando midia");
        fs::create_dir(&downloads).expect("Falha criando downloads");
        fs::create_dir(&configs).expect("Falha criando configs");
        fs::create_dir(&documentos).expect("Falha criando documentos");
        fs::create_dir(&userfiles).expect("Falha criando .userfiles");
        fs::create_dir(&uservars).expect("Falha criando .vars");

        let config_template = format!("
        [user]
        name = {}
        password = false
        protected = false

        [folder_structure]
        folder_path = {}
        midia = {}
        documentos = {}
        downloads = {}
        configs = {}
        userfiles = {}
        vars = {}
        ", &username, &folder_path, &midia, &documentos, &downloads, &configs, &userfiles, &uservars);

        let config_template = String::from(config_template);
        let mut file = File::create(user_filepath).unwrap();
        let _ = write!(&mut file, "{}", config_template);

        if Path::new("./Maid/MaidFiles/Info/Users/users").exists() == false {
            let _file = fs::File::create("./Maid/MaidFiles/Info/Users/users");
        };

        {let mut user_file = fs::OpenOptions::new()
                                            .read(false)
                                            .append(true)
                                            .open("./Maid/MaidFiles/Info/Users/users")
                                            .unwrap();

        let _ = writeln!(user_file, "{}", &username);}
        
        println!("Usuário {} criado com sucesso!", username);
    }
}

fn user_default(username: &String) {

    use std::io::{Write};
    use std::fs;

    let mut file = fs::File::create("./Maid/MaidFiles/Info/Users/default").unwrap();
    file.write(username.as_bytes()).expect("Um erro ocorreuu enquanto o arquivo era escrito: ");

    println!("Usuário {} definido como usuário padrão!", username);

}