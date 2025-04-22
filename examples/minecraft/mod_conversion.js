/**
 * Minecraft Mod to MCP Module Conversion Example
 * 
 * This example demonstrates how to convert a Minecraft mod (Forge or Spigot)
 * into an MCP module that can be deployed to the Meta Public Chain.
 */

const fs = require('fs');
const path = require('path');
const { MCPModule, MCPParser } = require('../../protocol/src/mcp_core');

// Configuration
const config = {
  inputMod: './input/example_rpg_mod.jar',
  outputDir: './output',
  creatorWallet: 'Bxp7yhNYS4TxvV2Vw8NqGqUcbN3FZ2xpBg1ba94LPMav', // Solana wallet
  issuanceType: 'paid', // 'paid' or 'free'
  price: 15, // $META tokens (if paid)
  deployToChain: true,
  ipfsUpload: true
};

/**
 * Main conversion function
 */
async function convertMinecraftMod() {
  console.log(`Starting conversion of ${config.inputMod} to MCP module`);
  
  // 1. Read the Minecraft mod file
  const modBuffer = fs.readFileSync(config.inputMod);
  console.log(`Read ${modBuffer.length} bytes from mod file`);
  
  // 2. Parse the mod using MCP Parser
  const module = MCPParser.parseMinecraftMod(modBuffer);
  console.log(`Parsed mod as "${module.name}"`);
  
  // 3. Customize module metadata
  module.creator = config.creatorWallet;
  module.metadata.tags = ['minecraft', 'rpg', 'adventure'];
  module.metadata.issuanceType = config.issuanceType;
  module.metadata.price = config.price;
  module.metadata.sourceType = 'minecraft_mod';
  
  // 4. Validate module structure
  if (module.validate()) {
    console.log('Module validation successful');
  } else {
    console.error('Module validation failed');
    return;
  }
  
  // 5. Export as MCP JSON
  const moduleJson = module.toJSON();
  if (!fs.existsSync(config.outputDir)) {
    fs.mkdirSync(config.outputDir, { recursive: true });
  }
  
  const jsonPath = path.join(config.outputDir, 'module.json');
  fs.writeFileSync(jsonPath, JSON.stringify(moduleJson, null, 2));
  console.log(`Module JSON exported to ${jsonPath}`);
  
  // 6. Upload to IPFS (if configured)
  let ipfsCid = null;
  if (config.ipfsUpload) {
    ipfsCid = await module.uploadToIPFS();
    console.log(`Module content uploaded to IPFS: ${ipfsCid}`);
  }
  
  // 7. Register on Meta Public Chain (if configured)
  if (config.deployToChain) {
    const txHash = await module.registerOnChain({
      ipfsCid,
      price: config.issuanceType === 'paid' ? config.price : 0,
      isFreeTips: config.issuanceType === 'free'
    });
    
    console.log(`Module registered on Meta Public Chain: ${txHash}`);
    console.log(`Creator revenue share: 90%`);
    console.log(`Access URL: https://mcpmeta.io/modules/${module.id}`);
  }
  
  console.log('\nConversion completed successfully!');
  
  // Return information about the converted module
  return {
    moduleId: module.id,
    moduleName: module.name,
    ipfsCid,
    price: config.price,
    issuanceType: config.issuanceType
  };
}

/**
 * Example use
 */
(async () => {
  try {
    const result = await convertMinecraftMod();
    console.log('\nConversion Result:');
    console.log(result);
    
    console.log('\nNext Steps:');
    console.log('1. Visit https://mcpmeta.io/creator to customize your module');
    console.log('2. Promote your module on social media');
    console.log('3. Monitor revenue in your Creator Dashboard');
  } catch (error) {
    console.error('Conversion failed:', error);
  }
})(); 