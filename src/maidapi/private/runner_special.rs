//Responsavel por rodar programas a partir da maid, porém com control characters e outras coisas que diferenciam da execução normal
//Basicamente, é uma das partes mais criticas

#[path = "runner.rs"] mod maid_runner;

pub fn run_uppersand(command: &mut String) { //Roda comandos com && (operador and)

    let splited_command = command.split("&&");
    let splited_command = splited_command.collect::<Vec<&str>>(); //Coletando as 2 partes do iterator gerado no comando anterior em um vector


    for i in splited_command {
        
        let mut i = i.trim().to_owned();

        println!("{}", i);

        if i.is_empty() {
            ()
        } else {

            let process = maid_runner::run(&mut i);

            if process == true && splited_command.is_empty() == false {
                continue
            } else {
                break
            }

        }

    }

}