dev:
	cargo watch -x run
resetdatabase:
	rm data.* && sqlx database create && sqlx migrate run
run:
	cargo run
