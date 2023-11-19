cargo bundle --release
rm target/CaesarCommander.dmg
appdmg spec.json target/CaesarCommander.dmg
