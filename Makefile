BOLD_GREEN = \033[1;32m
BOLD_YELLOW = \033[1;33m
BOLD_RED = \033[1;31m
BOLD_BLUE = \033[1;34m
NO_COLOR = \033[0m

%:
	@echo "$(BOLD_BLUE)Running: cargo run --bin $@$(NO_COLOR)"
	@cargo run --bin $@

help:
	@echo "$(BOLD_GREEN)Usage:$(NO_COLOR)"
	@echo "$(BOLD_YELLOW)  make <target>  # Run a specific binary target$(NO_COLOR)"
	@echo "$(BOLD_YELLOW)  make all           # Show this help message$(NO_COLOR)"

all: help

optimize:
	@echo "$(BOLD_GREEN)Optimizing...🚀$(NO_COLOR)"
	cargo run -- opt-level="3"

build:
	@echo "$(BOLD_GREEN)Building...🏗️$(NO_COLOR)"
	cargo build

clean:
	@echo "$(BOLD_RED)Cleaning up...🗑️$(NO_COLOR)"
	rm -rf target

.PHONY: help all optimize build clean