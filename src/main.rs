mod helpers;

use iced::widget::pane_grid::Content;
use iced::{Element, Sandbox, Settings};
use iced::widget::{column, container, text_editor, Column, Row};


const EMPTY_REGEX: &str = "Enter your regex here";
const EMPTY_TEXT: &str = "Enter your text here";
const EMPTY_RESULT: &str = "Result will appear here";


fn main() -> iced::Result {
    Lab::run(Settings::default())
}


struct Lab {
    current_regex: text_editor::Content,
    current_text: text_editor::Content,
    current_result: text_editor::Content,
}


#[derive(Clone, Debug)]
enum Message {
    RegexInputAnyActionPerform(text_editor::Action),
    TextInputAnyActionPerform(text_editor::Action),
    ResultActionPerform(text_editor::Action),
}


impl Sandbox for Lab {
    type Message = Message;

    fn new() -> Lab {
        Lab {
            current_regex: text_editor::Content::with_text(EMPTY_REGEX),
            current_text: text_editor::Content::with_text(EMPTY_TEXT),
            current_result: text_editor::Content::with_text(EMPTY_RESULT),
        }
    }

    fn title(&self) -> String {
        String::from("RegEx-Lab")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::RegexInputAnyActionPerform(action) => {
                self.current_regex.perform(action);
                let result = helpers::search(&self.current_regex.text(), &self.current_text.text());
                let output = helpers::format_output(result);
                self.current_result = text_editor::Content::with_text(&output);
                },
            Message::TextInputAnyActionPerform(action) => {
                self.current_text.perform(action);
                let result = helpers::search(&self.current_regex.text(), &self.current_text.text());
                let output = helpers::format_output(result);
                self.current_result = text_editor::Content::with_text(&output);
            },
            Message::ResultActionPerform(action) => self.current_result.perform(action),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        // Top row
        let regex_input = text_editor(&self.current_regex)
            .on_action(Message::RegexInputAnyActionPerform);

        let top_row = Row::new()
            .padding(3)
            .push(regex_input);

        // Mid row
        let result_output = text_editor(&self.current_result)
            .on_action(Message::ResultActionPerform);

        let mid_row = Row::new()
            .padding(3)
            .push(result_output);
        

        // Bot row
        let text_input = text_editor(&self.current_text)
            .on_action(Message::TextInputAnyActionPerform);

        let bottom_row = Row::new()
            .padding(3)
            .push(text_input);


        // Main composition
        let main_column = Column::new()
            .push(top_row)
            .push(mid_row)
            .push(bottom_row);

        main_column.into()

    }
}
