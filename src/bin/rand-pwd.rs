
use grp::*;
use iced::{
    slider, Slider,
    button, Button,
    Container, Element, Settings, Align,
    Text, Column, Length, Sandbox, Row, HorizontalAlignment
};



fn main() {
    RandPwdWin::run(Settings::default())
}


#[derive(Default)]
struct RandPwdWin {

    letter: slider::State, ltr_cnt: u16,
    symbol: slider::State, sbl_cnt: u16,
    number: slider::State, num_cnt: u16,

    refresh: button::State,

    rand_pwd: RandPwd,
}



#[derive(Debug, Clone)]
enum Message {

    LetterChanged(u16),
    SymbolChanged(u16),
    NumberChanged(u16),
    RefreshPressed,

}

impl Sandbox for RandPwdWin {
    type Message = Message;

    fn new() -> Self {
        RandPwdWin::default()
    }

    fn title(&self) -> String {
        String::from("Random Password Generator")
    }

    fn update(&mut self, message: Message) {

        match message {
            Message::LetterChanged(value) => {
                self.ltr_cnt = value;
                self.rand_pwd.set_cnt("ltr", value);
            },
            Message::SymbolChanged(value) => {
                self.sbl_cnt = value;
                self.rand_pwd.set_cnt("sbl", value);
            },
            Message::NumberChanged(value) => {
                self.num_cnt = value;
                self.rand_pwd.set_cnt("num", value);
            },
            Message::RefreshPressed => {
                self.rand_pwd.join();
            }
        }

    }

    fn view(&mut self) -> Element<Message> {

        let letter = Slider::new(
            &mut self.letter,
            0..=100,
            self.ltr_cnt,
            Message::LetterChanged,
        );

        let symbol = Slider::new(
            &mut self.symbol,
            0..=100,
            self.sbl_cnt,
            Message::SymbolChanged,
        );

        let number = Slider::new(
            &mut self.number,
            0..=100,
            self.num_cnt,
            Message::NumberChanged,
        );

        let ltr_cnt = self.ltr_cnt;
        let sbl_cnt = self.sbl_cnt;
        let num_cnt = self.num_cnt;
        let sum = ltr_cnt + sbl_cnt + num_cnt;

        let pane_factory = |text, line|
            Row::new()
                .spacing(50)
                .max_height(50)
                .push(text)
                .push(line);



        let selection_pane =
            Column::new()
                .spacing(20)
                .padding(2)
                .max_width(1000)
                .align_items(Align::Center)
                .push(pane_factory(
                    Text::new(format!("Letters: {:>5}", ltr_cnt))
                        .width(Length::Units(120))
                        .size(20),
                    letter)
                )
                .push(pane_factory(
                    Text::new(format!("Symbols: {:>2}", sbl_cnt))
                        .width(Length::Units(120))
                        .size(20),
                    symbol)
                )
                .push(pane_factory(
                    Text::new(format!("Numbers: {}", num_cnt))
                        .width(Length::Units(120))
                        .size(20),
                    number)
                )
                .push(Text::new(sum.to_string()).size(50))
                .push(Text::new(self.rand_pwd.show()).size(20))
                .push(Button::new(
                    &mut self.refresh,
                    Text::new("Generate!")
                        .width(Length::Fill)
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .size(16)
                )
                    .width(Length::Units(200))
                    .padding(10)
                    .on_press(Message::RefreshPressed)
                );

        Container::new(selection_pane)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()

    }

}

