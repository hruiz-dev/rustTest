mod almacen;
mod articulo;

use almacen::Almacen;
use articulo::Articulo;

use std::io::{self, Write};
fn main() {
    let mut articulos: Vec<Articulo> = Vec::new();
    let mut almacenes: Vec<Almacen> = Vec::new();

    almacenes.push(Almacen::new("Almacen Central".to_string()));
    almacenes.push(Almacen::new("Almacen Secundario".to_string()));
    almacenes.push(Almacen::new("Almacen Kardex".to_string()));

    println!("Al Admin Central");
    print!(
        "{}{}{}{}",
        "1. Crear Nuevo Articulo\n",
        "2. Añadir Articulo a Almacen\n",
        "3. Listar Articulos de un almacen\n",
        "4. Mover Articulos entre Almacenes\n"
    );
    articulos.push(crear_articulo());
}

fn add_articulo_almacen(almacenes: &mut Vec<Almacen>, articulos: &mut Vec<Articulo>) {
    mostrar_almacenes(&almacenes);

    let mut almacen_index: i32 = 0;

    loop {
        let mut almacen_str = String::new();
        io::stdin()
            .read_line(&mut almacen_str)
            .expect("Failed to read line");
        almacen_str = almacen_str.trim().to_string();

        match almacen_str.parse() {
            Ok(num) => {
                almacen_index = num;
                if almacen_index < 0 || almacen_index >= almacenes.len() as i32 {
                    println!("Error: El almacen debe ser un número válido.");
                    // Reintentar
                } else {
                    break;
                }
            }
            Err(_) => {
                println!("Error: El almacen debe ser un número entero válido.");
                // Reintentar
            }
        }
    }
    let almacen = &mut almacenes[almacen_index as usize];

    mostrar_articulos(articulos);

    let mut articulo_index: i32 = 0;

    loop {
        let mut articulo_str = String::new();
        io::stdin()
            .read_line(&mut articulo_str)
            .expect("Failed to read line");
        articulo_str = articulo_str.trim().to_string();

        match articulo_str.parse() {
            Ok(num) => {
                articulo_index = num;
                if articulo_index < 0 || articulo_index >= articulos.len() as i32 {
                    println!("Error: El articulo debe ser un número válido.");
                    // Reintentar
                } else {
                    break;
                }
            }
            Err(_) => {
                println!("Error: El articulo debe ser un número entero válido.");
                // Reintentar
            }
        }
    }
    let articulo = articulos.remove(articulo_index as usize);

    almacen.add_articulo(articulo);
}

fn mostrar_almacenes(almacenes: &Vec<Almacen>) {
    for (index, almacen) in almacenes.iter().enumerate() {
        println!("{}: {}", index, almacen.get_nombre());
    }
}

fn mostrar_articulos(articulos: &Vec<Articulo>) {
    for articulo in articulos {
        println!("{}", articulo.to_string());
    }
}

fn crear_articulo() -> Articulo {
    let mut nombre = String::new();
    let mut stock_str = String::new();

    print!("Nombre del articulo: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut nombre)
        .expect("Failed to read line");
    nombre = nombre.trim().to_string();

    print!("Stock: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut stock_str)
        .expect("Failed to read line");
    stock_str = stock_str.trim().to_string();

    let stock: i32 = match stock_str.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El stock debe ser un número entero válido.");
            return crear_articulo(); // Reintentar
        }
    };

    let articulo = Articulo::new(nombre, stock);
    print!("Articulo creado:\n {}", articulo.to_string());
    articulo
}
