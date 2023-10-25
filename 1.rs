/* *\
use std::default::Default;
mod utiles;

#[derive(Default)]
struct Libro{
  titulo:String,
  autor:String,
  año:i32
} //LIBRO ES UN TIPO DE DATO TMB CUANDO HACES LA ESTRUCTURA

//define tamaño del arreglo q vas a retornar
//LAS VARIABLES SON REFERENCIAS PARA COSAS Q HEMOS CREADO
//PUEDEN SER ACCEDIDOS POR MUCHAS VARIABLES, TENINENDO EL CONTROL DE QN FUE EL ULTIMO Q EDITO ESTA CAJITA, Y CON ELLA PODEMOS TENER ACCESO A LOS VALORES DE LA ESTRUCTURA
fn crear_arreglo_libro() -> [Libro;2] {
    let arreglo:[Libro;2] = [];

    return arreglo;
}

fn main(){
  let arreglo = crea_arreglo_libro();
  println!("{:?}",arreglo);
}
