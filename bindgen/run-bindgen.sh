#!/bin/sh

set -e

cd "$(dirname "$0")"

bindgen wrapper.h -o ../src/bindings.rs \
	--disable-name-namespacing \
	--no-doc-comments \
	--no-prepend-enum-name \
	--use-core \
	--ctypes-prefix libc \
	--size_t-is-usize \
	--whitelist-function "FLAC_.*" \
	--whitelist-type "FLAC_.*" \
	--whitelist-var "FLAC_.*" \
	--blacklist-type FILE \
	--blacklist-type "_IO_.*" \
	--
