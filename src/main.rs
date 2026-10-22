use std::io::{self, Write};
mod ip;



fn main () {
    loop{
        print! ("zerosh> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io:: stdin().read_line(&mut line).unwrap();

        let line = line.trim();
        let parts: Vec<&str>  = line.split(' ').collect();
        let program:&str = parts[0];
        let args = &parts[1..];

        if program == "ip"  {
            match ip::ip(args) {
                Ok(_) => {
                    println!("Succes");
                }
                Err(e) => {
                    eprintln!("Error!!!: {}", e);
                }
            }
        } else if line.trim() == "exit"{
            break;
        } else {
            println!("コマンド部：{:?}", program);
            println!("引数部：{:?}", args);
        }
    }
}
