# SELECTOR ?= fzf
SELECTOR ?= termenu --color=always
SQLITE_SCHEMA = resource/sqlite.schema.sql
SQLITE_ALL = resource/sqlite.all.sql
.PHONY: example
example:
	@eg=$$(ls examples | awk -F. '{print $$1}' | ${SELECTOR}) && \
		cargo run --example $$eg

.PHONY: dump_schema
dump_schema:
	@sqlite3 raac.db ".schema" > ${SQLITE_SCHEMA}
	@sql-formatter --fix ${SQLITE_SCHEMA}

.PHONY: dump_all
dump_all:
	@sqlite3 raac.db ".dump" > ${SQLITE_ALL}
	@sql-formatter --fix ${SQLITE_ALL}

.PHONY: restore_db
restore_db:
	@rm -f raac.db
	@sqlite3 raac.db < ${SQLITE_ALL}
