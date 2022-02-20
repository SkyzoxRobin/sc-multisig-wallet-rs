# config
PROXY="https://devnet-gateway.elrond.com"
CHAIN="D"
BYTECODE="output/multisig.wasm"
PEM="../../no-name.pem"

# interaction 

deploy() {
    quorum="0x$(printf '%x' 2)"

    # board member
    board_member_1="0x$(erdpy wallet bech32 --decode erd17frjmjj93klz0w9gh0xj76atp8lk7h2k4uhawe8yp5g0rusprn0qm7n49c)"
    board_member_2="0x$(erdpy wallet bech32 --decode erd17frjmjj93klz0w9gh0xj76atp8lk7h2k4uhawe8yp5g0rusprn0qm7n49c)"

    erdpy --verbose contract deploy --bytecode="$BYTECODE" --recall-nonce \
        --pem=$PEM \
        --gas-limit=599000000 \
        --proxy=$PROXY --chain=$CHAIN \
        --arguments $quorum $board_member_1 $board_member_2 \
        --outfile="deploy-devnet.interaction.json" --send || return

    TRANSACTION=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emitted_tx']['address']")

    erdpy data store --key=address-devnet --value=${ADDRESS}
    erdpy data store --key=deployTransaction-devnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

depositEgld() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce \
        --pem=${PEM} \
        --proxy=${PROXY} --chain=${CHAIN} \
        --gas-limit=10000000 \
        --value=100000000000000000 \ # 1 EGLD
        --function="deposit" \
        --arguments $to \
        --send || return
}

depositEsdt() {
    token="0x$(echo -n 'ticker' | xxd -p -u | tr -d '\n')"
    amount="0x$(printf '%x' AMOUNT)"
    method="0x$(echo -n 'deposit' | xxd -p -u | tr -d '\n')"
    
        erdpy --verbose contract call ${ADDRESS} --recall-nonce \
        --pem=${PEM} \
        --proxy=${PROXY} --chain=${CHAIN} \
        --gas-limit=10000000 \
        --function="ESDTTransfer" \
        --arguments $token $amount $method \
        --send || return
}

proposeAddBoardMember() {
    new_board_member="0x$(erdpy wallet bech32 --decode erd17frjmjj93klz0w9gh0xj76atp8lk7h2k4uhawe8yp5g0rusprn0qm7n49c)"

    erdpy --verbose contract call ${ADDRESS} --recall-nonce \
        --pem=${PEM} \
        --proxy=${PROXY} --chain=${CHAIN} \
        --gas-limit=10000000 \
        --function="proposeAddBoardMember" \
        --arguments $new_board_member \
        --send || return
}

proposeAddProposer() {
    new_proposer="0x$(erdpy wallet bech32 --decode erd17frjmjj93klz0w9gh0xj76atp8lk7h2k4uhawe8yp5g0rusprn0qm7n49c)"

    erdpy --verbose contract call ${ADDRESS} --recall-nonce \
        --pem=${PEM} \
        --proxy=${PROXY} --chain=${CHAIN} \
        --gas-limit=10000000 \
        --function="proposeAddProposer" \
        --arguments $new_proposer \
        --send || return
}

proposeChangeQuorum() {
    new_quorum="0x$(printf '%x' 3)"

    erdpy --verbose contract call ${ADDRESS} --recall-nonce \
        --pem=${PEM} \
        --proxy=${PROXY} --chain=${CHAIN} \
        --gas-limit=10000000 \
        --function="proposeChangeQuorum" \
        --arguments $new_quorum \
        --send || return
}

proposeRemoveUser() {
    user_to_remove="0x$(erdpy wallet bech32 --decode erd17frjmjj93klz0w9gh0xj76atp8lk7h2k4uhawe8yp5g0rusprn0qm7n49c)"

    erdpy --verbose contract call ${ADDRESS} --recall-nonce \
        --pem=${PEM} \
        --proxy=${PROXY} --chain=${CHAIN} \
        --gas-limit=10000000 \
        --function="proposeRemoveUser" \
        --arguments $user_to_remove \
        --send || return
}

discardAction() {
    action_id="0x$(printf '%x' 2)"

        erdpy --verbose contract call ${ADDRESS} --recall-nonce \
        --pem=${PEM} \
        --proxy=${PROXY} --chain=${CHAIN} \
        --gas-limit=10000000 \
        --function="discardAction" \
        --arguments $action_id \
        --send || return
}

proposeEgldTransfer() {
    to="0x$(erdpy wallet bech32 --decode erd17frjmjj93klz0w9gh0xj76atp8lk7h2k4uhawe8yp5g0rusprn0qm7n49c)"
    egld_amount="0x$(printf '%x' 100000000000000000)"

    erdpy --verbose contract call ${ADDRESS} --recall-nonce \
        --pem=${PEM} \
        --proxy=${PROXY} --chain=${CHAIN} \
        --gas-limit=10000000 \
        --function="proposeEgldTransfer" \
        --arguments $to $egld_amount \
        --send || return
}

proposeEsdtTransfer() {
    to="0x$(erdpy wallet bech32 --decode erd17frjmjj93klz0w9gh0xj76atp8lk7h2k4uhawe8yp5g0rusprn0qm7n49c)"
    token="0x$(echo -n 'TICKER' | xxd -p -u | tr -d '\n')"
    amount="0x$(printf '%x' AMOUNT)"

    erdpy --verbose contract call ${ADDRESS} --recall-nonce \
        --pem=$PEM \
        --proxy=${PROXY} --chain=${CHAIN} \
        --gas-limit=10000000 \
        --function="proposeEsdtTransfer" \
        --arguments $to $token $amount \
        --send || return
}

sign() {
    action_id="0x$(printf '%x' 2)"
    
    erdpy --verbose contract call ${ADDRESS} --recall-nonce \
        --pem=${PEM} \
        --proxy=${PROXY} --chain=${CHAIN} \
        --gas-limit=10000000 \
        --function="sign" \
        --arguments $action_id \
        --send || return
}

unsign() {
    action_id="0x$(printf '%x' 2)"
    
    erdpy --verbose contract call ${ADDRESS} --recall-nonce \
        --pem=${PEM} \
        --proxy=${PROXY} --chain=${CHAIN} \
        --gas-limit=10000000 \
        --function="unsign" \
        --arguments $action_id \
        --send || return
}

performAction() {
    action_id="0x$(printf '%x' 2)"
    
    erdpy --verbose contract call ${ADDRESS} --recall-nonce \
        --pem=${PEM} \
        --proxy=${PROXY} --chain=${CHAIN} \
        --gas-limit=10000000 \
        --function="performAction" \
        --arguments $action_id \
        --send || return
}

# views : todo 
signed() {}
getQuorum() {}
getActionLastIndex() {}
getNumBoardMembers() {}
getNumProposers() {}
quorumReached() {}
getActionData() {}
getActionSignerCount() {}
getActionSigners() {}
getActionValidSignerCount() {}
getAllBoardMembers() {}
getAllProposers() {}
getPendingActionFullInfo() {}
userRole() {}