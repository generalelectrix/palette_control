use shared::Color as ColorType;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct ColorProps {
    pub color: ColorType,
}

#[function_component(Color)]
pub fn color(ColorProps { color }: &ColorProps) -> Html {
    let (r, g, b) = color.as_u8();
    let style = format!(
        "width:100px;height:100px;display:inline-block;margin:0px;background: rgb({},{},{})",
        r, g, b
    );
    html! {
        <div style={style} />
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct PaletteProps {
    pub colors: Vec<ColorType>,
}

#[function_component(Palette)]
pub fn palette(PaletteProps { colors }: &PaletteProps) -> Html {
    colors
        .iter()
        .map(|color| {
            html! {
                <Color color={*color} />
            }
        })
        .collect()
}
