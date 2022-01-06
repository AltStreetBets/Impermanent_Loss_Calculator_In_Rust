use text_io::read;

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
}

fn amountbased() {
    println!("You choose amount based");
    println!("What was initial amount of token A?");
    let amount_a :f32 = read!();

    println!("What was initial amount of token B?");
    let amount_b :f32 = read!();
    
    println!("What was final amount of token A?");
    let amount_af :f32 = read!();

    println!("What was final amount of token B?");
    let amount_bf :f32 = read!();

    let step_11 :f32 = amount_b*amount_af+amount_a*amount_bf;
    let step_22 :f32 = (amount_a*amount_b*amount_bf)/amount_af;
    let step_33 :f32 = (amount_a*amount_b*amount_af)/amount_bf;
    let step_44 :f32 = amount_b*amount_af+amount_a*amount_bf;

    //lol, I am such a nitwitt, you can just use the same formula. Anyways, was good practice.
    let impermanent_loss1 :f32 = ((step_11-(((step_22.sqrt())*amount_af)+((step_33.sqrt())*amount_bf)))/step_44)*100_f32;
    println!("Impermanent loss is: {:.2}%", impermanent_loss1);
}
