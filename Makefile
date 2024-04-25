AZERO_ENV ?= dev

.PHONY: help
help: # Show help for each of the Makefile recipes.
	@grep -E '^[a-zA-Z0-9 -]+:.*#'  Makefile | sort | while read -r l; do printf "\033[1;32m$$(echo $$l | cut -f 1 -d':')\033[00m:$$(echo $$l | cut -f 2- -d'#')\n"; done

.PHONY: clean
clean: # Remove all node and compilation data
clean:
	git clean -fdx

.PHONY: devnode
devnode: # Run azero devnode
devnode:
	docker compose -f ./devnet-azero/devnet-azero-compose.yml up

.PHONY: deps
deps: # Install npm dependencies
deps:
	npm install

.PHONY: compile
compile: # compile contracts and generate artifacts
compile: deps
	npm run compile

.PHONY: deploy-escrow
deploy-escrow: # deploy escrow contract
deploy-escrow: compile
	npm run deploy-escrow

.PHONY: deploy-old-a
deploy-old-a: # deploy old-a contract
deploy-old-a: compile
	npm run deploy-old-a

.PHONY: upload-new-a
upload-new-a: # uploads new contract code
upload-new-a: deploy-old-a
	npm run upload-new-a

.PHONY: upgrade-a
upgrade-a: # uploads new contract code
upgrade-a: upload-new-a
	npm run upgrade-a
