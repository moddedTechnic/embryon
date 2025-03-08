SRC := $(wildcard ../../**/src/*.rs)
EXE := ../../target/release/embryon-cli

TEST_FILES := $(wildcard *.embryon)
LL_FILES := $(patsubst %.embryon,%.ll,$(TEST_FILES))


test: $(EXE) $(LL_FILES)

$(EXE): $(SRC)
	cargo build --release

%.ll: %.embryon
	@echo Compiling $^
	@$(EXE) $^
	@lli $@; echo Exited with status code $$?
	@echo

clean:
	-rm -f *.ll
	-rm -f *.bc
	-rm -f *.s
	-rm -f *.elf

.PHONY: test clean
