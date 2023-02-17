dev:
	cargo watch -x run

initdatabase:
	sqlx database create && sqlx migrate run

resetdatabase: 
	rm data.*
	$(MAKE) initdatabase

run:
	cargo run
