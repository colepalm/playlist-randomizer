# Writes WASM UI to /static directory
compile:
	wasm-pack build --target web

# Serves application at port 8080
serve:
	miniserve . --index index.html