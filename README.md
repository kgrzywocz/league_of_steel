# League of Steel

Make [SteelSeries GameSense](https://steelseries.com/engine/apps) work with [League of Legends](https://leagueoflegends.com/) !!!

It currently supports events: HEALTH, MANA and HIT.

## How to run

1. Make sure you have [SteelSeries Engine 3](https://steelseries.com/engine) installed and it is running properly.
2. Launch ```league_of_steel_setup.exe``` - it should install and run.
3. Configure events for your device
4. Enjoy playing LoL with GameSense ;)

## How to build this app

This application written in Rust and C++, so on Windows with [correctly setup environment](https://rustup.rs/) run:

    cargo build --release

And if every thing went correct you will find app in ```target/release/league_of_steel.exe```

## Remarks etc

Program support only fullscreen mode and was tested under resolution 1920x1080.

Special thanks to ~~Kasia~~Karolina (fabuleuse) ;)