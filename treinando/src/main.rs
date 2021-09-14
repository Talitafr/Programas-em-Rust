use std::io;
use rand::Rng;

fn main(){
    adivinhe();

}

fn adivinhe(){
    let numero_secreto = rand::thread_rng().gen_range(1, 101);
    
    loop{
        let mut numero = String::new();
        println!("\n Adivinhe o número:");
        println!("Digite o número:(se quiser sair do jogo digite um número negativo)");

        
        io::stdin().read_line(&mut numero).expect("Failed to read line");
        let numero:i32 = match numero.trim().parse(){
        //a instrução acima passa de String para inteiro i32 
        //Tratamento de erro
        Ok(num) => num,
        Err(_) => continue,
        };

        
        if numero > numero_secreto{
            println!("Muito Grande");
        }
        else if numero==numero_secreto{
                println!("Você Ganhou");
                break;
        }
        else if numero<0{
                println!("Você saiu do jogo, o número para adivinhar era {}",numero_secreto);
                break;
        }
        else{
            println!("Muito baixo");
            }
    }
}