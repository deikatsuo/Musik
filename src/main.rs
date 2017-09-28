extern crate gtk;
extern crate gio;

pub mod musik;

use gio::ApplicationExt;

fn main() {
    match gtk::Application::new("org.haseup.musik",
                                gio::APPLICATION_FLAGS_NONE)
    {
        Ok(app) => {
            app.connect_activate(|app| musik::init(app));
            app.run(&[]);
        },
        Err(e)  => {
            println!("Failed to initialize Gtk: {:?}", e);
            return;
        }
    }
}