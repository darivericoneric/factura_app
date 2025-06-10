use std::io::{self, Write};

use crate::models::factura::{Cliente, Factura, Producto};

pub fn crear_factura() -> Factura {
    let mut input = String::new();

    println!("Ingrese nombre del cliente:");
    io::stdin().read_line(&mut input).unwrap();
    let nombre = input.trim().to_string();
    input.clear();

    let cliente = Cliente {
        nombre,
        identificacion: None,
        direccion: None,
    };

    let mut productos = Vec::new();

    loop {
        println!("Agregar producto (s para salir)? [s/n]:");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() == "s" {
            break;
        }

        input.clear();
        println!("Nombre del producto:");
        io::stdin().read_line(&mut input).unwrap();
        let nombre = input.trim().to_string();

        input.clear();
        println!("Precio:");
        io::stdin().read_line(&mut input).unwrap();
        let precio: f64 = input.trim().parse().unwrap_or(0.0);

        input.clear();
        println!("Cantidad:");
        io::stdin().read_line(&mut input).unwrap();
        let cantidad: u32 = input.trim().parse().unwrap_or(1);

        productos.push(Producto {
            nombre,
            precio,
            cantidad,
        });
    }

    Factura {
        cliente,
        productos,
        impuestos: 0.15,
        descuento: 0.0,
    }
}
