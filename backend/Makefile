dev:
	cargo watch -x run

increaselimit:
	ulimit -n 100000

initdatabase:
	sqlx database create && sqlx migrate run

resetdatabase: 
	rm data.*
	$(MAKE) initdatabase

run:
	cargo run | bunyan

test:
	TEST_LOG=true cargo test | bunyan
