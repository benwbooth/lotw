# Matching-disassembly build. Run inside `nix develop`.
AS = ca65
LD = ld65
CFG = lotw.cfg
OBJS = header.o bank00.o bank01.o bank02.o bank03.o bank04.o bank05.o bank06.o bank07.o bank08.o bank09.o bank10.o bank11.o bank12.o bank13.o bank14.o bank15.o chr.o
ROM = ../rom/lotw.nes
OUT = build/lotw.nes

.PHONY: all verify clean

all: $(OUT)

build:
	mkdir -p build

%.o: %.s
	$(AS) -o $@ $<

$(OUT): $(OBJS) $(CFG) | build
	$(LD) -C $(CFG) -o $(OUT) $(OBJS)

verify: $(OUT)
	@a=$$(sha256sum $(OUT) | cut -d' ' -f1); \
	 b=$$(sha256sum $(ROM) | cut -d' ' -f1); \
	 if [ "$$a" = "$$b" ]; then echo "MATCH: $$a"; \
	 else echo "MISMATCH"; echo " built: $$a"; echo " rom:   $$b"; \
	   cmp $(OUT) $(ROM) || true; exit 1; fi

clean:
	rm -f *.o $(OUT)
