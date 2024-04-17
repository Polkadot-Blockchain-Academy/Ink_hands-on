# Assignment

There is an on-chain faucet that gives 1 unit of a token to every requesting account, but just once.

Here is how it works:
Upon calling into the `receiveRequest` method of the `Multisig` contract, an off-chain `Signer` listens to the events, verifies the request, signs it and calls the `transfer` method.

As part of the `transfer` call the `Multisig` contract verifies the signatures and if they were signed by the key corresponding to the public key of the signers, the calling address receives a token from the faucet.

Unfortuantely, the authors of the contract left some vulnerabilities in it!
In this assignment you get to be the blackhat and employ your hacking skills to drain the multisig of it's funds.

How you do it is entirely up to you!

## Practical informations

To test your ideas:

1. You can run a one-node chain with `make devnet`.
2. You can start the offchain component with `make run-signer`. It will compile and deploy the contracts before launching the process.

Additionally:

1. Your solution needs to drain ALL of the tokens from the vulnerable contract, leaving a balance of 0.
2. The scripts testing it will use a different keypair than the one in the development setup!
