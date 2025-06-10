use crate::models::factura::{Factura, Producto};

pub fn mostrar_factura(factura: &Factura) {
    println!("--- FACTURA ---");
    println!("Cliente: {}", factura.cliente.nombre);
    if let Some(id) = &factura.cliente.identificacion {
        println!("ID: {}", id);
    }
    println!("Productos:");
    println!("{:<20} {:>8} {:>8} {:>10}", "Nombre", "Precio", "Cantidad", "Subtotal");

    for p in &factura.productos {
        println!(
            "{:<20} {:>8.2} {:>8} {:>10.2}",
            p.nombre,
            p.precio,
            p.cantidad,
            p.subtotal()
        );
    }

    println!("---------------------------");
    println!("Total: {:.2}", factura.total());
}
