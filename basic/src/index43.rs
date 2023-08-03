fn give_adult(drink: Option<&str>) {
    // Specify a course of action for each case.
    match drink {
        Some("lemonade") => println!("Yuck! Too sugary."),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No drink? Oh well."),
    }
}

fn drink(drink: Option<&str>) {
    // `unwrap` returns a `panic` when it receives a `None`.
    // `unwrap`を使用すると値が`None`だった際に`panic`を返します。
    let inside = drink.unwrap();
    if inside == "lemonade" {
        panic!("AAAaaaaa!!!!");
    }

    println!("I love {}s!!!!!", inside);
}

fn main() {
    let water: Option<&str> = Some("water");
    let lemonade: Option<&str> = Some("lemonade");
    let void: Option<&str> = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee: Option<&str> = Some("coffee");
    let nothing: Option<&str> = None;

    drink(coffee);
    drink(nothing);
}
