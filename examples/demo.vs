fn main() {
    print("Iniciando prueba de inteligencia...");

    // OJO AQUÍ: Usamos := (dos puntos e igual) para asignar.
    // Antes teniamos "=" y por eso falló.
    val estado : vbit := 2;

    // Prueba de fallo (No debe salir)
    if estado == 1 {
        print("ERROR: Esto NO deberia salir");
    }

    // Prueba de éxito (SI debe salir)
    // El motor busca 'estado' en RAM, ve que es 2, y entra.
    if estado == 2 {
        print("EXITO: El sistema detecto el estado Desconocido (2)");
    }

    print("Fin del programa.");
}