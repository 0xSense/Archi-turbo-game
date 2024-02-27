// This is where your main game loop code goes
// The stuff in this block will run ~60x per sec
turbo::go! {
    text!("Choose your class");
    text!(
        "Paladin",
        x = 32,
        y = 48,
        color = 0xff00ffff,
        font = Font::L
    );
}

// game configuration

turbo::cfg!{r#"
    [solana]
    http-rpc-url = "http://localhost:8899"
    ws-rpc-url = "ws://localhost:8900"
"#}