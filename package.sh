#!/bin/sh

set -e

full_name="$(cargo read-manifest | jq -r '"\(.name)-\(.version)"')"

cargo package
cd target/package

rm -fr "$full_name"
tar -xzf "$full_name.crate"

tar -cjf "$full_name.tar.bz2" "$full_name"
tar -czf "$full_name.tar.gz" "$full_name"
tar -cJf "$full_name.tar.xz" "$full_name"
zip -rq "$full_name.zip" "$full_name"
