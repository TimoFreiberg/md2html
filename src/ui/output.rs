use yew::{html, Component, ComponentLink, Properties};

pub struct Output {
    props: Props,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub text: String,
}

impl Component for Output {
    type Message = ();

    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> yew::ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        if props != self.props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> yew::Html {
        html! {
            <div class="html-output text-container">
                <label>
                    <h2>{"Generated HTML"}</h2>
                    // TODO replace readonly with disabled and introduce a copy-click-handler
                    <textarea readonly=true>
                        { &self.props.text }
                    </textarea>
                </label>
            </div>
        }
    }
}
