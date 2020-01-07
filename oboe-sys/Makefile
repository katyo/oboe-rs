TARGETS := armv7-linux-androideabi aarch64-linux-android i686-linux-android x86_64-linux-android

lib ?= oboe
crate ?= oboe-sys
conf ?= release
link ?= shared
ver ?= 0.1.0

arch = $(firstword $(subst -, ,$(1)))
outpath = ../target/$(1)/$(conf)/build/$(crate)-*/out/$(2)
out = $(if $(wildcard $(call outpath,$(1),$(2))),$(firstword $(shell ls -t $(call outpath,$(1),$(2)))))
libext = $(if $(filter static,$(link)),a,so)

define copy-target-rs
copy-out: copy-out-$(1)
copy-out-$(1):
ifneq ($(call out,$(1)),)
	$(if $(call out,$(1),bindings.rs),@cp $(call out,$(1),bindings.rs) src/bindings_$(call arch,$(1)).rs)
	@mkdir -p lib/$(conf)/$(call arch,$(1))
	$(if $(call out,$(1),build/lib$(lib)-ext.$(libext)),@cp $(call out,$(1),build/lib$(lib)-ext.$(libext)) lib/$(conf)/$(call arch,$(1))/lib$(lib)-ext.$(libext))
else
	@echo No prebuild outputs for target:$(1) and config:$(conf)
endif
show-out: show-out-$(1)
show-out-$(1):
ifneq ($(call out,$(1)),)
	$(if $(call out,$(1),bindings.rs),@ls -al $(call out,$(1),bindings.rs))
	$(if $(call out,$(1),build/lib$(lib)-ext.$(libext)),@ls -al $(call out,$(1),build/lib$(lib)-ext.$(libext)))
else
	@echo No prebuild outputs for target:$(1) and config:$(conf)
endif
pack-out: copy-out-$(1)
endef

pack-out:
	@mkdir -p ../target/prebuilt
	@tar -czf ../target/prebuilt/$(crate)_$(ver)_$(conf).tar.gz -C lib/$(conf) .

$(foreach target,$(TARGETS),$(eval $(call copy-target-rs,$(target))))

doc:
	@cargo doc --features rustdoc
