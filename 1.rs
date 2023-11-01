use std::default::Default;
mod utiles;

#[derive(Default)]
#[derive(Clone)]
struct Libro {
    titulo: String,
    autor: String,
    anio: i32,
}


fn create_array_libros() -> [Libro;10]{

    let arreglo: [Libro; 10] = Default::default();
    return arreglo;

}

fn read_array_libros(libros: &[Libro; 10]) -> () {

    for (number, item) in libros.iter().enumerate() {
        print!("{}: Título: {} ", number+1, item.titulo);
        print!("Autor: {} ", item.autor);
        print!("Anio publicacion: {}\n", item.anio);
    }

}



fn edit_array_libros(libros: [Libro; 10]) -> [Libro; 10]{
    let mut nuevos_libros: [Libro; 10] = libros.clone();

    for i in 0..nuevos_libros.len(){
        nuevos_libros[i].titulo = utiles::ingreso_texto("Título".to_string());
        nuevos_libros[i].autor = utiles::ingreso_texto("Autor".to_string());
        print!("Ingrese el anio de publicación: ");
        nuevos_libros[i].anio = utiles::texto_numero("año de publicacion".to_string());
}

    return nuevos_libros
}





fn main() {


    let arreglo = create_array_libros();
    read_array_libros(&arreglo);

    println!("##########");

    let arreglo = edit_array_libros(arreglo);
    read_array_libros(&arreglo);

}

