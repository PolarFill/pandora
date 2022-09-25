pub fn restart() {
    use std::process::{Command};
    #[path = "dir.rs"] mod maid_dir;

    let bin_path = maid_dir::getbinpath();
    
    print!("\x1B[2J\x1B[1;1H");

    Command::new("./Maid/MaidFiles/Modules/CoreModules/restart.exe")          //executa o executavel do módulo
    .arg(bin_path)
    .spawn()                           //spawna o processo, para assim pegar o output dele em "tempo real"
    .expect("Um erro ocorreu durante a execução deste módulo: ");

    std::process::exit(0)

}