fn main() {
    let ordinal = [
        "first", 
        "second", 
        "third", 
        "fourth", 
        "fifth", 
        "sixth", 
        "seventh", 
        "eighth", 
        "ninth", 
        "tenth", 
        "eleventh", 
        "twelfth"
    ];

    let lyrics = [
        "And a partridge in a pear tree", 
        "Two turtledoves", 
        "Three French hens",
        "Four calling birds",
        "Five gold rings (five golden rings)",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "I sent eleven pipers piping",
        "Twelve drummers drumming"
    ];

    for i in 0..12 {
        println!("On the {} day of Christmas, my true love sent to me", ordinal[i]);
        if i == 0 {
            println!("A partridge in a pear tree");
        }
        else {
            for j in (0..i + 1).rev() {
                println!("{}", lyrics[j]);
            }
        }
        println!();
    }
    println!("{}", lyrics[0]);
}
