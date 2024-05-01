use framework::bots_api::BotsApi;
use framework::structs::update::Update;

fn main() {
    let bots_api = BotsApi::new();

    bots_api.pooling(true, move |update: Update| {
        println!("{update:?}");
    })
}
