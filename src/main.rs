mod battle;

use parthia_lib::fegame::FEGame;
use parthia_lib::simple_calc::{CombatStats, SpeedDiff};
use battle::Battle;


use warp::Filter;

#[tokio::main]
async fn main() {
    env_logger::init();
    let bat = Battle{
        atk_hp: 40,
        atk: CombatStats{
            dmg: 20,
            hit: 90,
            crit: 10,
            is_brave: false,
        },
        def_hp: 50,
        def: CombatStats{
            dmg: 10,
            hit: 70,
            crit: 50,
            is_brave: false,
        },
        speed: SpeedDiff::AtkDoubles,
        game: FEGame::FE15,

    };

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["User-Agent", "Sec-Fetch-Mode", "Referer", "Origin",
                            "Access-Control-Request-Method", "Content-Length",
                            "Access-Control-Request-Headers", "Content-Type" ])
        .allow_method("POST");

    let debug = warp::path!("debug")
        .map(move || format!("Hi!\n {}", serde_json::to_string(&bat.clone()).unwrap()));


    let calc = warp::path!("calc")
        .and(warp::body::json())
        .map(|extract: Battle| serde_json::to_string(&extract.outcomes()).unwrap())
        .with(cors);


    warp::serve(debug.or(calc))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
