rust_lib_name := axlibc
rust_lib := target/$(TARGET)/$(MODE)/lib$(rust_lib_name).a

ulib_dir := ulib/axlibc
src_dir := $(ulib_dir)/c
obj_dir := $(ulib_dir)/build_$(ARCH)
inc_dir := $(ulib_dir)/include
inc_gen_dir = $(ulib_dir)/include_gen
c_lib := $(obj_dir)/libc.a
libgcc :=

in_feat := $(APP)/features.txt
out_feat := $(obj_dir)/.features.txt

ulib_src := $(wildcard $(src_dir)/*.c)
ulib_hdr := $(wildcard $(inc_dir)/*.h)
ulib_obj := $(patsubst $(src_dir)/%.c,$(obj_dir)/%.o,$(ulib_src))

CFLAGS += -nostdinc -static -no-pie -fno-builtin -ffreestanding -Wall
CFLAGS += -I$(inc_dir) -I$(inc_gen_dir)
LDFLAGS += -nostdlib -static -no-pie --gc-sections -T$(LD_SCRIPT)

ifeq ($(MODE), release)
  CFLAGS += -O3
endif

ifneq ($(wildcard $(in_feat)),)	# check if features.txt contains "fp_simd"
  fp_simd := $(shell grep "fp_simd" < $(in_feat))
endif

ifeq ($(ARCH), riscv64)
  CFLAGS += -march=rv64gc -mabi=lp64d -mcmodel=medany
endif

ifeq ($(fp_simd),)
  ifeq ($(ARCH), x86_64)
    CFLAGS += -mno-sse
  else ifeq ($(ARCH), aarch64)
    CFLAGS += -mgeneral-regs-only
  endif
else
  ifeq ($(ARCH), riscv64)
    # for compiler-rt fallbacks like `__addtf3`, `__multf3`, ...
    libgcc := $(shell $(CC) -print-libgcc-file-name)
  endif
endif

ifneq ($(wildcard $(in_feat)),)
_gen_feat: $(obj_dir)
  # copy "feature.txt" to ".feature.txt" and trigger rebuild if changed
  ifneq ($(shell diff -Nq $(in_feat) $(out_feat)),)
	$(shell cp $(in_feat) $(out_feat))
  endif
else
_gen_feat: $(obj_dir)
  # create an empty ".feature.txt"
  ifneq ($(shell cat $(out_feat) 2>&1),)
	@touch $(out_feat)
  endif
endif

$(obj_dir):
	$(call run_cmd,mkdir,-p $@)

$(obj_dir)/%.o: $(src_dir)/%.c $(out_feat)
	$(call run_cmd,$(CC),$(CFLAGS) -c -o $@ $<)

$(c_lib): $(obj_dir) _gen_feat $(ulib_obj)
	@rm -f $@
	$(call run_cmd,$(AR),rc $@ $(ulib_obj))
	$(call run_cmd,$(RANLIB),$@)

app-objs := main.o

-include $(APP)/axbuild.mk  # override `app-objs`

app-objs := $(addprefix $(APP)/,$(app-objs))

$(APP)/%.o: $(APP)/%.c $(ulib_hdr)
	$(call run_cmd,$(CC),$(CFLAGS) $(APP_CFLAGS) -c -o $@ $<)

$(OUT_ELF): $(app-objs) $(c_lib) $(rust_lib) $(libgcc)
	@printf "    $(CYAN_C)Linking$(END_C) $(OUT_ELF)\n"
	$(call run_cmd,$(LD),$(LDFLAGS) $^ -o $@)

$(APP)/axbuild.mk: ;

.PHONY: _gen_feat
