use yew::prelude::*;

/* Messages */
enum MsgApp {
    List,
}
enum MsgList {
    RemoveItem,
}

/* Components */
struct App {
    section: u8,
}
struct ListItem {
    id: u8,
    name: String,
}
struct List {
    items: Vec<ListItem>,
}

impl Component for App {
    type Message = MsgApp;
    type Properties = ();
    
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            section: 0
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgApp::List => {
                self.section = 0;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <List />
        }
    }
}

impl Component for List {
    type Message = MsgList;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            // items: Vec::<ListItem>::new()
            items: vec![
                ListItem {
                    id: 1,
                    name: "Gabriel".to_string(),
                },
            ]
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgList::RemoveItem => {
                self.items.pop();
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        self.items.iter().map(|item| {
            html! {
                <p>{format!("{} - {}", item.id, item.name)}</p>
            }
        }).collect()
    }
}

fn main() {
    yew::start_app::<App>();
}
