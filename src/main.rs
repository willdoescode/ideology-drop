use rand::seq::SliceRandom;
use rand::thread_rng;
use rocket::response::content::Html;

#[macro_use]
extern crate rocket;

#[get("/")]
async fn index() -> Html<String> {
    Html(std::fs::read_to_string("public/index.html").unwrap())
}

#[get("/ideology")]
async fn ideology() -> Html<String> {
    let prefixes = [
        "Monarchic",
        "Anarcho",
        "Communist",
        "Capitalist",
        "Technocratic",
        "Primitivist",
        "Accelerationist",
        "Fudile",
        "Socialist",
        "Maoist",
        "Dengist",
        "Marxist",
        "Leninist",
        "Post",
        "Transhumanist",
        "Authoritarianist",
        "Autocratic",
        "State",
        "Conservative",
        "Fascist",
        "Anti",
        "Market",
        "National",
        "Stalinist",
        "Neo",
        "Eco",
        "Absolute",
        "Libertarian",
        "Theocratic",
        "Alt",
        "Counter",
        "Revolutionary",
        "Liberal",
        "Cultural",
        "Classical",
        "Techno",
    ];
    let ideologys = [
        "Communism",
        "Socialism",
        "Monarchism",
        "Anarchism",
        "Transhumanism",
        "Technocraticism",
        "Primitivism",
        "Accelerationism",
        "Feudalism",
        "Moaism",
        "Capitalism",
        "Dengism",
        "Marxism",
        "Leninism",
        "Authoritarianism",
        "Autocracy",
        "Dictatorship",
        "Colonialism",
        "Conservatism",
        "Facism",
        "Juche",
        "Nationalism",
        "Stalanism",
        "Libertarianism",
        "Theocracy",
        "Kevelinism",
        "Liberalism",
    ];

    let mut rng = thread_rng();
    let idiology = format!(
        "{}-{}-{}",
        prefixes.choose(&mut rng).unwrap(),
        prefixes.choose(&mut rng).unwrap(),
        ideologys.choose(&mut rng).unwrap()
    );

    Html(std::fs::read_to_string("public/ideology.html").unwrap().replace("Idiology", &idiology))
}

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![index, ideology])
}
