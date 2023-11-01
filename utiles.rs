use std::io;

pub fn texto_numero(campo:String) -> i32 {
    loop {
        println!("Ingrese un número para el/la {}: ",campo);
        let mut numero = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut numero).unwrap();
        let numero: i32 = match numero.trim().parse(){
            Ok(numero) => numero,
            Err(_) => {
                println!("Error, no es un número");
                continue;
            },
        };
        return numero;
    }
}

pub fn texto_numero_float() -> f32 {
    loop {
        println!("Ingrese un número: ");
        let mut numero = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut numero).unwrap();
        let numero: f32 = match numero.trim().parse(){
            Ok(numero) => numero,
            Err(_) => {
                println!("Error, no es un número");
                continue;
            },
        };
        if numero <= 7.0{
            println!("");
            return numero;
        } else {
            println!("ingrese un numero entre el 1 y el 7");
        }
    }
}



pub fn ingreso_texto(campo: String) -> String {

    println!("Ingrese {}", campo);
    let mut texto = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut texto).unwrap();
    return texto

}


pub fn si_no() -> bool {

    loop{
        println!("digite 1 para un SI, 0 para un NO\n");
        let desicion = texto_numero("desicion".to_string());
        if desicion == 1 {
            return true;
        }
        if desicion == 0{
            return false;
        } 
    }
}
