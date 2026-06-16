ChainSubscription Hub

Project Title

Campus Marketplace

Project Description

Campus Marketplace is a decentralized smart contract platform designed to facilitate secure peer to peer trading among college students within a campus ecosystem. Built using Soroban on the Stellar blockchain, the platform employs a trustless Escrow mechanism to ensure safe transactions, transparent item listings, and verified delivery confirmations without relying on centralized campus intermediaries.

Project Vision

The vision of Campus Marketplace is to empower student communities with a secure, transparent, and fee free internal economy. By leveraging blockchain technology, the platform eliminates the risks of fraud and non payment in student to student transactions, fostering a high trust digital environment for trading textbooks, electronics, and personal items using a dedicated campus utility token.

Key Features

Tokenized Economy Initialization: Setup and bind a specific campus utility token to the marketplace as the native payment method for all transactions.

Product Listing Management: Students can list products for sale by specifying a price in campus tokens, secured by on chain cryptographic authentication.

Escrow and Funds Locking: When a buyer purchases an item, funds are securely locked within the smart contract rather than being transferred directly to the seller, eliminating counterparty risk.

Verified Delivery Confirmation: Funds are only released and disbursed to the seller once the buyer explicitly confirms the successful physical receipt of the product.

Self Purchase Prevention: Built in security rules prevent sellers from manipulating listings or buying their own products.

Immutable Ledger and State Isolation: Utilizes Soroban instance and persistent storage models to optimize state history and maintain auditable product records.

Usage Instructions

Initialize Contract: Deploy the contract and bind the official campus token contract address to establish the marketplace currency.

List Products: Sellers call the listing function with the product details and price. The contract auto increments and assigns a unique Product ID.

Purchase and Lock Funds: Buyers initiate a purchase by specifying the Product ID. The contract automatically pulls the token amount from the buyer wallet into the escrow storage.

Confirm Delivery: Upon physical exchange of the item, the buyer signs a delivery confirmation, triggering the contract to release the locked funds to the seller.

Query Inventory: Anyone can publicly query the marketplace state to check product details, pricing, ownership, and transaction statuses.

Future Scope

Reputation and Rating System: Implement on chain student feedback mechanisms to score reliable buyers and sellers.

Dispute Resolution Oracle: Introduce a decentralized student court or admin arbitration system to handle disputed deliveries or refunds.

Multi Item Categories and Images: Support batch listings, categories, and decentralized metadata links for product photos.

Auction and Bidding Mechanism: Allow students to place active bids and host timed auctions for high demand items.

Interactive Web UI Dashboard: Build a responsive frontend application connecting seamlessly with Freighter wallet.

Technology Stack

Rust and Soroban SDK for writing highly optimized, typesafe, and secure smart contracts.

Stellar Blockchain for fast, low cost, and immutable ledger state management.

Checks Effects Interactions Pattern: Applied to contract methods to strictly guarantee re entrancy protection during fund transfers.

Contribution

Community contributions, bug reports, and optimizations from blockchain developers and student innovators are highly welcomed. Please feel free to fork the repository and submit pull requests for review.

License

This project is licensed under the MIT License.

Contract Detail

Contract ID: Please insert your contract address here after deployment