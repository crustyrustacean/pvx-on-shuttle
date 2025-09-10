# justfile commands

# use PowerShell instead of sh:
set shell := ["powershell.exe", "-c"]

# format code
fmt:
  cargo fmt

# clean the project
clean:
  cargo clean

# for local development
dev:
  cargo px build; shuttle run

# build for release
build:
  cargo px build --release

# for local testing
test:
  cargo px test

# update cargo-hakari
update:
  cargo hakari generate; cargo hakari manage-deps -y
