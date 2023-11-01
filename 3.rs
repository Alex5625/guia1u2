use std::default::Default;
mod utiles;
#[derive(Default)]
#[derive(Clone)]


struct Peliculas {
    titulo: String,
    director:String,
    anio_lanzamiento:i32
}

fn create_array() -> [Peliculas;4]{

    let arreglo:[Peliculas;4] = Default::default();
    return arreglo;

} 
fn read_array(cine: &[Peliculas;4]) -> (){

    for (number, item) in cine.iter().enumerate() {
        print!("{}: Titulo de la pelicula: {} ", number+1, item.titulo);
        print!("  Director: {} \n", item.director);
        print!("    Año de lanzamiento: {:?}\n", item.anio_lanzamiento);
    }
}


fn edit_array(cine: [Peliculas;4]) -> [Peliculas;4]{
    let mut nuevo_cine :[Peliculas;4]= cine.clone(); 

    for i in 0..nuevo_cine.len() as usize{
        nuevo_cine[i].titulo = utiles::ingreso_texto("Titulo de la pelicula".to_string());
        nuevo_cine[i].director = utiles::ingreso_texto("Director".to_string());
        nuevo_cine[i].anio_lanzamiento = utiles::texto_numero("año de lanzamiento".to_string());
    }
    return nuevo_cine;
}

fn buscador(cine: &[Peliculas;4]) ->(){
    
    loop{
        println!("Quieres saber que peliculas que estan dentro de este arreglo se estrenaron en el año que digites?");
        if utiles::si_no(){
            let busqueda = utiles::texto_numero("año de publicacion para buscar".to_string());
            for (i,x) in cine.iter().enumerate(){
                if cine[i].anio_lanzamiento == busqueda {
                    print!(" Titulo de la pelicula: {} ", x.titulo);
                    print!("  Director: {}", x.director);
                    print!("    Año de lanzamiento: {:?}\n", x.anio_lanzamiento);
                }
            }
        } else {
            break
        }
}


}

fn main(){
    let arreglo = create_array();
    read_array(&arreglo);
    let arreglo = edit_array(arreglo);
    buscador(&arreglo);
}