[private]
@default:
	just --list

# prune db
remake-db:
	rm data.db
