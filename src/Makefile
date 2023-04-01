SHELL   = /bin/sh
RC      = rustc
RCFLAGS = -O
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
