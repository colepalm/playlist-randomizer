# Compiles UI
compile-ui:
	cd wasm-ui; \
		wasm-pack build --target web --out-name wasm --out-dir ./static

serve-ui:
	miniserve ./static --index index.html