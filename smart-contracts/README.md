# MCPMeta Smart Contracts

Smart contracts for the MCPMeta ecosystem, powering the economic system and NFT marketplace.

## Contract Categories

- **Solana**: Core economic and marketplace contracts
- **Bridge**: Cross-chain bridges for interoperability

## Solana Contracts

- **TokenContract**: $META token implementation
- **MarketplaceContract**: NFT trading and revenue distribution
- **ModuleRegistry**: MCP module registration and ownership
- **RevenueDistribution**: 90% creator, 10% platform split

## Bridge Contracts

- **SolanaEthereumBridge**: Bridge between Solana and Ethereum
- **SolanaPolygonBridge**: Bridge between Solana and Polygon
- **MetaverseBridge**: Special bridge for metaverse platform interoperability

## Security Features

- Audit-ready code with comprehensive tests
- Secure multi-signature requirements for admin functions
- Rate limiting for sensitive operations
- Permanent immutable records on-chain

## Development

```bash
# Install dependencies
npm install

# Compile contracts
npm run build

# Run tests
npm test

# Deploy to testnet
npm run deploy:testnet
```

## Audits

All contracts are audited by CertiK and other leading security firms before mainnet deployment. 