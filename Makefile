# Writes WASM UI to /static directory
compile-ui:
	cd wasm-ui; \
	wasm-pack build --target web --out-name wasm --out-dir ../static

# Serves application at port 8080
serve:
	miniserve ./static --index index.html