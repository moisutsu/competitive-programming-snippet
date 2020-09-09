link:
	cargo fmt
	cargo test
	cargo snippet -t vscode >| ~/Library/Application\ Support/Code/User/snippets/rust.json

cat:
	cat ~/Library/Application\ Support/Code/User/snippets/rust.json
