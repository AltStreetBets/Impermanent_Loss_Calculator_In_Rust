#![windows_subsystem = "windows"]
use fltk::{app, prelude::*, window, input::{self, FloatInput}, group, frame, enums::CallbackTrigger, utils::filename_expand};



fn main() {
    

    let app = app::App::default();
    let mut win = window::Window::default()
        .with_size(750, 500)
        .center_screen()
        .with_label("Impermanent Loss Calculator");


    let mut pack = group::Pack::default()
        .with_size(200, 100)
        .with_pos(150, 100);
    let frame1 = frame::Frame::new(0, 0, 0, 50, "Token symbol");
    let mut tokenname1 = input::Input::new(0, 0, 200, 20, "");
    let frame2 = frame::Frame::new(0, 0, 0, 50, "Initial token amount");
    let mut price_a = input::FloatInput::new(0, 0, 200, 20, "");
    let frame_1 = frame::Frame::new(0, 0, 0, 50, "Final token amount");
    let mut price_af = input::FloatInput::new(0, 0, 200, 20, "");

    pack.end();

    
    let mut pack2 = group::Pack::default()
        .with_size(200, 100)
        .right_of(&pack, 50);
    let frame3 = frame::Frame::new(0, 0, 0, 50, "Token symbol");
    let mut tokenname2 = input::Input::new(0, 0, 200, 20, "");
    let frame4 = frame::Frame::new(0, 0, 0, 50, "Initial token amount");
    let mut price_b = input::FloatInput::new(0, 0, 200, 20, "");
    let frame_2 = frame::Frame::new(0, 0, 0, 50, "Final token amount");
    let mut price_bf = input::FloatInput::new(0, 0, 200, 20, "");

    pack2.end();

    


    let mut frame5 = frame::Frame::default()
        .with_size(400, 100)
        .with_label("Impermanent loss is: 6.90%")
        .below_of(&pack, 100);
    
    


    win.make_resizable(true);
    win.end();
    win.show();

    price_a.set_value(&format!("{}", 1.0));
    price_b.set_value(&format!("{}", 1.0));
    price_af.set_value(&format!("{}", 2.14));
    price_bf.set_value(&format!("{}", 0.9955));
    tokenname1.set_value(&format!{"i.e. PEE"});
    tokenname2.set_value(&format!{"i.e. BAN"});

    price_a.set_trigger(CallbackTrigger::Changed);
    price_b.set_trigger(CallbackTrigger::Changed);
    price_af.set_trigger(CallbackTrigger::Changed);
    price_bf.set_trigger(CallbackTrigger::Changed);


    let (s, r) = app::channel::<bool>();

    price_a.emit(s, true);
    price_b.emit(s, true);
    
    price_af.emit(s, false);
    price_bf.emit(s, false);


    
    while app.wait() {


        //FUCK ERROR HANDLING
        let price_a_clean :f64 = if price_a.value().is_empty() {1.0} else {price_a.value().parse().unwrap()};
        let price_b_clean :f64 = if price_b.value().is_empty() {1.0} else {price_b.value().parse().unwrap()};
        let price_af_clean :f64 = if price_af.value().is_empty() {price_a_clean} else {price_af.value().parse().unwrap()};
        let price_bf_clean :f64 = if price_bf.value().is_empty() {price_b_clean} else {price_bf.value().parse().unwrap()};




        match r.recv() {
            Some(msg) => {
                if msg {
                    frame5.set_label(&format!("Impermanent loss is: {:.2}%", calculation(price_a_clean, price_b_clean, price_af_clean, price_bf_clean)));
                } else {
                    frame5.set_label(&format!("Impermanent loss is: {:.2}%", calculation(price_a_clean, price_b_clean, price_af_clean, price_bf_clean)));
                }
            }
            None => (),

        }
    }

}

fn calculation(price_a :f64, price_b :f64, price_af :f64, price_bf :f64) -> f64 {
    
    let step_1 :f64 = price_b*price_af+price_a*price_bf;
    let step_2 :f64 = (price_a*price_b*price_bf)/price_af;
    let step_3 :f64 = (price_a*price_b*price_af)/price_bf;
    let step_4 :f64 = price_b*price_af+price_a*price_bf;

    let impermanent_loss :f64 = ((step_1-(((step_2.sqrt())*price_af)+((step_3.sqrt())*price_bf)))/step_4)*100_f64;
    return impermanent_loss;
}