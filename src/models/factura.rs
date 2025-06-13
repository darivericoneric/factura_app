pub struct Producto {
    pub nombre: String,
    pub precio: f64,
    pub cantidad: u32,
}

impl Producto {
    pub fn subtotal(&self) -> f64 {
        self.precio * self.cantidad as f64
    }
}

pub struct Cliente {
    pub nombre: String,
    pub identificacion: Option<String>,
    pub direccion: Option<String>,
}

pub struct Factura {
    pub cliente: Cliente,
    pub productos: Vec<Producto>,
    pub impuestos: f64, 
    pub descuento: f64,
}

impl Factura {
    pub fn total(&self) -> f64 {
        let subtotal: f64 = self.productos.iter().map(|p| p.subtotal()).sum();
        let impuesto_total = subtotal * self.impuestos;
        let descuento_total = subtotal * self.descuento;
        subtotal + impuesto_total - descuento_total
    }
}
