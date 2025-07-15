use esp_idf_hal::gpio::Gpio5;

use crate::{
    app::ChatApp,
    graphics::primitives::GraphicsPrimitives,
    peripherals::{qmi8658::motion_detector::MotionState, st77916::lcd::LcdController},
};

#[derive(Debug)]
pub enum EventMessage {
    Motion(MotionState),
}

pub struct DisplayActor {
    app: ChatApp<'static>,
    receiver: std::sync::mpsc::Receiver<EventMessage>,
}

impl DisplayActor {
    pub fn new(bl_io: Gpio5, rx: std::sync::mpsc::Receiver<EventMessage>) -> anyhow::Result<Self> {
        let lcd = LcdController::new(bl_io)?;

        let static_lcd = Box::leak(Box::new(lcd));

        let graphics = GraphicsPrimitives::new(static_lcd);

        Ok(DisplayActor {
            app: ChatApp::new(graphics),
            receiver: rx,
        })
    }

    pub fn handle_event(&mut self, event: EventMessage) -> anyhow::Result<()> {
        match event {
            EventMessage::Motion(motion_state) => match motion_state {
                MotionState::Shaking => {
                    self.app.enter_dizziness()?;
                }
                MotionState::Still => {
                    self.app.back()?;
                }
                MotionState::Tilting => self.app.enter_tilting()?,
            },
        }

        self.app.update()?;
        Ok(())
    }
}

pub struct DisplayActorManager {
    sender: std::sync::mpsc::Sender<EventMessage>,
}

impl DisplayActorManager {
    pub fn new(bl_io: Gpio5) -> Self {
        let (sender, receiver) = std::sync::mpsc::channel::<EventMessage>();

        std::thread::Builder::new()
            .stack_size(32 * 1024) // 32KB stack size for WiFi compatibility
            .spawn(move || {
                let mut app_actor = DisplayActor::new(bl_io, receiver).unwrap();

                while let Ok(event) = app_actor.receiver.recv() {
                    app_actor.handle_event(event).unwrap();
                }
            })
            .expect("Failed to spawn display actor thread");

        DisplayActorManager { sender }
    }

    pub fn on_motion(&self, motion_state: MotionState) -> anyhow::Result<()> {
        self.sender.send(EventMessage::Motion(motion_state))?;
        Ok(())
    }
}
