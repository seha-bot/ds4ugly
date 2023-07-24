use gilrs_core::{Event, EventType, Gilrs};
use std::collections::HashSet;

fn map_code(code: u32) -> Button {
    return match code {
        0 => Button::SQUARE,
        1 => Button::X,
        2 => Button::CIRCLE,
        3 => Button::TRIANGLE,
        4294967295 => Button::LEFT,
        4294967294 => Button::DOWN,
        4294967293 => Button::RIGHT,
        4294967292 => Button::UP,
        4 => Button::L1,
        5 => Button::R1,
        8 => Button::SHARE,
        9 => Button::OPTIONS,
        10 => Button::L3,
        11 => Button::R3,
        _ => Button::NONE,
    };
}

fn main() {
    let mut gilrs = Gilrs::new().expect("Failed to start :/");
    println!("Waiting for gamepad...");

    let mut emulator = None;
    let mut emulator_controller = vigem_client::DS4Report::default();

    let mut pressed_buttons = HashSet::<Button>::new();

    while let Some(Event { id, event, time: _ }) = gilrs.next_event_blocking(None) {
        if id != 0 {
            continue;
        }
        match event {
            EventType::Connected => {
                println!("Gamepad found: {}", gilrs.gamepad(0).unwrap().name());
                let client = vigem_client::Client::connect().unwrap();
                let id = vigem_client::TargetId::DUALSHOCK4_WIRED;
                let mut target = vigem_client::DualShock4Wired::new(client, id);
                target.plugin().unwrap();
                target.wait_ready().unwrap();
                emulator = Some(target);
            }
            EventType::ButtonPressed(button) => {
                pressed_buttons.insert(map_code(button.into_u32()));
            }
            EventType::ButtonReleased(button) => {
                pressed_buttons.remove(&map_code(button.into_u32()));
            }
            EventType::AxisValueChanged(data, axis) => {
                let data_trigger = (data >> 7) as u8;
                let data_stick = data as u8;
                match axis.into_u32() {
                    65539 => emulator_controller.trigger_l = data_trigger,
                    65540 => emulator_controller.trigger_r = data_trigger,
                    65536 => emulator_controller.thumb_lx = data_stick,
                    65537 => emulator_controller.thumb_ly = data_stick,
                    65538 => emulator_controller.thumb_rx = data_stick,
                    65541 => emulator_controller.thumb_ry = data_stick,
                    _ => (),
                }
            }
            EventType::Disconnected => {
                println!("Gamepad disconnected!");
                return;
            }
        }

        emulator_controller.buttons = assemble_code(&mut pressed_buttons);
        let _ = emulator.as_mut().unwrap().update(&emulator_controller);
    }
}

fn assemble_code(buttons: &mut HashSet<Button>) -> u16 {
    let mut shapes = [0, 0, 0, 0];
    let mut arrows = [0, 0];
    let mut other = 0;

    for &e in buttons.iter() {
        if e.is_shape() {
            *shapes.iter_mut().find(|e| **e == 0).unwrap() = e as u16;
        } else if e.is_direction() {
            *arrows.iter_mut().find(|e| **e == 0).unwrap() = match e {
                Button::LEFT => 1,
                Button::DOWN => 2,
                Button::RIGHT => 4,
                Button::UP => 8,
                _ => 0,
            };
        } else {
            other += match e {
                Button::L1 => 256,
                Button::R1 => 512,
                Button::SHARE => 6144,
                Button::OPTIONS => 9216,
                Button::L3 => 18432,
                Button::R3 => 33792,
                _ => 0,
            };
        }
    }

    8 + shapes.iter().sum::<u16>() * 16
        - match arrows.iter().sum::<u16>() {
            9 => 1,
            1 => 2,
            3 => 3,
            2 => 4,
            6 => 5,
            4 => 6,
            12 => 7,
            8 => 8,
            _ => 0,
        }
        + other
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
enum Button {
    NONE = 0,
    SQUARE = 1,
    X = 2,
    CIRCLE = 4,
    TRIANGLE = 8,
    LEFT,
    DOWN,
    RIGHT,
    UP,
    L1,
    R1,
    SHARE,
    OPTIONS,
    L3,
    R3,
}

impl Button {
    fn is_shape(&self) -> bool {
        matches!(self, Self::SQUARE | Self::X | Self::CIRCLE | Self::TRIANGLE)
    }

    fn is_direction(&self) -> bool {
        matches!(self, Self::LEFT | Self::DOWN | Self::RIGHT | Self::UP)
    }
}
