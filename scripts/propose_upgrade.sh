# bin/bash

dfx identity use catalyze_production
OWNER_IDENTITY=$(dfx identity whoami)
PEM_FILE="$(readlink -f "$HOME/.config/dfx/identity/${OWNER_IDENTITY}/identity.pem")"
DEVELOPER_NEURON_ID="37173462e82235788f2592e076b31cf0e8601eff16b2a8687b564589d867de36"

VERSION=$(grep '^version = ' src/proxy/Cargo.toml | sed -E 's/version = "(.*)"/\1/')
CHANGELOG="Bugfix"
WASM_PATH="wasm/proxy.wasm.gz"

bash scripts/build.sh
UPGRADE_CANISTER_ID="2jvhk-5aaaa-aaaap-ahewa-cai"

TITLE="Upgrade proxy canister to version $VERSION"
URL="https://github.com/Catalyze-Software/proxy/releases/tag/$VERSION"
SUMMARY=$CHANGELOG

quill sns make-upgrade-canister-proposal \
   --canister-ids-file scripts/sns_canister_ids.json  \
   --target-canister-id "${UPGRADE_CANISTER_ID}" \
   --wasm-path "${WASM_PATH}" \
   --summary "${SUMMARY}" \
   --pem-file "${PEM_FILE}" \
   --url "${URL}" \
   --title "${TITLE}" \
   "${DEVELOPER_NEURON_ID}" > msg.json
   
quill send --yes msg.json

rm msg.json