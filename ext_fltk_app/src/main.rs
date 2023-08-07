use fltk::*;
use fltk::app::Scheme;
use fltk::enums::Color;
use fltk::frame::Frame;
use fltk::input::Input;
use fltk::prelude::*;
use fltk_theme::*;
use fltk_theme::colors::aqua::light::systemBlueColor;

fn main() {
    let mut app = app::App::default();
    let widget_theme = WidgetTheme::new(ThemeType::Aero);
    widget_theme.apply();
    let widget_scheme = WidgetScheme::new(SchemeType::Aqua);
    widget_scheme.apply();
    let mut window = window::Window::new(100, 100, 400, 300, "My window");
    window.set_color(Color::White);
    let mut frame = frame::Frame::default().with_size(100, 40).center_of(&window);
    let mut btn = button::Button::new(10, 10, 80, 50, "点我");
    btn.set_color(Color::from_rgb(242, 238, 223));
    let mut input = input::Input::new(160, 10, 160, 40, "输入: ");
    input.set_color(Color::from_rgb(239, 233, 249));
    window.end();
    window.show();

    let mut p_input = &mut input as *mut Input;
    let mut p_frame = &mut frame as *mut Frame;
    btn.set_callback(move |btn| {
        unsafe {(*p_frame).set_label(&(*p_input).value());}
        btn.set_label("好");

    });

    input.set_trigger(enums::CallbackTrigger::Changed);
    input.set_callback(move |input| {
       // frame.set_label(&input.value())
    });

    app.run().unwrap();
}
