fn main() {
    print("--- VASO UNIVERSAL DEMO ---");

    // 1. Usando la Standard Library (Extensibilidad)
    print("Obteniendo Timestamp del sistema...");
    
    // Llamada a modulo nativo de Rust (Time.now)
    var tiempo := Time.now(); 
    
    print("Timestamp UNIX:");
    print(tiempo);

    // 2. Simulando latencia de red en el tiempo
    var red_status := loading;
    
    print("Intentando sincronizar reloj...");
    // Aritmetica: Tiempo + Loading = Loading
    tiempo += red_status;

    print("Estado del Reloj:");
    print(tiempo);

    print("--- FIN DEMO ---");
}