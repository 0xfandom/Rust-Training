// Ethereum txn -> revm (Rust EVM) -> Context -> Execution Result

// Who use revm :  Reth , Foundry , Hardhat

// What does revm actually do?
// You give it a transaction, it gives you back a result. Internally it:

// Takes the transaction input (sender, calldata, value, gas limit)
// Loads the smart contract bytecode from state
// Executes opcodes one by one — PUSH, ADD, CALL, SSTORE etc
// Tracks exact gas consumed at every step
// Applies all state changes (balance transfers, storage writes)
// Returns the result — logs, return data, gas used, any reverts

// The Inspector — built-in tracer
// One of revm's killer features. It enables users to extend logic, incorporate various context types, and offers built-in support for inspection. Paradigm
// The Inspector trait lets you hook into every opcode execution — perfect for building debuggers, gas profilers, coverage tools, or MEV simulators:



// revm and Alloy are sister libraries — both in the `bluealloy` GitHub org, both used by Reth. Alloy handles the **network layer** (connecting to nodes, sending transactions, reading chain data). revm handles the **execution layer** (actually running smart contract code locally). In a full Ethereum Rust stack you'd use both together:
// ```
// Alloy  →  fetches state from a node
// revm   →  simulates transactions against that state locally


// This is exactly how Foundry works under the hood — Alloy fetches the forked state, 
// revm runs your tests against it at full EVM speed without touching the actual network.


