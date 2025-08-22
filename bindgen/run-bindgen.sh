#!/bin/sh

set -e

cd "$(dirname "$0")"

bindgen wrapper.h -o ../src/bindings.rs \
	--disable-name-namespacing \
	--no-doc-comments \
	--no-prepend-enum-name \
	--use-core \
	--ctypes-prefix libc \
	--allowlist-function "FLAC_.*" \
	--allowlist-type "FLAC_.*" \
	--allowlist-var "FLAC_.*" \
	--blocklist-type FILE \
	--blocklist-type "_IO_.*" \
	--rust-target 1.65 \
	--
