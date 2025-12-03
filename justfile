alias i := install

alias f := format

alias r := run

alias a := add  

alias b := build

alias g := generate

install:
	pnpm install

format:
	pnpm run format

run:
	pnpm run dev

build:
	pnpm build

clean:
	pnpm run clean

lint:
	pnpm run lint

[working-directory: 'crates/calculator']
build-wasm:
	pnpm run build:wasm

