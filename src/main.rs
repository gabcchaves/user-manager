use yew::prelude::*;

/* Messages */
enum MsgApp {
    ListUsers,
    AddUser,
}
enum MsgList {
    RemoveItem,
}

/* Components */
struct App {
    section: u8,
}
struct Header;
struct ListItem {
    id: u8,
    name: String,
}
struct List {
    items: Vec<ListItem>,
}

/* Properties */
#[derive(Properties, PartialEq)]
struct HeaderProps {
    children: Children,
}

/* Implementations */
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
            MsgApp::ListUsers => {
                self.section = 0;
                true
            },
            MsgApp::AddUser => {
                self.section = 1;
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Header>
                    <div class="logo-area">
                        <h1>{"Compraqui"}</h1>
                    </div>
                    <div class="options">
                        <button onclick={ctx.link().callback(|_| MsgApp::ListUsers)}>{"List Users"}</button>
                        <button onclick={ctx.link().callback(|_| MsgApp::AddUser)}>{"Add User"}</button>
                    </div>
                    if self.section == 0 {
                        <List />
                    } else {
                        {"HI"}
                    }
                </Header>
            </>
        }
    }
}

impl Component for Header {
    type Message = ();
    type Properties = HeaderProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            { for ctx.props().children.iter() }
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
