doc:
	cargo doc
	rsync -vre ssh --delete target/doc/ purple:www/rust
