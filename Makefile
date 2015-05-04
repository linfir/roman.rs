doc:
	cargo doc
	rsync -vre ssh --delete target/doc/ tentacle:www/rust
