fn main() {
    print("1. Soy Vaso hablando nativo");

    // Bloque Python (Interoperabilidad)
    // Usamos ; al final para separar las instrucciones
    use(python): {
        print("2. Soy Python ejecutado desde dentro de Vaso");
        import math;
        print("   Pi desde Python es:");
        print(math.pi);
    }

    print("3. Volvemos a Vaso. Fin.");
}