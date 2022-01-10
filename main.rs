//#![windows_subsystem = "windows"]
use std::{thread, time};
use text_io::read;
use fltk::{app, button, frame::*, input, prelude::*, window,};
use fltk_theme::{WidgetTheme, ThemeType};
use fltk_flex::{Flex, FlexType};

fn main() {

    loop {
        println!("price based (0) or amount based (1)");
        let choice_number :i32 = read!();

        if choice_number == 0 {
            pricebased();
            break;
        } else if choice_number == 1 {
            amountbased();
            break;
        } else {
            println!("this wasn't a valid option");
            continue;
        };
    };
}

fn pricebased() {
    println!("You choose price based");
    println!("What was initial price of token A?");
    let price_a :f32 = read!();

    println!("What was initial price of token B?");
    let price_b :f32 = read!();
    
    println!("What was final price of token A?");
    let price_af :f32 = read!();

    println!("What was final price of token B?");
    let price_bf :f32 = read!();

    let step_1 :f32 = price_b*price_af+price_a*price_bf;
    let step_2 :f32 = (price_a*price_b*price_bf)/price_af;
    let step_3 :f32 = (price_a*price_b*price_af)/price_bf;
    let step_4 :f32 = price_b*price_af+price_a*price_bf;

    let impermanent_loss :f32 = ((step_1-(((step_2.sqrt())*price_af)+((step_3.sqrt())*price_bf)))/step_4)*100_f32;
    println!("Impermanent loss is: {:.2}%", impermanent_loss);

    println!("Press 0 to reset values, press any other key to let the programme shit itself");
    let exitchoice :i32 = read!();
    if exitchoice == 0 {
        main();
    } else {
        println!("shitting myself");
        thread::sleep(time::Duration::from_millis(500));
        let mut n = 5;
        while n > 0 {
            println!("{}", n);
            n -= 1;
            thread::sleep(time::Duration::from_millis(1000));
        };
    };
}

fn amountbased() {
    println!("You choose amount based");
    
    println!("What was initial amount of token A?");
    let mut amount_a :f32 = read!();

    println!("What was initial amount of token B?");
    let mut amount_b :f32 = read!();
    

    loop {
        println!("Did you add more tokens? yes(0)");
        let choicetoken :i32 =read!();

        if choicetoken == 0 {
            println!("What was the amount of token A?");
            let amount_a1 :f32 = read!();
            amount_a += amount_a1;

            println!("What was the amount of token B?");
            let amount_b1 :f32 = read!();
            amount_b += amount_b1;            

        } else {
            break;
        };
    };


    let amount_total = amount_a + amount_b;
    let average_a = amount_a / amount_total;
    let average_b = amount_b / amount_total;

    println!("What was final amount of token A?");
    let amount_af :f32 = read!();

    println!("What was final amount of token B?");
    let amount_bf :f32 = read!();
    let total_final = amount_af + amount_bf;
    let average_af = amount_af / total_final;
    let average_bf = amount_bf / total_final;

    let step_11 :f32 = average_b*average_af+average_a*average_bf;
    let step_22 :f32 = (average_a*average_b*average_bf)/average_af;
    let step_33 :f32 = (average_a*average_b*average_af)/average_bf;
    let step_44 :f32 = average_b*average_af+average_a*average_bf;


    //lol, I am such a nitwitt, you can just use the same formula. Anyways, was good practice.
    let impermanent_loss1 :f32 = ((step_11-(((step_22.sqrt())*average_af)+((step_33.sqrt())*average_bf)))/step_44)*100_f32;
    println!("Impermanent loss is: {:.2}%", impermanent_loss1);

    println!("Press 0 to reset values, press any other key to let the programme shit itself");
    let exitchoice :i32 = read!();
    if exitchoice == 0 {
        main();
    } else {
        println!("shitting myself");
        thread::sleep(time::Duration::from_millis(500));
        let mut n = 5;
        while n > 0 {
            println!("{}", n);
            n -= 1;
            thread::sleep(time::Duration::from_millis(1000));
        };
    };
}
