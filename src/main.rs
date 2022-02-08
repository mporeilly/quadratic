use std::env;
use std::str::FromStr;

fn cmdlineformater(cmd_value: &Vec<String>, index: usize) -> f64 {                      // index values within vectors are "usize" values not i32, f64, etc.
    let value = f64::from_str(&cmd_value[index]).expect("THE INPUT IS NOT A NUMBER!");
    return value
}

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    // if less than 4 args or not numbers throw error
    if args.len() != 4{
        println!("This program requires 3 inputs");
    }else if f64::from_str(&args[1]) == Ok(0.0){
        println!("DIVIDE BY ZERO ERROR: \"A\" VALUE ZERO!");
    } else{

        let a: f64 = cmdlineformater(&args,1);   // variable needs to be in function which is in another scope so it has to be referenced
        let b: f64 = cmdlineformater(&args,2);
        let c: f64 = cmdlineformater(&args,3);
        //let a = f64::from_str(&args[1]).expect("The Input is not a number!");   left as a comparison between it and funciton
        println!("{}x^2 + {}x + {}", a,b,c);

        let front_term: f64 = -b/(2f64*a);
        let back_term_numerator: f64 = b.powf(2f64)-(4f64*a*c);

        if back_term_numerator < 0.0{       // get an error if "0" is there didn't want to use 0f64, compiler offered up 0.0 as a float literal to compare

            let back_term: f64 = ((b.powf(2f64)-(4f64*a*c))*(-1f64)).sqrt()/(2f64*a);
            
            println!("The roots of the above function are {}±{:.4}j",front_term,back_term);

        }else{

            let back_term: f64 = (b.powf(2f64)-(4f64*a*c)).sqrt()/(2f64*a);

            let positive_root: f64 = front_term+back_term;
            let negative_root: f64 = front_term-back_term;

            println!("The roots of the above function are {:.4} and {:.4}",positive_root,negative_root);
        }
    }
}