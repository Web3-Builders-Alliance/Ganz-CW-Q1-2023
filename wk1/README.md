# WBA CosmWasm Week 1

Add this funcionality to Counter contract
- Decrement
- IncrementBy
- DecrementBy
- ReflectFunds
    - send funds to an address, 
    - set a callback to reflect the information (amount sent, address)


## Code Journal Questions

1. *What are the concepts (borrowing, ownership, vectors etc)*

- Concepts include actors, message passing

2. *What is the organization?*

- Organized into main contract, state management, messages, error handling,

3. *What is the contract doing? What is the mechanism?*

- Its a Counter contract, with ability to increment, decrement by values, and reset counter.

- This is done by sending messages to modify the state.

4. *How could it be better? More efficient? Safer?*
