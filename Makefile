MAKEFILE_PATH		:= $(realpath $(firstword $(MAKEFILE_LIST)))
GIT_ROOT		:= $(shell dirname $(MAKEFILE_PATH))
SRC_ROOT		:= $(GIT_ROOT)/src
MANPAGE_DIR		:= $(GIT_ROOT)/man
ORIGINAL_MANPATH	:= $(MANPATH)
TARGET_MANPATH		:= $(HOME)/opt/man
export MANPATH		:= $(MANPATH):$(MANPAGE_DIR)


env: man check-man
	@printf '\nexport MANPATH=%s\n' "$(ORIGINAL_MANPATH):$(MANPAGE_DIR)"

man: purge $(MANPAGE_DIR)
	cargo run generate-man-pages $(MANPAGE_DIR)

check-man:
	man git-br
	man git-remotes

install: man
	@rsync -HUNXLAuprogtv --ignore-errors --fsync --mkpath --progress $(MANPAGE_DIR)/ $(TARGET_MANPATH)/

purge:
	@rm -rf $(MANPAGE_DIR)


$(TARGET_MANPATH) $(MANPAGE_DIR):
	mkdir -p $@

.PHONY: man purge check-man install env
