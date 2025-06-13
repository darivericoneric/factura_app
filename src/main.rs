use chrono::format::format;
use colored::*;

#[derive(Debug)]
struct Company {
    name: String,
    address: String,
    telephone: String,
    email: String,
}

fn main() {
    print!("\x1B[2J\x1B[1;1H");

    let tech_nova = Company {
        name: String::from("Tech Nova, S.A. de C.V."),
        address: String::from("Tabasco, Cárdenas, Col. Centro, 86500"),
        telephone: String::from("937-123-4567"),
        email: String::from("contacto@technova.com"),
    };
    
    let custumer_01 = Company {
        name: String::from("Servicios de Limpieza S.A. de C.V."),
        address: String::from("Tabasco, Villahermosa, Col. Tamulte, 23456"),
        telephone: String::from("993-987-6543"),
        email: String::from("facturas@serviciolimpieza.com"),
    };

    let info_company = "Información Empresa"; 
    let info_customer ="Información Cliente";

    println!("");
    print!("{}\t\t\t\t\t\t\t", info_company.green()); println!("{}", info_customer.green());
    print!("------------------------------\t\t\t\t\t\t"); println!("------------------------------");
    print!("Nombre: {}\t\t\t\t\t\t", tech_nova.name.green()); println!("Nombre: {}", custumer_01.name.green());
    print!("Dirección: {}\t\t\t", tech_nova.address.green()); println!("Dirección: {}", custumer_01.address.green());
    print!("Telefono: {}\t\t\t\t\t\t\t", tech_nova.telephone.green()); println!("Telefono: {}", custumer_01.telephone.green());
    print!("Correo: {}\t\t\t\t\t\t", tech_nova.email.green()); println!("Correo: {}", custumer_01.email.green());
    print!("------------------------------\t\t\t\t\t\t"); println!("------------------------------");
    println!("");
    println!("");

    let id_factura = "001-2025";
    let id_costumer = "12345";
    let fecha = "11-06-2025";
    let condiciones_de_pagos ="30 días";

    print!("Nº de la Factura: {}\t\t\t\t\t\t",id_factura.green()); 
    println!("Fecha de la Factura: {}", fecha.green());
    print!("ID Cliente: {}",id_costumer.green()); 
    print!("\t\t\t\t\t\t\tCondiciones de Pago: ");
    println!("{}", condiciones_de_pagos.green());
    println!("");
    println!("");


    let detalle_numero = "Nº";
    let detalle_descripcion = "Descripción";
    let detalle_cantidad = "Cant.";
    let detalle_unidad = "Unidad";
    let detalle_total = "Total";

    // Encabezado
    println!("┌────────┬────────────────────────────────────────────────┬────────┬────────┬────────┐");
    println!(
        "│ {:<6} │ {:<46} │ {:<6} │ {:<6} │ {:<6} │",
        detalle_numero,
        detalle_descripcion,
        detalle_cantidad,
        detalle_unidad,
        detalle_total
    );
    println!("├────────┼────────────────────────────────────────────────┼────────┼────────┼────────┤");

    // Fila 1
    println!(
        "│ {:<6} │ {:<46} │ {:<6} │ {:<6} │ {:<6} │",
        "001", "Servicio de limpieza mensual", "1", "mes", "$500"
    );

    // Fila 2
    println!(
        "│ {:<6} │ {:<46} │ {:<6} │ {:<6} │ {:<6} │",
        "002", "Suministro de productos de limpieza", "5", "paqs", "$250"
    );

    // Fila 3
    println!(
        "│ {:<6} │ {:<46} │ {:<6} │ {:<6} │ {:<6} │",
        "003", "Desinfección profunda de oficinas", "2", "serv", "$1,200"
    );

    // Cierre
    println!("└────────┴────────────────────────────────────────────────┴────────┴────────┴────────┘");
}

