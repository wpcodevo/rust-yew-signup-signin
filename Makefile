install:
	cargo add yew --features csr
	cargo add serde_json
	cargo add serde --features derive
	cargo add chrono --features serde
	cargo add reqwasm
	cargo add yew-router
	cargo add gloo
	cargo add validator --features derive
	cargo add yewdux
	cargo add wasm-bindgen
	cargo add web-sys --features "HtmlInputElement Window"
	cargo add wasm-bindgen-futures

start-app:
	trunk serve --port 3000