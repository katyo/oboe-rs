doc:
	@DOCS_RS=1 cargo +nightly doc --features java-interface,doc-cfg --target x86_64-linux-android

ANDROID_TARGETS := \
  armv7-linux-androideabi \
  aarch64-linux-android \
  i686-linux-android \
  x86_64-linux-android

ANDROID_API-armv7-linux-androideabi := 16
ANDROID_API-i686-linux-android := 16
ANDROID_API-aarch64-linux-android := 21
ANDROID_API-x86_64-linux-android := 21

define bindgen-rules
bindgen: bindgen-$(1)
bindgen-$(1):
	@cargo ndk --android-platform $$(ANDROID_API-$(1)) --target $(1) -- build --release --features generate-bindings
	@cp target/$(1)/release/build/oboe-sys-*/out/bindings.rs sys/src/bindings_`echo $(1) | sed -r 's/^([^-]+).*$$$$/\1/'`.rs
endef

$(foreach target,$(ANDROID_TARGETS),$(eval $(call bindgen-rules,$(target))))

keygen:
	@keytool -genkey -dname "cn=Illumium, ou=RnD, o=illumium.org, c=US" -v -keystore demo/release.keystore -alias demo -keyalg RSA -keysize 2048 -validity 20000
