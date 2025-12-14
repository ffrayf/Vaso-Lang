// SIMULADOR DE SEMAFORO INTELIGENTE (State Machine)

fn main() {
    print("--- INICIANDO SISTEMA DE TRAFICO ---");

    // 1. Estado inicial del sistema
    var system_state := loading;
    print("Estado del Sistema:");
    print(system_state);

    // 2. Simulamos que el sistema cargó correctamente
    system_state = on;
    
    // 3. Variable de control del semáforo
    var light_signal := 0; // 0: Rojo, 1: Verde

    // 4. Lógica de Control basada en Estado (MATCH)
    // Esto es lo que el TF quería ver: Ramificación por estado
    
    print("Analizando estado para decision...");
    
    match system_state {
        loading => {
            print("Sistema cargando... Espere.");
        }
        error => {
            print("CRITICO: Modo de seguridad activado.");
            light_signal = 0; // Poner en Rojo por seguridad
        }
        on => {
            print("Sistema ONLINE. Operacion normal.");
            light_signal = 1; // Poner en Verde
        }
    }

    print("Senal de Trafico (0=Rojo, 1=Verde):");
    print(light_signal);

    // 5. Simulación de Falla (Dominancia)
    print("--- SIMULANDO CORTE DE ENERGIA ---");
    var critical_failure := error;
    
    // El sistema se infecta
    match critical_failure {
        error => {
            print("DETECTADO: Falla critica.");
            print("Reiniciando protocolos...");
        }
    }
}