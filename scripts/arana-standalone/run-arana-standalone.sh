#!/usr/bin/env bash
set -e

CLEAN=${CLEAN:-false}
# Parse arguments for the script

while [[ $# -gt 0 ]]; do
    key="$1"
    case $key in
        -c|--clean)
            CLEAN=true
            shift # past argument
            ;;
        *)    # unknown option
            shift # past argument
            ;;
    esac
done

pushd .

# Check if we should clean the tmp directory
# if [ "$CLEAN" = true ]; then
#   echo "Cleaning tmp directory"
#   rm -rf ./tmp
# fi

# The following line ensure we run from the project root
PROJECT_ROOT=$(git rev-parse --show-toplevel)
cd "$PROJECT_ROOT"

echo "*** Start Webb DKG Node ***"
./target/release/tangle-standalone-node --base-path=/tmp/standalone/dkg-lion -lerror \
  --name dkg-lion \
  --chain "./resources/arana-standalone-raw.json" \
  --validator \
  --rpc-cors all \
  --rpc-port 9933 \
  --port 30304 \
  --ws-port 9944 &
./target/release/tangle-standalone-node --base-path=/tmp/standalone/dkg-tiger \
  --name dkg-tiger \
  --chain "./resources/arana-standalone-raw.json" \
  --validator \
  --rpc-cors all \
  --rpc-port 9934 \
  --port 30305 \
  --ws-port 9945 &
./target/release/tangle-standalone-node --base-path=/tmp/standalone/dkg-lynx \
    --name dkg-lynx \
    --chain "./resources/arana-standalone-raw.json" \
    --validator \
    --ws-port 9946 \
    --rpc-port 9935 \
    --rpc-cors all \
    --port 30306 \
    -ldkg=debug \
    -ldkg_gadget::worker=debug \
    -lruntime::dkg_metadata=debug \
    -ldkg_metadata=debug \
    -lruntime::dkg_proposal_handler=debug \
    -lruntime::offchain=debug \
    -ldkg_proposal_handler=debug \
    --charlie
popd