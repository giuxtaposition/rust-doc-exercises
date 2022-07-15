pub fn first_letter_to_uppercase(input: &str) -> String {
    format!("{}", input[0..1].to_uppercase() + &input[1..])
}

pub fn twelve_days_of_christmas() -> String {
    let gifts = [
        "a partridge in a pear tree",
        "two turtledoves",
        "three french hens",
        "four calling birds",
        "five gold rings",
        "six geese-a-laying",
        "seven swans-a-swimming",
        "eight maids-a-milking",
        "nine ladies dancing",
        "ten lords-a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let mut song = String::new();
    for i in 0..12 {
        song.push_str(&format!(
            "On the {} day of Christmas, my true love sent to me ",
            days[i]
        ));
        song.push_str("\n");
        for j in (0..i + 1).rev() {
            if j == 0 && i != 0 {
                song.push_str(&format!("And {}", gifts[j]));
            } else {
                song.push_str(&first_letter_to_uppercase(gifts[j]));
            }
            song.push_str("\n");
        }
        song.push_str("\n");
    }

    song.push_str(&format!("And {}", gifts[0]));
    song
}

pub fn main() {
    println!("{}", twelve_days_of_christmas());
}
