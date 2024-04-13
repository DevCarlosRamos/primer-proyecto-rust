use std::io; // Importamos la biblioteca estándar de E/S para leer desde la entrada estándar.

fn main() {
    // La función principal de Rust. Es el punto de entrada de nuestro programa.

    println!("¡Hola! ¿Cuál es tu nombre?"); 
    // Imprime un mensaje en la consola pidiendo al usuario que ingrese su nombre.

    let mut nombre = String::new(); 
    // Creamos una nueva variable llamada `nombre` para almacenar la entrada del usuario.
    // `mut` indica que la variable es mutable, es decir, puede cambiar su valor.

    io::stdin().read_line(&mut nombre)
        .expect("Error al leer la entrada."); 
    // Lee la entrada del usuario y la guarda en la variable `nombre`.
    // `read_line()` lee la entrada del usuario y la almacena en una cadena (`String`).
    // `expect()` maneja posibles errores en la lectura de entrada.

    println!("¡Hola, {}!", nombre.trim());
    // Imprime un saludo usando el nombre ingresado por el usuario.
    // `trim()` elimina cualquier espacio en blanco al principio o al final del nombre.
}
