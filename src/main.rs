use iced::widget::{container, text, Column};
use iced::{alignment, Element, Length, Task, Task as Command};


#[derive(Default, Debug, Clone)]
struct GroceryList {
    grocery_items: Vec<String>
}


#[derive(Debug, Clone)]
struct Message {

}

impl GroceryList {
    fn new() -> (Self, Command<Message>) {
        (
            Self {
                grocery_items: vec!["Eggs".to_owned(), "Milk".to_owned(), "Flour".to_owned()]
            },
            Command::none(),
        )
    }

    fn view(&self) -> Element<Message> {
        container(items_list_view(&self.grocery_items))
            .height(Length::Fill)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        let _ = message;
        Task::none()
	}
}

fn items_list_view(items: &Vec<String>) -> Element<'_, Message> {
    let mut column = Column::new()
        .spacing(20)
        .align_x(iced::Alignment::Center)
        .width(Length::Fill);

    for value in items {
        column = column.push(text(value));
    }

    container(column).height(250.0).width(300).into()
}

pub fn main() -> iced::Result {
    iced::application("A cool application", GroceryList::update, GroceryList::view).run_with(GroceryList::new)
}
