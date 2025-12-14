fn main() {
    print("--- VASO CALCULATOR ---");
    
    // 1. Probamos el Input
    print("Escribe un numero inicial (ej. 10):");
    var x := input;

    print("Multiplicando por 2...");
    
    // 2. Probamos Multiplicación (*=)
    x *= 2;
    print("Resultado:");
    print(x);

    print("Restando 5...");
    
    // 3. Probamos Resta (-=)
    x -= 5;
    print(x);

    // 4. Bucle Inverso (Cuenta regresiva)
    print("Iniciando cuenta regresiva...");
    while x > 0 {
        print(x);
        x -= 5; // Bajamos de 5 en 5
    }

    print("🚀 Fin del programa.");
}