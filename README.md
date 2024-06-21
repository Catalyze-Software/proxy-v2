# Proxy canister

## Reward mechanism

User activity - > Reward buffer -> Periodically process buffer -> Send to reward canister

User activity is logged from several places within proxy  
Reward buffer is stored in `reward_storage.rs`  
Timer to process buffer in `reward_buffer.rs`  
Buffer processing logic in `reward_bufferlogic.rs`

Public calls in `reward_calls.rs`
