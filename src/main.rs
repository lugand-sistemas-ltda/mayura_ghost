use gtk::prelude::*;
use gtk::{Image, Window, WindowType};
use gdk::Display;

fn main() {
    // Inicializa GTK
    gtk::init().expect("Failed to initialize GTK.");

    // Cria a janela
    let window = Window::new(WindowType::Popup);
    window.set_decorated(false); // Remove os botões e bordas
    window.set_app_paintable(true); // Permite que a janela seja pintada manualmente
    window.set_default_size(250, 280);

    // Caminho do GIF armazenado nos recursos
    let gif_path = "resources/gifs/mayura.gif";
    // Caminha para imagens armazenas nos recursos
    let mayura_idle_img = "resources/imgs/mayura-idle.png";
    let mayura_angry_img = "resources/imgs/mayura-angry.png";
    let mayura_wait_img = "resources/imgs/mayura-wait.png";

    // Carrega o GIF e cria um widget de imagem
    let image = Image::from_file(gif_path);
    
    // Define um tamanho padrão para a imagem
    image.set_pixel_size(250); // Define a largura e altura máxima para a imagem

    // Adiciona a imagem à janela
    window.add(&image);

    // Obtém o monitor padrão para pegar a resolução da tela
    if let Some(display) = Display::default() {
        if let Some(monitor) = display.primary_monitor() {
            let monitor_geo = monitor.geometry();
            let screen_height = monitor_geo.height();
            let screen_width = monitor_geo.width();

            // Define a posição da janela no canto inferior esquerdo
            window.move_(screen_width - 250, screen_height - 280);
        }
    }

    // Mostra todos os widgets da janela
    window.show_all();

    // Conecta o sinal de fechar a aplicação
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        false.into() // Retorna false para permitir que a janela seja fechada
    });

    // Exibir a janela
    window.show();

    // Mantém a janela visível
    gtk::main();
}
