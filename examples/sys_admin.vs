fn main() {
    print("--- VASO SYSTEM OVERLORD ---");

    print("OS Detectado:");
    var os := Sys.os();
    print(os);

    print("Intentando leer archivo secreto...");
    // Esto va a fallar y devolver un error rico
    var content := File.read("secreto_inexistente.txt");

    print("Resultado de lectura:");
    print(content); // Debe decir error("IO Error: ...")

    // Prueba de dominancia con Strings
    var log := "Log del sistema: ";
    log += content; // El error infecta al string

    print("Log Final:");
    print(log);
}