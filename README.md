# Sushi Role

![image](https://github.com/CosmoSushi/Sushi-Role/assets/20056309/2c315631-6c55-406c-ae83-a3b3c90af2fb)


Smart Account from user onboarding to its management

# What is being achieved.

The SmartAccount module and corresponding CosmWasm contract provided by Aura Network, as well as the feegrant module, allow users to easily access projects on the Aura Network for free.
The business that pays for the user's gas can control who the user's smart account is used for.

- Users can open a smart account for free.
- Users can continue to use their free smart accounts for a variety of projects.
- In the event that a user's seed phrase is leaked, a new public key can be provided to protect the assets on the smart account.
- The Service Provider that pays for the user's gas can limit the functions that the user's smart account can perform.

## How does this work

We used AuraNetwork's SmartAccount and feegrant for our implementation.

### Gassless Onbording　Flow

<img width="751" alt="image" src="https://github.com/CosmoSushi/Sushi-Role/assets/20056309/c471dc0c-3b46-4573-9740-f1b0a992ee11">

1.Service Provider deploys the contract it wants users to use. (In this case, we have prepared a CW721 contract that anyone can mint.)

2.Through the UI, the user gives the Service Provider the address of the Smart Account he/she is about to open.

3.The Serivice Provider uses the freegant module to Approve the available gas costs for the user's account.

4.The user activates a smart account with the public key of user’s EOA account.

5.the user can create a smart account for free, as they consume the cost of gas that is Approved by feegrant.

6＆７ After opening a smart account, transactions can still be executed for free as long as the Service Provider has set up a feegrant Approve for that smart account.In this HackWasm Berlin, the NFT was issued from a smart account.

### Access Control

The Service Provider will continue to pay for the user's SmartAccount gas through the feegrant, but it is not desirable for the user to use the Smart Account in a way that it was not intended.

Suhi Role solves this problem by providing a SmartAccount that can perform only the functions desired by the Service Provider.

SmartAccount in SushiRole allows the registration of target addresses that can execute transactions from SmartAccount only from the Service Provider's account.
Like the function to set SpendLimit, we have prepared a function in Smart Account that only a specific account can execute. In this case, only the Service Provider account, not the Owner, can execute the function.

Address control refers to the address at which the Msg contained by the transaction is to be executed in preExecute when SmartAccount is executed, and if the transaction contains only Msg for allowed addresses, the transaction will be executed.

We are currently testing this feature.

The reference implementation can be found at [access-control contract](https://github.com/CosmoSushi/Sushi-Role/tree/access-control/contracts/access-control).

## Demo

We demonstrate "Feegrant" and "Minting NFT with SmartAccount" in this HackWasm.

The details of the procedure are published in Notion.

https://www.notion.so/Feegrant-sample-083c39ac89c2442881da58585228d5d6
