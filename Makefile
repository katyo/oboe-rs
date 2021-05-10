doc:
	@DOCS_RS=1 cargo +nightly doc --features java-interface,doc-cfg --target x86_64-linux-android
