mod almacen;
mod articulo;

use almacen::Almacen;
use articulo::Articulo;

use std::io::{self, Write};
fn main() {
    let mut articulos: Vec<Articulo> = Vec::new();

    println!("Al Admin Central");
    print!(
        "   1. Crear Nuevo Articulo\n
            2. Añadir Articulo a Almacen\n
            3. Listar Articulos de un almacen\n
            4. Mover Articulos entre Almacenes\n"
    );

    articulos.push(crear_articulo());
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
    print!("Articulo creado: {}\n", articulo.to_string());
    articulo
}
