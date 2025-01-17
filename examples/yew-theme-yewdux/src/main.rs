use stylist::yew::Global;
use stylist::{StyleSource, YieldStyle};
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yewdux::prelude::*;
use yewtil::NeqAssign;

use log::Level;

mod store;

use store::theme::ThemeKind;
use store::{Action, AppDispatch};

pub(crate) struct BaseInside {
    dispatch: AppDispatch,
}

impl Component for BaseInside {
    type Message = ();
    type Properties = AppDispatch;

    fn create(dispatch: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { dispatch }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, dispatch: Self::Properties) -> ShouldRender {
        self.dispatch.neq_assign(dispatch)
    }

    fn view(&self) -> Html {
        let theme_str = match self.dispatch.state().theme.kind {
            ThemeKind::Light => "Dark Theme",
            ThemeKind::Dark => "Light Theme",
        };

        let other_theme = match self.dispatch.state().theme.kind {
            ThemeKind::Light => ThemeKind::Dark,
            ThemeKind::Dark => ThemeKind::Light,
        };

        let switch_theme = self
            .dispatch
            .callback(move |_| Action::SetTheme(other_theme.clone()));

        html! {
            <div class=self.style()>
                <button onclick=switch_theme id="yew-sample-button">{"Switch to "}{theme_str}</button>
            </div>
        }
    }
}

impl YieldStyle for BaseInside {
    fn style_from(&self) -> StyleSource<'static> {
        r#"
            button {
                color: white;
                height: 50px;
                width: 300px;
                font-size: 20px;
                background-color: rgb(88, 164, 255);
                border-radius: 5px;
                border: none;
            }
        "#
        .into()
    }
}

pub(crate) type Inside = WithDispatch<BaseInside>;

pub(crate) struct App {
    dispatch: AppDispatch,
}

impl Component for App {
    type Message = ();
    type Properties = AppDispatch;

    fn create(dispatch: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { dispatch }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, dispatch: Self::Properties) -> ShouldRender {
        self.dispatch.neq_assign(dispatch)
    }

    fn view(&self) -> Html {
        let theme = self.dispatch.state().theme.clone();

        let theme_str = match theme.kind {
            ThemeKind::Light => "light theme",
            ThemeKind::Dark => "dark theme",
        };

        html! {
            <>
                // Global Styles can be applied with <Global /> component.
                <Global css=format!(
                    r#"
                        html, body {{
                            font-family: sans-serif;

                            padding: 0;
                            margin: 0;

                            display: flex;
                            justify-content: center;
                            align-items: center;
                            min-height: 100vh;
                            flex-direction: column;

                            background-color: {bg};
                            color: {ft_color};
                        }}
                    "#,
                    bg = theme.current().background_color,
                    ft_color = theme.current().font_color,
                ) />
                <h1>{"Yew Theming w/ Yewdux"}</h1>
                <div class=self.style() id="yew-sample-content">
                    {"You are now using the "}{theme_str}{"!"}
                    <Inside />
                </div>
            </>
        }
    }
}

impl YieldStyle for App {
    fn style_from(&self) -> StyleSource<'static> {
        let theme = self.dispatch.state().theme.current();

        format!(
            r#"
                box-shadow: 0 0 5px 1px rgba(0, 0, 0, 0.7);
                height: 500px;
                width: 500px;
                border-radius: 5px;

                display: flex;
                justify-content: space-around;
                align-items: center;

                padding: 15px;
                box-sizing: border-box;

                flex-direction: column;
                background-color: {bg};
            "#,
            bg = theme.paper_color
        )
        .into()
    }
}

fn main() {
    console_log::init_with_level(Level::Trace).expect("Failed to initialise Log!");
    yew::start_app::<WithDispatch<App>>();
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    use web_sys::window;

    #[wasm_bindgen_test]
    fn test_simple() {
        yew::app::App::<WithDispatch<App>>::new()
            .mount(yew::utils::document().get_element_by_id("output").unwrap());
        let window = window().unwrap();
        let doc = window.document().unwrap();
        let body = window.document().unwrap().body().unwrap();

        let content = doc.query_selector("#yew-sample-content").unwrap().unwrap();

        let body_style = window.get_computed_style(&body).unwrap().unwrap();
        let content_style = window.get_computed_style(&content).unwrap().unwrap();

        let bg_color = body_style.get_property_value("background-color").unwrap();
        assert_eq!(bg_color, "rgb(237, 244, 255)");

        let content_display = content_style.get_property_value("display").unwrap();
        assert_eq!(content_display, "flex");

        let button = doc
            .query_selector("#yew-sample-button")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();
        button.click();

        let bg_color = body_style.get_property_value("background-color").unwrap();
        assert_eq!(bg_color, "rgb(0, 0, 0)");
    }
}
