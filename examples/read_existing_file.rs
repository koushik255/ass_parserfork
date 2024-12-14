use ass_parser::{AssFile, Dialogue};

fn main() -> Result<(), std::io::Error>{
    let ass_file = AssFile::from_file("examples/subtitles.ass")?;
    let dialogues: Vec<Dialogue> = ass_file.events.get_dialogues();

    for dialogue in dialogues {
        println!("layer: {:?}", &dialogue.get_layer());
        println!("name: {:?}", &dialogue.get_name());
        println!("end: {:?}", &dialogue.get_end());
        println!("start: {:?}", &dialogue.get_start());
        println!("text: {:?}", &dialogue.get_text());
        println!("marginl: {:?}", &dialogue.get_marginl());
        println!("marginr: {:?}", &dialogue.get_marginr());
        println!("marginv: {:?}", &dialogue.get_marginv());
        println!("style: {:?}", &dialogue.get_style());
        println!("effect: {:?}", &dialogue.get_effect());
        println!("colour: {:?}", &dialogue.get_colour());
    }


    Ok(())
}
