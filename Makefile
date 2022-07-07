# SPDX-License-Identifier: GPL-2.0

KDIR ?= /opt/linux-rust/linux

default:
	$(MAKE) -C $(KDIR) M=$$PWD modules LLVM=1

clean:
	$(MAKE) -C $(KDIR) M=$$PWD clean