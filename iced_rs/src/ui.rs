use iced::application::StyleSheet;
use iced::keyboard::Event as KeyboardEvent;
use iced::theme::{Container as ContainerTheme, Theme};
use iced::widget::container::Appearance;
use iced::widget::image::Handle;
use iced::widget::{button, column, container, image, row, text, Container, Image};
use iced::{executor, Application, Color, Command, Element, Length, Subscription};
use iced_native::futures::channel::mpsc;
use iced_native::{subscription, Event};
use rand::Rng;

use std::cmp::min;
use std::fs;
use std::path::PathBuf;

pub struct KeyTuber {
    theme: Theme,
    images: Vec<image::Handle>,
    current_image: image::Handle,
    index: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    KeyPressed,
    TestEvent,
    EventOccurred(Event),
    ImageLoaded(image::Handle),
}

pub enum TestEvent {
    Ready(mpsc::Sender<Message>),
    WorkFinished,
}

pub enum TestState {
    Starting,
    Ready(mpsc::Receiver<Message>),
}

const APP_PADDING: u16 = 20;

fn rand(max: usize) -> usize {
    return rand::thread_rng().gen_range(0..max);
}

async fn loadimage(path: PathBuf) -> image::Handle {
    //println!("did this run");
    return image::Handle::from_path(path);
}

impl Application for KeyTuber {
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;
    type Message = Message;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let files = match fs::read_dir("../images") {
            Ok(paths) => {
                //how do i turn this into an vec
                let mut path_vec: Vec<PathBuf> = vec![];

                for path in paths {
                    //println!("{:?}", path);
                    path_vec.push(path.unwrap().path());
                }

                path_vec
            }
            Err(err) => {
                //println!("{:?}", err);
                vec![]
            }
        };

        //TODO filter images from files

        //TODO listen to keyboard events from outside active window
        return (
            KeyTuber {
                theme: Theme::Dark,
                images: vec![],
                current_image: image::Handle::from_path(""),
                index: 0,
            },
            Command::batch((0..min(15, files.len())).into_iter().fold(
                Vec::with_capacity(15),
                |mut v, i| {
                    let file = files[i].clone();
                    v.push(Command::perform(loadimage(file), Message::ImageLoaded));
                    v
                },
            )),
        );
    }

    fn title(&self) -> String {
        return String::from("KeyTuber");
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        //println!("test");
        match message {
            Message::EventOccurred(event) => match event {
                Event::Keyboard(e) => {
                    //println!("Key event {:?}", e);
                    match e {
                        KeyboardEvent::KeyPressed {
                            key_code,
                            modifiers,
                        } => {
                            println!("{:?}{:?}", key_code, modifiers);
                            self.current_image = self.images[rand(self.images.len())].clone();
                        }
                        KeyboardEvent::KeyReleased {
                            key_code,
                            modifiers,
                        } => {
                            self.current_image = self.images[0].clone();
                        }
                        _ => {}
                    }
                }
                _ => {}
            },
            Message::ImageLoaded(image_handle) => {
                self.images.push(image_handle);
            }
            _ => {}
        }

        return Command::none();
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        return subscription::events().map(Message::EventOccurred);
        //subscription::unfold(
        //    std::any::TypeId::of::<Message>(),
        //    TestState::Starting,
        //    |state| async move {
        //        match state {
        //            TestState::Starting => {
        //                let (sender, receiver) = mpsc::channel(100);
        //                (Some(TestEvent::Ready(sender)), TestState::Ready(receiver))
        //            }
        //            TestState::Ready(mut receiver) => {
        //                use iced_native::futures::StreamExt;

        //                let input = receiver.select_next_some().await;

        //                match input {
        //                    Message::KeyPressed => {
        //                        (Some(TestEvent::WorkFinished), TestState::Ready(receiver))
        //                    }
        //                    _ => (Some(TestEvent::WorkFinished), TestState::Ready(receiver)),
        //                }
        //            }
        //        }
        //    },
        //)
        //.map(Message::TestEvent)
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        //println!("image handles: {:?}", self.images);
        let img = image::viewer(self.current_image.clone())
            .width(Length::Fill)
            .height(Length::Fill);

        return container(img)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            //bruuhhh
            //.style(ContainerTheme::Custom(Box::new(container::Appearance {
            //    background: Color::from_rgb8(50, 50, 50).into(),
            //    ..container::Appearance::default()
            //})))
            .into();
    }
}
