use uuid::Uuid;

pub struct Articulo {
    nombre: String,
    uuid: Uuid,
    stock: i32,
}

impl Articulo {
    pub fn new(nombre: String, stock: i32) -> Articulo {
        Articulo {
            nombre,
            uuid: Uuid::new_v4(),
            stock,
        }
    }

    pub fn get_nombre(&self) -> &String {
        &self.nombre
    }

    pub fn get_uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn get_stock(&self) -> i32 {
        self.stock
    }

    pub fn set_stock(&mut self, stock: i32) {
        self.stock = stock;
    }

    pub fn to_string(&self) -> String {
        format!(
            "Articulo: {}\nStock: {}\nUUID: {}\n",
            self.nombre, self.stock, self.uuid
        )
    }
}
