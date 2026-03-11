# Makefile for zsh-chat

INSTALL_DIR = /usr/local/bin
CLIENT_SCRIPT = client/zsh-chat.zsh
SYMLINK_NAME = zsh-chat

.PHONY: all help test install uninstall

all: help

help:
	@echo "zsh-chat Makefile"
	@echo ""
	@echo "Targets:"
	@echo "  test      Run the client script with some mock commands"
	@echo "  install   Symlink the script to $(INSTALL_DIR)/$(SYMLINK_NAME) (requires sudo)"
	@echo "  uninstall Remove the symlink from $(INSTALL_DIR)/$(SYMLINK_NAME) (requires sudo)"

test:
	./$(CLIENT_SCRIPT) version
	./$(CLIENT_SCRIPT) feed
	./$(CLIENT_SCRIPT) post "Test post from Makefile"

install:
	ln -s $(PWD)/$(CLIENT_SCRIPT) $(INSTALL_DIR)/$(SYMLINK_NAME)

uninstall:
	rm $(INSTALL_DIR)/$(SYMLINK_NAME)
