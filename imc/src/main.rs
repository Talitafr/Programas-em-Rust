use std::io;

fn main() {
    let mut peso = String::new();
    let mut altura = String::new(); 
    println!("Calculadora IMC");
    println!("Digite o peso(Kg):");
    io::stdin().read_line(&mut peso).expect("Failed to read line");
    let peso:f32 =  peso.trim().parse().expect("Dado Inválido");
    println!("Digite a altura(m) Exemplo: 1.40:");
    io::stdin().read_line(&mut altura).expect("Failed to read line");
    let altura:f32 =  altura.trim().parse().expect("Dado Inválido");

    calcular_imc(peso,altura);
}
fn calcular_imc(peso:f32,altura:f32){
    
    let imc:f32 =  peso/((altura*altura));
    println!(" Seu IMC: {}",imc);

    if imc>=40.0{
        println!("Obesidade III");
    }
    else if imc>=35.0 && imc<=39.9{
        println!("Obesidade II");
    }
    else if imc>= 30.0 && imc<=34.9{
        println!("Obesidade I");
    }
    else if imc>=25.0 && imc<=29.9{
        println!("Sobrepeso");
    }
    else if imc>=18.6 && imc<=24.9{
        println!("Normal");
    }
    else{
        println!("Abaixo do Normal");
    }
}
