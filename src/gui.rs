use druid::widget::{Align, Flex, Label, TextBox};
use druid::{AppLauncher, Data, Lens, LocalizedString, Widget, WindowDesc, WidgetExt};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<InitState> = LocalizedString::new("Rex - The rust based Pokédex");

#[derive(Clone, Data, Lens)]
struct InitState {
    name: String,
}

fn build_root_widget() -> impl Widget<InitState> {
    // a label that will determine its text based on the current app data.
    let label = Label::new(format!("Dex"));
    // a textbox that modifies `name`.
    let textbox = TextBox::new()
        .with_placeholder("001")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(InitState::name);

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(textbox);

    // center the two widgets in the available space
    Align::centered(layout)
}

pub fn build_ui() {
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .window_size((600.0, 400.0));

    // create the initial app state
    let initial_state = InitState {
        name: "".into(),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}