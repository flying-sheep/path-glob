docs:
	cargo doc
	echo '<meta http-equiv=refresh content=0;url=path_glob>' > target/doc/index.html
	ghp-import -np target/doc
