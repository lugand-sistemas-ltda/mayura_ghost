use gtk::prelude::*;
use gtk::{Image, Window, WindowType};
use gdk::Display;

fn main() {
    // Init GTK
    gtk::init().expect("Failed to initialize GTK.");

    // Create window
    let window = Window::new(WindowType::Popup);
    window.set_decorated(false); // Remove os botões e bordas
    window.set_app_paintable(true); // Permite que a janela seja pintada manualmente
    window.set_default_size(250, 280);

    // Resources root
    let gif_path = "resources/gifs/mayura.gif";
    let mayura_idle_img = "resources/imgs/mayura-idle.png";
    let mayura_angry_img = "resources/imgs/mayura-angry.png";
    let mayura_wait_img = "resources/imgs/mayura-wait.png";

    // Load resource and create widget (GIF)
    let image = Image::from_file(gif_path);
    
    // Define standard image size
    image.set_pixel_size(250);

    // Add widget image to the window
    window.add(&image);

    // Get standard monitor to get window resolution
    if let Some(display) = Display::default() {
        if let Some(monitor) = display.primary_monitor() {
            let monitor_geo = monitor.geometry();
            let screen_height = monitor_geo.height();
            let screen_width = monitor_geo.width();

            // Define window position
            window.move_(screen_width - 250, screen_height - 280);
        }
    }

    // Show all window widgets
    window.show_all();

    // Connect signal to close the application
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        // Return 'false' to allow the window to be closed
        false.into()
    });

    // Show window
    window.show();

    // Keep the window visible
    gtk::main();
}
