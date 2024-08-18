[Server]

cargo watch -q -c -w src/ -x run

[Client]

cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
