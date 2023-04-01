SHELL   = /bin/sh
RC      = rustc
RCFLAGS = -O
RM      = rm -f

Sources     = $(wildcard */*/*.rs)
Executables = $(Sources:.rs=)

.PHONY: all clean

all: $(Executables)

clean:
	$(RM) $(Executables)

%: %.rs
	$(RC) $(RCFLAGS) -o $@ $<
