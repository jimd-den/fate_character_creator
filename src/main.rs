use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::io;
use std::io::Write;

/// This is a simple fate character creator.
/// 
/// it's a pet project. nothing serious. Open source.
///
/// We basically prompt the users for the values and give them a json of the character sheet

#[derive(Debug, Serialize, Deserialize)]
struct AspectList {
    high_concept: String,
    trouble: String,
    aspect_1: String,
    aspect_2: String,
    aspect_3: String,
}

/// This is the Character with name, consequences approaches
#[derive(Debug, Serialize, Deserialize)]
struct Character {
    name: String,
    stress: i32,
    consequences: ConsequenceList,
    aspect_list: AspectList,
    approaches: ApproachTuple,

}



/// there are three consequences on the charactersheet
#[derive(Debug, Serialize, Deserialize)]
struct ConsequenceList {
    con1: String,
    con2: String,
    con3: String,
}

/// This allows you store approach values for dice rolls
#[derive(Debug, Serialize, Deserialize)]
struct ApproachTuple {
    careful: i32,
    clever: i32,
    flashy: i32,
    forceful: i32,
    quick: i32,
    sneaky: i32,
}


/// take input set it to value
fn prompt(x: &str) -> String {
    print!{"{}", x};
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().to_string()
}

/// this allows the player to select the skills based on the fate game mechanics
fn approach_generator() -> Vec<i32> {
    fn print_appr_and_prompt(x: &Vec<i32>) -> i32 {
        let mut sel = 0;
        let mut appr = vec!["Careful", "Clever", 
                            "Flashy", "Forceful", 
                            "Quick", "Sneaky"];
        let mut app_count = 0;
        while app_count < appr.len() {
            if x[app_count] == 999 {
                print!("({}. {}) ", (app_count + 1), appr[app_count]);
            }
            app_count += 1;
        }
        app_count = 0;
        print!("\n");
        
        sel = prompt("> ").parse::<i32>().unwrap();
        sel = sel - 1;
        while x[sel as usize] != 999 {
            println!("Invalid Selection: Already Chosen");
            sel = prompt("> ").parse::<i32>().unwrap();
            sel = sel - 1;
        }
        return sel
            
    }

    let mut appr_num = vec![999,999,999,999,999,999];
    let mut skill_key = 0;
    println!("good strength?");
    skill_key = print_appr_and_prompt(&appr_num);
    appr_num[skill_key as usize] = 3;
    
    println!("fair strength 1?");
    skill_key = print_appr_and_prompt(&appr_num);
    appr_num[skill_key as usize] = 2;

    println!("fair strength 2?");
    skill_key = print_appr_and_prompt(&appr_num);
    appr_num[skill_key as usize] = 2;
    
    println!("mediocre strength 1?");
    skill_key = print_appr_and_prompt(&appr_num);
    appr_num[skill_key as usize] = 1;

    println!("mediocre strength 2?");
    skill_key = print_appr_and_prompt(&appr_num);
    appr_num[skill_key as usize] = 1;
    
    let mut ap_ct = 0;
    while ap_ct < appr_num.len() {
        if appr_num[ap_ct] == 999 {
            appr_num[ap_ct] = 0;
        }
        ap_ct += 1
    }

    println!("{:?}", appr_num);
    appr_num
}


fn aspect_generator() -> AspectList {
    let mut asp_gen = AspectList {high_concept: String::new(), 
                                  trouble: String::new(), 
                                  aspect_1: String::new(), 
                                  aspect_2: String::new(), 
                                  aspect_3: String::new(),};
    println!("Let's create your aspects:");
    println!("Aspects are phrases or facts that are important to your character");
    asp_gen.high_concept = prompt("Who is your character and why does this person exist?");
    asp_gen.trouble = prompt("What is a troubling aspect about your character? ");
    asp_gen.aspect_1 = prompt("What is another aspect of your character? (anything)  ");
    let ans = prompt("Would you like to add additional aspects? (y)es or (n)o ");
    if ans == "y" {
        asp_gen.aspect_2 = prompt("2nd aspect? ");
        asp_gen.aspect_3 = prompt("3rd aspect? ");
        asp_gen
    }
    else {
        asp_gen
    }

}

fn main() {
    let _name = prompt("Name: ");
    let _aspect_list = aspect_generator();
    let mut appr_vec = Vec::new();
    appr_vec =  approach_generator();
    let _appr_buf_tuple = ApproachTuple {careful: appr_vec[0], 
                                         clever: appr_vec[1],
                                         flashy: appr_vec[2],
                                         forceful: appr_vec[3],
                                         quick: appr_vec[4],
                                         sneaky: appr_vec[5],};
    let cons_blank =  ConsequenceList {con1: "blank".to_string(),
                                       con2: "blank".to_string(),
                                       con3: "blank".to_string()};

    let char_1 = Character {name: _name,
                            stress: 0,
                            consequences: cons_blank,
                            aspect_list: _aspect_list,
                            approaches: _appr_buf_tuple,};
    let mut char_output = serde_json::to_string_pretty(&char_1).unwrap();


    std::fs::write("characters/chr1.json", &char_output).unwrap();
}
