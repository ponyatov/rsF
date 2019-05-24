CARGO = ~/.cargo/bin/cargo

RSF = target/debug/rs_forth

all: $(RSF) test.ml
	./$^
$(RSF): src/*
	$(CARGO) build
