// 🏗️ VASO CI/CD PIPELINE (Concept Proof)
// Reemplazando Bash con lógica de estados segura.

fn main() {
    print("--- 🚀 INICIANDO DEPLOY AUTOMATIZADO ---");

    print("1. Ejecutando Tests Unitarios...");
    // Simulamos tests con 'echo'. En produccion seria 'npm test'
    var tests := Sys.exec("echo", "Running tests... OK");
    print(tests);

    print("2. Construyendo Artefacto...");
    // Simulamos build. Si este falla, todo el deploy debe parar.
    var build := Sys.exec("echo", "Building binary... OK");
    print(build);

    // --- MAGIA DE VASO: PROPAGACION DE ESTADO ---
    // En Bash: if [ $? -ne 0 ]; then exit 1; fi (repetido mil veces)
    // En Vaso: Suma simple.
    
    var pipeline_health := tests + build;

    print("Estado del Pipeline:");
    print(pipeline_health);

    match pipeline_health {
        on => {
            print("✅ Tests y Build exitosos. Procediendo a Deploy...");
            var deploy := Sys.exec("echo", "Deploying to AWS S3... SUCCESS");
            print(deploy);
        }
        error(msg) => {
            print("🚨 PIPELINE FALLIDO. Deteniendo deploy.");
            print("Causa: " + msg);
            // Aqui podriamos mandar alerta a Slack
        }
    }
}