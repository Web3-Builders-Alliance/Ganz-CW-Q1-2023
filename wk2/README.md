# _WIP, DOES NOT COMPILE YET_

# WBA CosmWasm Week 2

Cluster 2 Challenge 😈

You need two actors, Sender and Receiver.

The Sender:
- Receives native tokens from anyone and forwards them to the Receiver.
- Stores how much tokens have been received/forwarded, which can be returned in a Query.

The Receiver:
- Stores the received tokens until the owner of the contract claims them.
- The owner can claim part of the tokens held by the Receiver, or all at once.

Optional:
- The Sender gets notified when the Receiver has transferred the funds.
- The Sender gets notified when the Receiver funds have been claimed by its owner.

Assume happy paths, though minor validations are expected. Pass any relevant information you need on the messages.



## Code Journal Questions

1. *What are the concepts (borrowing, ownership, vectors etc)*

- Concepts include actors, message passing

2. *What is the organization?*

- Organized into main contract, state management, messages, error handling,

3. *What is the contract doing? What is the mechanism?*

- Its a Counter contract, with ability to increment, decrement by values, and reset counter.

- This is done by sending messages to modify the state.

4. *How could it be better? More efficient? Safer?*
