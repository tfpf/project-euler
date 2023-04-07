SHELL   = /bin/sh
RC      = rustc
RCFLAGS = -C opt-level=2 -C strip=debuginfo
RM      = rm -f
PRINT   = printf

Sources     = $(wildcard */*/*.rs)
Executables = $(Sources:.rs=)

.PHONY: all clean

all: $(Executables)

clean:
	@$(PRINT) "Removing all executables.\n"
	@$(RM) $(Executables)

%: %.rs
	$(RC) $(RCFLAGS) -o $@ $<
