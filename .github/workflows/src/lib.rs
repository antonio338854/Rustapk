use android_activity::{AndroidApp, InputStatus, MainEvent, PollEvent};
use log::{info, Level};

#[no_mangle]
fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_min_level(Level::Info),
    );

    info!("Iniciando App de Rastreamento de Mãos...");

    loop {
        app.poll_events(Some(std::time::Duration::from_millis(16)), |event| {
            match event {
                PollEvent::Main(MainEvent::Resume) => {
                    info!("App Resumido: Inicializando Câmera Traseira...");
                    // Aqui você chamaria a função para abrir a ACameraManager (NDK)
                }
                PollEvent::Main(MainEvent::Destroy) => {
                    info!("Encerrando...");
                }
                _ => {}
            }
        });
        
        // Loop de processamento de frame:
        // 1. Capturar frame da câmera via NDK
        // 2. Enviar para o MediaPipe
        // 3. Renderizar esqueleto na tela
    }
}
