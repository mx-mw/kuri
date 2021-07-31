use ansi_term::Colour;

/*****************************************
*****Print nice looking text(nice io)*****
*****************************************/
pub fn red(text: String) {
    println!("{}", Colour::Red.bold().paint(text));
}   

pub fn green(text: String) {
    println!("{}", Colour::Green.bold().paint(text))
}   