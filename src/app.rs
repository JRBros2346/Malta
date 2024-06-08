use super::db::*;
use executor::Default;
use iced::*;
use std::borrow::{Borrow, Cow};

pub enum Page<'a> {
    ProjectList {
        projects: Vec<Cow<'a, str>>,
        textbox: Cow<'a, str>,
    },
    EmployeeList {
        employees: Vec<Cow<'a, str>>,
        textbox: Cow<'a, str>,
    },
    ToolList {
        tools: Vec<Cow<'a, str>>,
        textbox: Cow<'a, str>,
    },
    Project(Cow<'a, str>),
    Employee(Cow<'a, str>),
    Tool(Cow<'a, str>),
}
pub enum State<'a> {
    Default {
        companies: Vec<Cow<'a, str>>,
        textbox: Cow<'a, str>,
    },
    Company {
        name: Cow<'a, str>,
        page: Page<'a>,
    },
}
pub struct App<'a> {
    states: Vec<State<'a>>,
    index: usize,
    db: Option<Database>,
}

#[derive(Debug, Clone)]
pub enum Message<'a> {
    Forward,
    Backward,
    TextChange(Cow<'a, str>),
    NewCompany(Cow<'a, str>),
}

impl<'a> Application for App<'a> {
    type Executor = executor::Default;
    type Message = Message<'a>;
    type Theme = theme::Theme;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                states: vec![State::Default(Cow::Borrowed(""))],
                index: 0,
                db: None,
            },
            Command::none(),
        )
    }
    fn title(&self) -> String {
        String::from("Malta")
    }
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Forward => {
                if self.index < self.states.len() - 1 {
                    self.index += 1;
                }
            }
            Message::Backward => {
                if self.index != 0 {
                    self.index -= 1;
                }
            }
            Message::TextChange(text) => match &mut self.states[self.index] {
                State::Default { textbox, .. } => {
                    *textbox = text;
                }
                State::Company {
                    page: Page::ProjectList { textbox, .. },
                    ..
                } => {
                    *textbox = text;
                }
                State::Company {
                    page: Page::EmployeeList { textbox, .. },
                    ..
                } => {
                    *textbox = text;
                }
                State::Company {
                    page: Page::ToolList { textbox, .. },
                    ..
                } => {
                    *textbox = text;
                }
                _ => {}
            },
            Message::NewCompany(com) => {
                self.index += 1;
                self.states
                    .resize_with(self.index + 1, || State::Default(Cow::Borrowed("")));
                self.states[self.index] = State::Company {
                    name: com.clone(),
                    page: Page::ProjectList {
                        projects: Vec::new(),
                        textbox: Cow::Borrowed(""),
                    },
                };
                self.db = Some(Database::create(&com).unwrap());
            }
            _ => {}
        }
        Command::none()
    }
    fn view(&self) -> Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        match &self.states[self.index] {
            State::Default(textbox) => widget::text_input("Enter New Company Name...", &textbox)
                .on_input(|com| Message::TextChange(Cow::Owned(com)))
                .on_submit(Message::NewCompany(textbox.to_owned()))
                .into(),
            _ => "WIP".into(),
        }
    }

    fn theme(&self) -> Self::Theme {
        Self::Theme::Dracula
    }
}
