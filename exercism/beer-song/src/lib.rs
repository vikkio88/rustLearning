const NUM_PH: &str = "FIRST";
const NUM2_PH: &str = "SECOND";
const VERSE: &str = "FIRST bottles of beer on the wall, FIRST bottles of beer.\nTake one down and pass it around, SECOND bottles of beer on the wall.\n";
const VERSE_ZERO: &str = "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n";
const VERSE_TWO: &str = "FIRST bottles of beer on the wall, FIRST bottles of beer.\nTake one down and pass it around, SECOND bottle of beer on the wall.\n";
const VERSE_ONE: &str = "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n";

pub fn verse(n: u32) -> String {
    match n {
        0 => String::from(VERSE_ZERO),
        1 => String::from(VERSE_ONE),
        2 => String::from(
            VERSE_TWO
                .replace(NUM_PH, format!("{n}").as_str())
                .replace(NUM2_PH, format!("{}", n - 1).as_str()),
        ),
        _ => String::from(
            VERSE
                .replace(NUM_PH, format!("{n}").as_str())
                .replace(NUM2_PH, format!("{}", n - 1).as_str()),
        ),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song = "".to_string();
    (end..=start).rev().for_each(|i| {
        if song != "".to_string() {
            song += "\n";
        }
        song += &verse(i);
    });

    song
}
