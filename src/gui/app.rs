use std::mem;
use std::env;
use std::path::Path;
use crate::{
    matcell::matcell::MatCell,
    csv::reader::{full_read, DEFAULT_FILE},
};
use iced::widget::{text, container, Column, Row, Button, scrollable, column};
use iced::{Application, Settings, Length, Command, Theme, Element, Alignment, alignment};

#[derive(Debug, Default, Clone)]
pub struct App {
    file: String,
    content: Vec<String>, 
    is_loading: bool,
    cols: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChange((usize, String)),
    // DragFile,
    // OpenFile,
    // FileOpened,

}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let a = env::args().collect::<Vec<String>>();
        let file = if a.len() >= 2 { a[1].clone() } else { DEFAULT_FILE.to_string() };
        let data = full_read(&file);
        (App {
            file,
            content: data.1,
            cols: data.0,
            is_loading: false,
        }, Command::none())
    }

    fn title(&self) -> String {
        "csv reader".into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::InputChange((i, val)) => self.content[i] = val,
            // Message::OpenFile => if self.is_loading {
                // Command::none()
            // } else {
        //         self.is_loading = true;
        //         Command::perform(open_file(), Message::FileOpened)
        //     },
        //     Message::FileOpened(res) => {
        //         self.is_loading = false;
        //         if let Ok((path, contents)) = res {
        //             self.file = Some(path);
        //             self.content = full_read(path);
        //         }
        //     }
        }

        Command::none()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn view(&self) -> Element<Message> {
        // let pos = self.inputs.len() - 1;
        // container(column![
        //     text("MatCell Sample"), 
        //     MatCell::new("", &self.inputs[pos - 1], pos - 1)
        //         // .width(100)
        //         // .height(80)
        //         .on_input(Message::IC),
        //     MatCell::new("", &self.inputs[pos], pos)
        //         // .width(100)
        //         // .height(500)
        //         .on_input(Message::IC),
        // ]).into()
        // file_btn().into()
        container(scrollable(column![
                        container(text(Path::new(&self.file).file_name().unwrap().to_string_lossy().to_owned())
                            .horizontal_alignment(alignment::Horizontal::Center)
                            .size(36)
                        ).padding(10)
                        .align_x(alignment::Horizontal::Center),

                        container(self.table())
                            .width(Length::Fill)
                            .align_x(alignment::Horizontal::Center)
                            .padding(20),
                    ]
                )
                  .width(Length::Fill)
                  .height(Length::Fill)
            ).width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl App {
    fn table(&self) -> Column<Message> {
        let mut vec: Vec<_> = Vec::new();
        let rows = self.content.len() / 9;
        (0..rows).map(|n| {
            let mut row = Vec::new();
            (0..self.cols).map(|m| {
                let index = self.cols * n + m;
                row.push(MatCell::new("", &self.content[index], index).on_input(Message::InputChange).into()); 
            }).collect::<()>();
            vec.push(Row::with_children(row).into());
        }).collect::<()>();
        // let mut cells = self.content.iter();
        // let mut index = 0;
        // while let Some(cell) = cells.next() {
        //     if index % 9 == 0 {
        //         vec.push(col.into());
        //         // let mut col: Column<'_, Message, iced::Renderer> = Column::new();
        //         col = Column::new();
        //     }
        //     col.push(MatCell::new("", &self.content[index], index).on_input(Message::InputChange));
        //     index += 1;
        // }

        Column::with_children(vec)
    }
}

fn file_btn() -> Button<'static, Message> {
    Button::new("Select a CSV file to read.")
        // .on_press(Message::OpenFile)
}



