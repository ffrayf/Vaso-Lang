// 1. Definimos la estructura biológica (Ingeniería Pesada)
mold Amigdala {
    nivel_miedo: int;
    activada: vbit;
}

fn main() {
    print("--- INICIANDO SIMULACION NEUROLOGICA ---");

    // 2. Instanciamos (Creamos el objeto)
    var cerebro := new Amigdala;

    print("Estado inicial (Default 0):");
    // Accedemos con punto (.)
    print(cerebro.nivel_miedo);

    print("Estimulo recibido! Aumentando miedo...");
    
    // 3. Modificamos la propiedad interna
    cerebro.nivel_miedo = 100;
    cerebro.activada = 1;

    print("Nuevo nivel de miedo:");
    print(cerebro.nivel_miedo);
    
    if cerebro.activada == 1 {
        print("ALERTA: Amigdala Activada (Fight or Flight)");
    }
}