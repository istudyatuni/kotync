[private]
@default:
	just --list

# prune and setup db
remake-db:
	rm data.db
	diesel setup
	diesel migration run
