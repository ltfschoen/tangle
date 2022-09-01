#!/bin/sh

echo "****************** NODE-1 KEY INSERTION ******************"

./target/release/tangle-standalone-node key insert --base-path /tmp/standalone/dkg-tiger \
--chain "./resources/arana-standalone-raw.json" \
--scheme Sr25519 \
--suri "gown surprise mirror hotel cash alarm raccoon you frog rose midnight enter//webb//0" \
--key-type acco

./target/release/tangle-standalone-node key insert --base-path /tmp/standalone/dkg-tiger \
--chain "./resources/arana-standalone-raw.json" \
--scheme Sr25519 \
--suri "gown surprise mirror hotel cash alarm raccoon you frog rose midnight enter//webb//0//stash" \
--key-type acco

./target/release/tangle-standalone-node key insert --base-path /tmp/standalone/dkg-tiger \
--chain "./resources/arana-standalone-raw.json" \
--scheme Sr25519 \
--suri "gown surprise mirror hotel cash alarm raccoon you frog rose midnight enter//webb//0//aura" \
--key-type aura

./target/release/tangle-standalone-node key insert --base-path /tmp/standalone/dkg-tiger \
--chain "./resources/arana-standalone-raw.json" \
--scheme Ed25519 \
--suri "gown surprise mirror hotel cash alarm raccoon you frog rose midnight enter//webb//0//grandpa" \
--key-type gran

./target/release/tangle-standalone-node key insert --base-path /tmp/standalone/dkg-tiger \
--chain "./resources/arana-standalone-raw.json" \
--scheme Ecdsa \
--suri "gown surprise mirror hotel cash alarm raccoon you frog rose midnight enter//webb//0//dkg" \
--key-type wdkg

echo "****************** NODE-2 KEY INSERTION ******************"

./target/release/tangle-standalone-node key insert --base-path /tmp/standalone/dkg-lion \
--chain "./resources/arana-standalone-raw.json" \
--scheme Sr25519 \
--suri "gown surprise mirror hotel cash alarm raccoon you frog rose midnight enter//webb//1" \
--key-type acco

./target/release/tangle-standalone-node key insert --base-path /tmp/standalone/dkg-lion \
--chain "./resources/arana-standalone-raw.json" \
--scheme Sr25519 \
--suri "gown surprise mirror hotel cash alarm raccoon you frog rose midnight enter//webb//1//stash" \
--key-type acco

./target/release/tangle-standalone-node key insert --base-path /tmp/standalone/dkg-lion \
--chain "./resources/arana-standalone-raw.json" \
--scheme Sr25519 \
--suri "gown surprise mirror hotel cash alarm raccoon you frog rose midnight enter//webb//1//aura" \
--key-type aura

./target/release/tangle-standalone-node key insert --base-path /tmp/standalone/dkg-lion \
--chain "./resources/arana-standalone-raw.json" \
--scheme Ed25519 \
--suri "gown surprise mirror hotel cash alarm raccoon you frog rose midnight enter//webb//1//grandpa" \
--key-type gran

./target/release/tangle-standalone-node key insert --base-path /tmp/standalone/dkg-lion \
--chain "./resources/arana-standalone-raw.json" \
--scheme Ecdsa \
--suri "gown surprise mirror hotel cash alarm raccoon you frog rose midnight enter//webb//1//dkg" \
--key-type wdkg


echo "****************** NODE-3 KEY INSERTION ******************"

./target/release/tangle-standalone-node key insert --base-path /tmp/standalone/dkg-lynx \
--chain "./resources/arana-standalone-raw.json" \
--scheme Sr25519 \
--suri "gown surprise mirror hotel cash alarm raccoon you frog rose midnight enter//webb//2" \
--key-type acco

./target/release/tangle-standalone-node key insert --base-path /tmp/standalone/dkg-lynx \
--chain "./resources/arana-standalone-raw.json" \
--scheme Sr25519 \
--suri "gown surprise mirror hotel cash alarm raccoon you frog rose midnight enter//webb//2//stash" \
--key-type acco

./target/release/tangle-standalone-node key insert --base-path /tmp/standalone/dkg-lynx \
--chain "./resources/arana-standalone-raw.json" \
--scheme Sr25519 \
--suri "gown surprise mirror hotel cash alarm raccoon you frog rose midnight enter//webb//2//aura" \
--key-type aura

./target/release/tangle-standalone-node key insert --base-path /tmp/standalone/dkg-lynx \
--chain "./resources/arana-standalone-raw.json" \
--scheme Ed25519 \
--suri "gown surprise mirror hotel cash alarm raccoon you frog rose midnight enter//webb//2//grandpa" \
--key-type gran

./target/release/tangle-standalone-node key insert --base-path /tmp/standalone/dkg-lynx \
--chain "./resources/arana-standalone-raw.json" \
--scheme Ecdsa \
--suri "gown surprise mirror hotel cash alarm raccoon you frog rose midnight enter//webb//2//dkg" \
--key-type wdkg