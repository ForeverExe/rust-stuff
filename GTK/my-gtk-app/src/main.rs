use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    //Connect to "activate" signal of 'app'
    app.connect_activate(build_ui);
    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    //create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("La mia prima App in GTK!")
        .build();

    window.present();
}
