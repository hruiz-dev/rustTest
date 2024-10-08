use crate::articulo::Articulo;

use uuid::Uuid;
pub struct Almacen {
    pub nombre: String,
    pub articulos: Vec<Articulo>,
}
/*


*/
impl Almacen {
    pub fn new(nombre: String) -> Almacen {
        Almacen {
            nombre,
            articulos: Vec::new(),
        }
    }

    pub fn add_articulo(&mut self, producto: Articulo) {
        self.articulos.push(producto);
    }

    pub fn get_articulos(&self) -> &Vec<Articulo> {
        &self.articulos
    }

    pub fn get_articulo(&self, uuid: &Uuid) -> Option<&Articulo> {
        self.articulos.iter().find(|p| p.get_uuid() == uuid)
    }

    pub fn get_articulo_mut(&mut self, uuid: &Uuid) -> Option<&mut Articulo> {
        self.articulos.iter_mut().find(|p| p.get_uuid() == uuid)
    }

    pub fn to_string(&self) -> String {
        let mut result = format!("Almacen: {}\n", self.nombre);
        for articulo in &self.articulos {
            result.push_str(&format!("{}\n", articulo.get_nombre()));
        }
        result
    }
}
