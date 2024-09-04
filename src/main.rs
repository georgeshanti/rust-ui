use gtk::{DrawingArea, glib, prelude::*, cairo::Context};
use ui::core::{App, AppProps, Element, ProppedWidget, Widget};

static mut ELEMENT_TREE: Element = Element{};

fn draw(_: &DrawingArea, ctx: &Context, w: i32, h: i32) {
    ctx.rectangle(0 as f64, 0 as f64, w as f64, h as f64);
    ctx.fill().expect("Could not fill rectagle");
}

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.basic")
        .build();
    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title(Some("First GTK Program"));
    window.set_default_size(350, 70);

    let drawing_ares: DrawingArea = DrawingArea::new();
    drawing_ares.set_draw_func(draw);

    window.set_child(Some(&drawing_ares));
    let prop = AppProps{};
    let app: &mut App = &mut App::create(prop);
    unsafe { ELEMENT_TREE = app.draw() };

    window.present();
}