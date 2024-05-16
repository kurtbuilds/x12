run:
    cargo run

test *ARGS:
    cd serde_x12 && just test --all-features {{ ARGS }}