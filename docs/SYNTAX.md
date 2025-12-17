# VASO LANGUAGE SPECIFICATION (v1.0 - Draft)
# Philosophy: "Simple as a spoon, Fast as a rocket."

# 1. VARIABLES Y MUTABILIDAD
# -----------------------------------------------------------
# Filosofía: Seguridad por defecto (Inmutabilidad preferida).
# Sintaxis: palabra_clave nombre : tipo := valor;

val pi := 3.1415;           # Inmutable (Constante) - Tipo inferido (float)
var contador := 0;          # Mutable (Variable) - Tipo inferido (int)
val nombre : string := "Vaso"; # Tipado explícito (Estilo C++/Rust)

# 2. SISTEMA DE TIPOS "QUANTUM-READY" (La Salsa)
# -----------------------------------------------------------
# Vaso introduce 'vbit' (5-State Logic) nativo.
# 0 = No (False)
# 1 = Si (True)
# 2 = Unknown (Probable/Pending)
# 3 = Paradox (Superposition)
# 4 = Null (Void/Empty)

val estado : vbit := 2; 

# 3. ESTRUCTURAS DE CONTROL
# -----------------------------------------------------------
# Condicionales (Sin paréntesis obligatorios en la condición)
if estado == 1 {
    print("Es verdadero");
} else {
    print("No es verdadero");
}

# Match (Esencial para manejar lógica de 5 estados)
match estado {
    case 1: print("Positivo");
    case 0: print("Negativo");
    case 2: print("Necesito más datos...");
    case 3: print("Error de lógica cuántica");
    else:   print("Vacío");
}

# Bucles (Loops)
while x < 10 {
    print(x);
    x := x + 1;
}

# 4. FUNCIONES
# -----------------------------------------------------------
# Sintaxis limpia 'fn'. Retorno opcional con '->'.

fn saludar(nombre: string) {
    print("Hola " + nombre);
}

fn sumar(a: int, b: int) -> int {
    return a + b;
}

# 5. EL PUENTE CAMALEÓN (Interoperabilidad Universal)
# -----------------------------------------------------------
# Permite inyectar código de otros lenguajes nativamente.
# El compilador delega la ejecución al entorno del host.

use(python): {
    import math;
    print(math.sqrt(16));
}

use(javascript): {
    console.log("Hola desde Node.js");
}

# 6. ESTRUCTURAS DE DATOS (Futuro)
# -----------------------------------------------------------
struct Usuario {
    nombre: string;
    id: int;
}

# 7. FILOSOFÍA DE SINTAXIS
# -----------------------------------------------------------
# - No requiere 'public static void main'.
# - El punto y coma ';' es recomendado para separar instrucciones pero
#   el parser inteligente podría inferirlo por saltos de línea (Pythonic).
# - Bloques delimitados por '{ }' para evitar errores de indentación (C++ Style).