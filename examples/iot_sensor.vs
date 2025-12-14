// DEMOSTRACIÓN: IoT Sensor State Dominance
// Objetivo: Calcular datos sin try/catch. El estado se propaga.

fn main() {
    print("--- IOT SENSOR MONITOR ---");

    // 1. Estado Inicial: Sensor cargando (Tipado estricto)
    var sensor_status := loading;
    var data_buffer := 0;

    print("Status Inicial:");
    print(sensor_status);

    // 2. Intentamos procesar datos (Simulamos un cálculo)
    // En Rust esto fallaría si data_buffer es Int y status es Loading.
    // En Vaso, Loading DOMINA al entero.
    print("Procesando datos del buffer...");
    
    // Sumamos: 0 (Int) + loading (VBit) -> loading
    data_buffer += sensor_status; 

    print("Resultado del Buffer (Propagado):");
    print(data_buffer); // Debería imprimir "loading"

    // 3. Simulamos un Error Crítico de Hardware
    print("!!! ALERTA: Falla de Hardware detectada !!!");
    var hardware_error := error;

    // El error contamina todo el flujo
    data_buffer += hardware_error;

    print("Estado Final del Sistema:");
    print(data_buffer); // Debería imprimir "error"
}