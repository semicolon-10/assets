use yew::{Callback, Component, Context, Html, html};
use crate::props::Props;

pub struct Card {
    props: Props
}

impl Component for Card {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props : ctx.props().clone()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            image_url,
            title,
            published_at,
            author,
            description,
            url,
        } = &self.props;

        let onclick = {
            let url = url.clone();
            Callback::from(move |_| {
             web_sys::window().unwrap().location().set_href(&url).unwrap();
            })
        };


        html! {
            <div class="card" {onclick} style="display: flex; border: 1px solid #ccc; margin: 10px; padding: 10px; cursor: pointer;">
                <img src={image_url.clone()} alt={title.clone()} style="width: 150px; height: 150px; object-fit: cover; margin-right: 10px;" />
                <div class="container" style="flex: 1;">
                    <h4 style="margin: 0 0 10px 0;"><b>{title.clone()}</b></h4>
                    <p style="margin: 0 0 10px 0;">{description.clone()}</p>
                    <p style="margin: 0 0 5px 0; color: gray;">{format!("Author: {}", author)}</p>
                    <p style="margin: 0; color: gray;">{format!("Published at: {}", published_at)}</p>
                </div>
            </div>
        }
    }
}
