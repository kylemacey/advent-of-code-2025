.PHONY: help new run test release build clean all

# Default day if not specified
DAY ?= 01

# Format day with leading zero
DAY_FORMATTED := $(shell printf "%02d" $(DAY))

help:
	@echo "🎄 Advent of Code 2025 - Makefile Commands"
	@echo ""
	@echo "Usage:"
	@echo "  make new [DAY=N]      Scaffold a new day (defaults to today)"
	@echo "  make run [DAY=N]      Run solution for day N (default: 1)"
	@echo "  make test [DAY=N]     Run tests for day N"
	@echo "  make release [DAY=N]  Run with optimizations"
	@echo "  make build            Build all days"
	@echo "  make clean            Clean build artifacts"
	@echo "  make all              Build everything"
	@echo ""
	@echo "Examples:"
	@echo "  make new              # Scaffold today"
	@echo "  make new DAY=5        # Scaffold day 5"
	@echo "  make run DAY=3        # Run day 3"
	@echo "  make test DAY=7       # Test day 7"

new:
	@cargo run --quiet --bin scaffold $(if $(filter-out 01,$(DAY_FORMATTED)),$(DAY))

run:
	@cargo run --bin day$(DAY_FORMATTED)

test:
	@cargo test --bin day$(DAY_FORMATTED)

release:
	@cargo run --release --bin day$(DAY_FORMATTED)

build:
	@cargo build --workspace

clean:
	@cargo clean

all: build

