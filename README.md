# League of Steel

Make [SteelSeries GameSense](https://steelseries.com/engine/apps) work with [League of Legends](https://leagueoflegends.com/) !!!
Now also work with [Fortnite](https://www.epicgames.com/fortnite) - maybe more is coming;)

[![How to install league_of_steel_video](http://img.youtube.com/vi/87hzCs3y4Yo/0.jpg)](http://www.youtube.com/watch?v=87hzCs3y4Yo "How to install league_of_steel")

It currently supports events: HEALTH, MANA and HIT.

## How to run

1. Make sure you have [SteelSeries Engine 3](https://steelseries.com/engine) installed and it is running properly.
2. Download ```league_of_steel_setup.exe```. You can find latest [here](https://github.com/kgrzywocz/league_of_steel/releases).
3. Launch ```league_of_steel_setup.exe``` - it should install and run.
4. Configure events for your device in the SteelSeries Engine 3.
5. Enjoy playing LoL with GameSense ;)

## How to build this app

This application written in Rust and C++, so on Windows with [correctly setup environment](https://rustup.rs/) run:

    cargo build --release

And if every thing went correct you will find app in ```target/release/league_of_steel.exe```

## Remarks etc

Program support only fullscreen mode and was tested under resolution 1920x1080.

Special thanks to ~~Kasia~~Karolina (fabuleuse), bandrews3030150(TheBrian666), Muratcan Öztürk, Marcel Ortgies, 김민준, Martijn Gribnau (foresterre)
 ;)
