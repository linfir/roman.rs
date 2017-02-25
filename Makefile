doc:
	cargo doc
	rsync -vre ssh --delete target/doc/ hoagie:www/roman.rs
