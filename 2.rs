use std::default::Default;
mod utiles;
#[derive(Default)]
#[derive(Clone)]

struct Estudiantes{
    nombre:String,
    matricula: i32,
    notas:[f32;5]
}

fn create_array() -> [Estudiantes;4]{

    let arreglo:[Estudiantes;4] = Default::default();
    return arreglo;

}

fn read_array(estudiante: &[Estudiantes;4]) -> (){

    for (number, item) in estudiante.iter().enumerate() {
        print!("{}: Nombre: {} ", number+1, item.nombre);
        print!("  Matricula: {} \n", item.matricula);
        print!("    Notas {:?}\n", item.notas);
    }


}

fn edit_array(estudiante: [Estudiantes;4]) -> [Estudiantes;4]{
    let mut nuevo_estudiante :[Estudiantes;4]= estudiante.clone(); 
    let mut prom:f32 = 0.0;
    let mut new_num:f32 = 0.0;
    for i in 0..nuevo_estudiante.len() as usize{
        // reinicio de la variable
        prom = 0.0;
        nuevo_estudiante[i].nombre = utiles::ingreso_texto("Nombre".to_string());
        println!("NUMERO DE MATRICULA\n");
        nuevo_estudiante[i].matricula = utiles::texto_numero("matricula".to_string());
        println!("NOTAS\n");
        //iterar arreglo de la estrucura NOTAS
    
        for x in 0..nuevo_estudiante[i].notas.len() as usize{
            new_num = utiles::texto_numero_float();
            nuevo_estudiante[i].notas[x] = new_num;
            prom += new_num;
        }
        //calcular promedio del estudiante
        let prom:f32 = prom / 5.0;
        println!("EL ESTUDIANTE TIENE UN PROMEDIO DE {}", prom);
        if prom >= 4.0 {
            println!("El estudiante aprobó");
        } else {
            println!("El estudiante reprobó");
        }
    }

    return nuevo_estudiante;
}


fn main(){

    let arreglo = create_array();
    //read_array(&arreglo);
    let arreglo = edit_array(arreglo);
    read_array(&arreglo);

}