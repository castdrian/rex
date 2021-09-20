use druid::widget::{Align, Flex, Label, TextBox, Button, Image};
use druid::{AppLauncher, Data, Lens, LocalizedString, Widget, WindowDesc, WidgetExt};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<InitState> = LocalizedString::new("Rex - The rust based Pokédex");

#[derive(Clone, Data, Lens)]
struct InitState {
    name: String,
}

fn build_root_widget() -> impl Widget<InitState> {
    let hash = Label::new(format!("#"));
    let dex = Label::new(format!("Dex"));

    // a textbox that modifies `name`.
    let numbox = TextBox::new()
        .with_placeholder("658")
        .fix_width(70.0)
        .lens(InitState::name);

    let namebox = TextBox::new()
        .with_placeholder("Greninja")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(InitState::name);
    
    let searchbutton = Button::new("Search!");

    let inputrow = Flex::row()
        .with_child(dex)
        .with_spacer(25.0)
        .with_child(hash)
        .with_child(numbox)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(namebox)
        .with_child(searchbutton);


    let speciesl: Label<String> = Label::new(format!("Species"));

    let outputrow = Flex::row()
        .with_child(speciesl).lens(InitState::name);

    let column = Flex::column()
    .with_child(inputrow)
    .with_spacer(20.0)
    .with_child(outputrow);

    Align::centered(column)
}

pub fn build_ui() {
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .window_size((600.0, 400.0));

    let initial_state = InitState {
        name: "".into(),
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}