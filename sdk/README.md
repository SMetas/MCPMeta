# MCPMeta SDK

Software Development Kits for integrating with the MCPMeta ecosystem.

## Available SDKs

- **JavaScript/TypeScript**: For web and Node.js applications
- **Rust**: For high-performance applications and blockchain integration

## Features

- MCP Protocol module creation and parsing
- Connection to Meta Public Chain
- Distribution Platform integration
- Wallet connectivity
- Asset conversion utilities

## JavaScript/TypeScript SDK

```js
// Example: Create an MCP module
import { MCPModule } from '@mcpmeta/sdk';

const module = new MCPModule({
  name: 'My Game Module',
  category: 'RPG',
  assets: [...],
  logic: [...],
});

// Export to MCP format
await module.exportToMCP();

// Upload to distribution platform
await module.publish({
  price: 10, // $META
  issuanceType: 'paid'
});
```

## Rust SDK

```rust
// Example: Parse a traditional game asset
use mcpmeta_sdk::parser;
use mcpmeta_sdk::module::Module;

fn main() {
    let minecraft_mod = fs::read("./my_minecraft_mod.jar").unwrap();
    
    // Parse into MCP module
    let module = parser::minecraft::parse_mod(&minecraft_mod).unwrap();
    
    // Export and publish
    module.export_to_mcp().unwrap();
    module.publish(10, IssuanceType::Paid).unwrap();
}
```

## Installation

### JavaScript/TypeScript

```
npm install @mcpmeta/sdk
```

### Rust

```
cargo add mcpmeta-sdk
``` 