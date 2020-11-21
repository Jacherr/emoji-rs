

fn main() {
    println!(
        "The {} emoji is at codepoint {}, called {}, is {}, and was introduced in unicode version {}",
        emoji::food_and_drink::food_marine::CRAB.glyph,
        emoji::food_and_drink::food_marine::CRAB.codepoint,
        emoji::food_and_drink::food_marine::CRAB.name,
        emoji::food_and_drink::food_marine::CRAB.status,
        emoji::food_and_drink::food_marine::CRAB.introduction_version
    );
    println!("{}", emoji::lookup_by_glyph::lookup("🤳").unwrap().name);
    println!("{}", emoji::lookup_by_name::lookup("crab").unwrap().glyph);

    println!("{:?}", emoji::ANNOTATION_LANGS);
    println!("{:?}", emoji::food_and_drink::food_marine::CRAB.annotations.iter().find(
	|a| a.lang == "en"
    ).unwrap().keywords);
    println!("{:?}", emoji::food_and_drink::food_marine::CRAB.annotations.iter().find(
	|a| a.lang == "fi"
    ).unwrap().keywords);
}
