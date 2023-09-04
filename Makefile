dump:
	DUMP_UPDATES=1 cargo run
silent:
	cargo run
dump-verbose:
	DUMP_UPDATES=1 DUMP_UPDATES_DATA=1 cargo run

.PHONY:
	silent dump dump-verbose
