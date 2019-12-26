TARGETS := armv7-linux-androideabi aarch64-linux-android i686-linux-android x86_64-linux-android

lib ?= oboe
crate ?= oboe-sys
conf ?= release

arch = $(firstword $(subst -, ,$(1)))
out = $(firstword $(shell ls -t ../target/$(1)/$(conf)/build/$(crate)-*/out/$(2)))

define copy-target-rs
copy-out: copy-out-$(1)
copy-out-$(1):
ifneq ($(call out,$(1)),)
	@cp $(call out,$(1),bindings.rs) src/bindings_$(call arch,$(1)).rs
	@cp $(call out,$(1),build/$(lib)/lib$(lib).a) lib/lib$(lib)_$(call arch,$(1)).a
	@cp $(call out,$(1),build/lib$(lib)-ext.a) lib/lib$(lib)-ext_$(call arch,$(1)).a
else
	@echo No prebuild outputs for target:$(1) and config:$(conf)
endif
show-out: show-out-$(1)
show-out-$(1):
ifneq ($(call out,$(1)),)
	@ls -al $(call out,$(1),bindings.rs) $(call out,$(1),build/$(lib)/lib$(lib).a) $(call out,$(1),build/lib$(lib)-ext.a)
else
	@echo No prebuild outputs for target:$(1) and config:$(conf)
endif
endef

$(foreach target,$(TARGETS),$(eval $(call copy-target-rs,$(target))))

doc:
	@cargo doc --features rustdoc
