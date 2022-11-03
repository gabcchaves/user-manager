use yew::prelude::*;

/* Business data */
#[derive(Clone, PartialEq)]
struct User {
    id: u8,
    name: String,
}

/* UI data */
enum MsgApp {
    List,
    Register,
}
enum MsgList {
    RemoveItem,
}
struct App {
    section: u8,
}
struct Header;
struct List {
    items: Vec<User>,
}
struct Registration;

#[derive(PartialEq, Properties)]
struct HeaderProps {
    children: Children,
}
#[derive(PartialEq, Properties)]
struct ListProps {
    items: Vec<User>,
    // on_click: Callback<MsgList>,
}
#[derive(PartialEq, Properties)]
struct RegistrationProps {
    children: Children,
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
            },
            MsgApp::Register => {
                self.section = 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let items = vec![
            User {
                id: 1,
                name: "Gabriel".to_string(),
            },
        ];
        html! {
            <>
                <Header>
                    <button class="btn-list" onclick={link.callback(|_| MsgApp::List)}>{"List Users"}</button>
                    <button class="btn-add-user" onclick={link.callback(|_| MsgApp::Register)}>{"Add User"}</button>
                </Header>
                if self.section == 0 {
                    <List items={items} />
                }
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
            <header>
                { ctx.props().children.clone() }
            </header>
        }
    }
}

impl Component for List {
    type Message = MsgList;
    type Properties = ListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            items: vec![
                User {
                    id: 1,
                    name: "Gabriel".to_string(),
                },
            ],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgList::RemoveItem => {
                self.items.pop();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
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
