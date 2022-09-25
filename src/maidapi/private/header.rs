pub fn print_header() { //Mostra o header na inicialização da maid
    #[path = "E:/programas/rust/maid-plus-renewed/src/maidapi/public/user.rs"] mod maid_user;
    #[path = "E:/programas/rust/maid-plus-renewed/src/maidapi/private/repair.rs"] mod maid_repair;
    use ini::Ini;
    use std::path::Path;

    let current_user = maid_user::get_current_user(); //Pega o usuario atual

    let conf_path = format!("./Maid/Users/{current_user}/Configurações/Customização.ini");

    if Path::new(&conf_path).exists() == false {    //Se o arquivo de configuração de titulo não existir
        maid_repair::try_fix(&current_user);                                                    //Chama o maid_repair para regerar ele
    }
    
    let conf = Ini::load_from_file(&conf_path).unwrap(); //Lendo o arquivo de configuração do titulo
    let section = conf.section(Some("Titulo")).unwrap();
    
    let print_title = section.get("exibir_titulo").unwrap_or("true");
    
    if print_title == "false" {
        ()
    }
    else {
        let title = section.get("titulo").unwrap_or("Maid");
        let use_figlet = section.get("fontes_alternativas").unwrap_or("true");

        if use_figlet == "false" {
            println!("{}", title)
        } else {
            print_figlet(&title)
        }
    }


}

fn print_figlet(title: &str) {
    use neofiglet::FIGfont;                             //Mostar título

    let figlet_font = FIGfont::standard().unwrap();
    let figlet = figlet_font.convert(&title);
    assert!(figlet.is_some());
    println!("{}", figlet.unwrap());
}