mod models;
mod views;
mod controllers;

use controllers::factura_controller::crear_factura;
use views::consola::mostrar_factura;

fn main() {
    let factura = crear_factura();
    mostrar_factura(&factura);
}
