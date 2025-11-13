CHARON_HOME	?= $(dir $(abspath $(lastword $(MAKEFILE_LIST))))/../charon
AENEAS_HOME	?= $(dir $(abspath $(lastword $(MAKEFILE_LIST))))/../aeneas

CHARON_EXE = $(CHARON_HOME)/bin/charon
AENEAS_EXE = $(AENEAS_HOME)/bin/aeneas

AENEAS_OPTIONS ?=

.PHONY: extract
extract: gcd.llbc
	$(AENEAS_EXE) -backend lean gcd.llbc -split-files -dest lean $(AENEAS_OPTIONS)

gcd.llbc: $(wildcard */*.rs)
	RUSTFLAGS="--cfg eurydice" $(CHARON_EXE) cargo --preset=aeneas
