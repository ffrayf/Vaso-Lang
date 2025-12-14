// -----------------------------------------------------
// 🚀 VASO SPACE PROGRAM: ORBITAL LAUNCH CONTROLLER
// -----------------------------------------------------
// Objetivo: Demostrar "State Dominance Arithmetic".
// Si UN solo sistema falla, todo el lanzamiento se cancela
// matemáticamente, sin un solo 'if' de error.
// -----------------------------------------------------

fn main() {
    print("--- 🛰️ INICIANDO SECUENCIA DE LANZAMIENTO ---");

    // 1. INICIALIZACION DE SUBSISTEMAS
    // Todos empiezan en 'loading' (chequeo)
    var navigation := loading;
    var fuel       := loading;
    var thrusters  := loading;

    print("Diagnostico de Sistemas:");

    // 2. SIMULACION DE TELEMETRIA (Cambia esto para probar)
    // Vamos a simular que Navigation y Fuel estan OK (on)
    navigation = on;
    fuel       = on;
    
    // PERO... los propulsores tienen un problema desconocido
    // En C++ esto podría ser un 'null pointer' que crashea todo.
    // En Vaso, es un estado seguro 'unknown'.
    thrusters = unknown;

    // 3. LA LINEA MAESTRA (La magia de Vaso)
    // En otros lenguajes: 
    // if nav.is_ok() && fuel.is_ok() && thrusters.is_ok() ...
    //
    // En Vaso: Sumamos los estados.
    // La Jerarquía (Error > Unknown > Loading > On) decide el resultado.
    
    var global_status := navigation;
    global_status += fuel;
    global_status += thrusters;

    print("Estado Global del Cohete (Calculado):");
    print(global_status); // Debería ser 'unknown' o 'loading', bloqueando el despegue.

    // 4. DECISION DE LANZAMIENTO (Pattern Matching)
    print("--- T-MINUS 10 SECONDS ---");
    print("Decision del Computador Central:");

    match global_status {
        on => {
            print("✅ GO FOR LAUNCH! Despegue exitoso.");
            print("🚀🚀🚀");
        }
        loading => {
            print("⚠️ HOLD: Sistemas aun verificando. Pausa cronometro.");
        }
        unknown => {
            print("⚠️ ABORT: Telemetria incompleta. No es seguro volar.");
        }
        error => {
            print("🚨 ABORT: FALLA CRITICA DETECTADA. Evacuacion.");
        }
        off => {
            print("🛑 Sistemas apagados.");
        }
    }

    // 5. INTENTO DE RECUPERACION (Auto-Fix)
    print("--- INTENTANDO REPARACION DE TELEMETRIA ---");
    
    // Arreglamos el sensor de propulsores
    thrusters = on;

    // Recalculamos matemáticamente
    global_status = navigation;
    global_status += fuel;
    global_status += thrusters;

    print("Nuevo Estado Global:");
    print(global_status); // Ahora debería ser 'on'

    match global_status {
        on => {
            print("✅ RECUPERACION EXITOSA. LANZAMIENTO INICIADO.");
            print("🚀 HASTA LAS ESTRELLAS, SOCIO!");
        }
    }
}