#![windows_subsystem = "windows"]
extern crate native_windows_gui as nwg;

use std::fs::File;
use std::io::Write;
use std::ops::Add;

use chrono::Duration;
use chrono::prelude::*;
use clipboard_win::{formats, set_clipboard};
use nwg::NativeUi;
use serde_json::to_string;

use crate::register::Register;
use crate::rsa_util::{decrypt, encrypt, generate_pem};


pub mod rsa_util;
pub mod register;

#[derive(Default)]
pub struct BasicApp {
    window: nwg::Window,
    layout: nwg::GridLayout,
    input_machine: nwg::TextInput,
    input_type:nwg::ComboBox<&'static str>,
    input_order: nwg::TextInput,
    input_encode: nwg::TextInput,
    button_generate: nwg::Button,
    button_decode: nwg::Button
}

impl BasicApp {

    fn generate(&self) {
        // generate_pem();
        let result = self.generate_validate();
        match result {
            Ok(register) => {
                // println!("register:{:?}",register);
                let register_json = serde_json::to_string(&register).unwrap();
                let json_file = File::create("./register.json").unwrap();
                serde_json::to_writer(json_file,&register);
                println!("register_json:{}",register_json);
                let register_code = encrypt(register_json.as_str());
                // {"machine_id":["1"],"expire_date":1653467064564,"expire_string":"2022-05-25 16:24:24","sign_date":1652862264564,"sign_string":"2022-05-18 16:24:24","order_id":""}
                println!("register_code:{}",register_code);
                let mut register_file = File::create("./register.ini").unwrap();
                let bytes = register_code.as_bytes();
                register_file.write_all(bytes);
                set_clipboard(formats::Unicode, register_code.as_str()).expect("To set clipboard");
                nwg::modal_info_message(&self.window, "成功,已复制注册码", register_code.as_str());
            }
            Err(e) => {
                nwg::modal_info_message(&self.window, "警告", &format!("{}", e));
            }
        }
    }
    // %Y-%m-%d %H:%M:%S
    fn generate_validate(&self) -> Result<Register,&'static str>{
        let machine = self.input_machine.text().trim().to_string();
        // println!("machine:{}",machine);
        if machine.is_empty() {
            return Err("机器码不能为空");
        }
        let card_type = self.input_type.selection().unwrap();
        // println!("select:{}",card_type);
        let order = self.input_order.text().trim().to_string();
        println!("order:{}",order);
        let now = Local::now();
        // let test = now.format("%Y-%m-%d %H:%M:%S");
        // println!("test:{}",test);
        // let sign_date = now.timestamp_millis();
        // let sign_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
        let expire_time = match card_type {
            0 => {
                now.add(Duration::days(1))
            }
            1 => {
                 now.add(Duration::days(8))
            }
            2 => {
                now.add(Duration::days(31))
            }
            3 => {
                now.add(Duration::days(93))
            }
            4 => {
                now.add(Duration::days(183))
            }
            5 => {
                now.add(Duration::days(368))
            }
            6 => {
                now.add(Duration::days(36600))
            }
            _ => now.add(Duration::days(1))
        };
        return Ok(Register{
            machine_id:machine.split(" ").map(|part|{part.to_string()}).collect(),
            expire_date: expire_time.timestamp_millis(),
            expire_string: expire_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            sign_date: now.timestamp_millis(),
            sign_string: now.format("%Y-%m-%d %H:%M:%S").to_string(),
            order_id: order
        });
    }

    fn decode(&self) {
        let register_code = self.input_encode.text().trim().to_string();
        if register_code.is_empty(){
            nwg::modal_info_message(&self.window, "警告", "注册码不能为空");
            return;
        }
        else {
            let register_json = decrypt(&register_code);
            println!("register_json_str:{}", register_json);
            File::create("./decode_register.json").unwrap().write_all(register_json.as_bytes());
            nwg::modal_info_message(&self.window, "成功", register_json.as_str());
        }
    }

}

mod basic_app_ui {
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::process::exit;
    use std::rc::Rc;

    use native_windows_gui as nwg;

    use super::*;

    pub struct BasicAppUi {
        inner: Rc<BasicApp>,
        default_handler: RefCell<Option<nwg::EventHandler>>
    }

    impl nwg::NativeUi<BasicAppUi> for BasicApp {
        fn build_ui(mut data: BasicApp) -> Result<BasicAppUi, nwg::NwgError> {
            use nwg::Event as E;

            // Controls
            nwg::Window::builder()
                .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
                .size((400, 300))
                .position((800, 400))
                .title("人机助手注册机")
                .build(&mut data.window)?;

            nwg::TextInput::builder()
                .text("")
                .parent(&data.window)
                .focus(true)
                .placeholder_text(Some("多个机器码用空格分割"))
                .build(&mut data.input_machine)?;

            nwg::ComboBox::builder()
                .parent(&data.window)
                .collection(vec!["天卡","周卡","月卡","季卡","半年","年卡","永久"])
                .selected_index(Some(0))
                .build(&mut data.input_type)?;

            nwg::TextInput::builder()
                .text("")
                .parent(&data.window)
                .focus(false)
                .placeholder_text(Some("订单号(可以不填)"))
                .build(&mut data.input_order)?;

            nwg::TextInput::builder()
                .text("")
                .placeholder_text(Some("注册码"))
                .parent(&data.window)
                .focus(false)
                .build(&mut data.input_encode)?;

            nwg::Button::builder()
                .text("生成注册码")
                .parent(&data.window)
                .size((100,50))
                .build(&mut data.button_generate)?;
            nwg::Button::builder()
                .text("解密注册码")
                .parent(&data.window)
                .build(&mut data.button_decode)?;

            // Wrap-up
            let ui = BasicAppUi {
                inner: Rc::new(data),
                default_handler: Default::default(),
            };

            // Events
            let evt_ui = Rc::downgrade(&ui.inner);
            let handle_events = move |evt, _evt_data, handle| {
                if let Some(ui) = evt_ui.upgrade() {
                    match evt {
                        E::OnButtonClick =>
                            if &handle == &ui.button_generate {
                                BasicApp::generate(&ui);
                            }
                            else if &handle == &ui.button_decode {
                                BasicApp::decode(&ui);
                            },
                        E::OnWindowClose =>
                            if &handle == &ui.window {
                                exit(0);
                            },
                        _ => {}
                    }
                }
            };

            *ui.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(&ui.window.handle, handle_events));

            // Layouts
            nwg::GridLayout::builder()
                .parent(&ui.window)
                .spacing(0)
                .margin([10,10,10,10])
                .child_item(nwg::GridLayoutItem::new(&ui.input_machine, 0, 0, 2, 1))
                .child_item(nwg::GridLayoutItem::new(&ui.input_type, 0, 1, 2, 1))
                .child_item(nwg::GridLayoutItem::new(&ui.input_order, 0, 2, 2, 1))
                .child_item(nwg::GridLayoutItem::new(&ui.input_encode, 0, 3, 2, 1))
                .child_item(nwg::GridLayoutItem::new(&ui.button_generate, 0, 4, 2, 1))
                .child_item(nwg::GridLayoutItem::new(&ui.button_decode, 0, 5, 2, 1))
                .build(&ui.layout)?;

            return Ok(ui);
        }
    }

    impl Drop for BasicAppUi {
        /// To make sure that everything is freed without issues, the default handler must be unbound.
        fn drop(&mut self) {
            let handler = self.default_handler.borrow();
            if handler.is_some() {
                nwg::unbind_event_handler(handler.as_ref().unwrap());
            }
        }
    }

    impl Deref for BasicAppUi {
        type Target = BasicApp;

        fn deref(&self) -> &BasicApp {
            &self.inner
        }
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _ui = BasicApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}