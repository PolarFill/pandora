//Este arquivo contém o unico intuito de mudar o cwd para o diretório onde o binario da maid está localizado.
//O código está quebrado em várias funções para melhor entendimento

pub fn getbinpath() -> String {     //Pega o diretorio do binario da maid
    use std::env;

    let path = env::current_exe();
    match path {
        Ok(path) =>
                path.into_os_string().into_string().unwrap(),
        Err(_) =>
                "Operação falha".to_string()
    }
}

fn changecwd(new_path: &String) {  //Muda o cwd
    use std::env;

    assert!(env::set_current_dir(new_path).is_ok());
}

fn getbinname() -> String {  //Pega o nome do binario da maid
    use std::env;
    use std::path::Path;

    let bin_path = env::current_exe().unwrap();

    let bin_path = Path::new(&bin_path).file_name().unwrap();

    let bin_name = bin_path.to_string_lossy().into_owned();

    return bin_name
}

pub fn changecwd_binpath() {  //Muda o cwd pro diretório do binario da maid

    let mut path = getbinpath();            //Pegando diretório do binario da maid
    let bin_name = getbinname();            //Pegando o nome do binario da maid 
    path = path.replace(&bin_name, "");     //Removendo o bin do diretório
    changecwd(&path);                       //Mudando o cwd pro diretório da maid
}