link:
	cargo fmt
	cargo test
	cargo snippet -t vscode >| ~/Library/Application\ Support/Code/User/snippets/rust.json

cat:
	bat ~/Library/Application\ Support/Code/User/snippets/rust.json
