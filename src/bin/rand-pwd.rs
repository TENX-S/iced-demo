
use grp::*;
use iced::{
    slider, button, Column,
    Container, Element, Length,
    Sandbox, Settings, Slider, Text, Button
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

        let content = Column::new()
            .spacing(20)
            .padding(2)
            .max_width(1000)
            .push(letter)
            .push(symbol)
            .push(number)
            .push(Text::new(ltr_cnt.to_string()).size(50))
            .push(Text::new(sbl_cnt.to_string()).size(50))
            .push(Text::new(num_cnt.to_string()).size(50))
            .push(Text::new(sum.to_string()).size(50))
            .push(Text::new(self.rand_pwd.show()).size(20))
            .push(Button::new(&mut self.refresh, Text::new("Generate!")).on_press(Message::RefreshPressed));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()

    }

}

