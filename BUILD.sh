#!env /bin/bash

set -e

# if .env file exists, load it
if [ -f .env ]; then
    set -o allexport
    source .env
    set +o allexport
fi

YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NORMAL='\033[0m'
GREEN='\033[0;32m'

# comma separated list of features to use for the container build
EXTRA_FEATURES="${EXTRA_FEATURES:-}"

echo -e "${YELLOW}EXTRA_FEATURES: ${EXTRA_FEATURES}${NORMAL}\n"

# test if tools are installed
if ! command -v cargo > /dev/null 2>&1; then
    echo "${RED}Cargo is not installed. Please install it first.${NORMAL}"
    exit 1
fi

if ! command -v docker > /dev/null 2>&1; then
    echo "${RED}Docker is not installed. Please install it first.${NORMAL}"
    exit 1
fi

echo -e "${BLUE}Testing...${NORMAL}"
cargo test --all --all-features || (echo -e "$RED [Tests failed] $NORMAL" && exit 1)

echo -e "${BLUE}Building...${NORMAL}"
cargo build --all --all-features || (echo -e "$RED [Build failed] $NORMAL" && exit 1)

echo -e "${BLUE}Testing...${NORMAL}"
cargo test --all --no-default-features || (echo -e "$RED [Tests (no default) failed] $NORMAL" && exit 1)

echo -e "${BLUE}Building...${NORMAL}"
cargo build --all --no-default-features || (echo -e "$RED [Build (no default) failed] $NORMAL" && exit 1)

echo -e "${BLUE}Checking...${NORMAL}"
cargo check --all --all-features --tests --benches --examples || (echo -e "$RED [Check failed] $NORMAL" && exit 1)

echo -e "${BLUE}Clippying...${NORMAL}"
cargo clippy --all --all-features --tests --benches --examples -- -D clippy::all || (echo -e "$RED [Clippy failed] $NORMAL" && exit 1)

echo -e "${BLUE}Formatting...${NORMAL}"
cargo +nightly fmt --all -- --check || (echo -e "$RED [Format failed] $NORMAL" && exit 1)

echo -e "${BLUE}Licensing...${NORMAL}"
cargo deny check || (echo -e "$RED [License check failed] $NORMAL" && exit 1)

echo -e "${BLUE}Udeps...${NORMAL}"
cargo +nightly udeps || (echo -e "$RED [Udep failed] $NORMAL" && exit 1)

echo -e "${BLUE}Benchmarking...${NORMAL}"
cargo criterion --all --features=unstable

if [ -e "Dockerfile" ]; 
then
    echo -e "${BLUE}Build containers...${NORMAL}"

    docker build --target gcp-vertex-ai-generative-core -t gcp-vertex-ai-generative-core --build-arg EXTRA_FEATURES="${EXTRA_FEATURES}" . || (echo -e "$RED [Container build failed] $NORMAL" && exit 1)

fi

echo -e "$GREEN === OK === $NORMAL"
exit 0
