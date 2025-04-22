"""
Natural Language Processing Generator for MCPMeta Creator Platform

This module provides AI-driven game generation from natural language descriptions.
It transforms user inputs into structured game elements for MCP modules.
"""

import os
import json
import torch
import numpy as np
from transformers import GPT2LMHeadModel, GPT2Tokenizer
from diffusers import StableDiffusionPipeline

class NLPGameGenerator:
    def __init__(self, config_path="./config.json"):
        """
        Initialize the NLP Game Generator
        
        Args:
            config_path: Path to configuration JSON
        """
        # Load configuration
        with open(config_path, 'r') as f:
            self.config = json.load(f)
            
        # Initialize text generation model
        self.tokenizer = GPT2Tokenizer.from_pretrained(self.config['model_paths']['text_generation'])
        self.text_model = GPT2LMHeadModel.from_pretrained(self.config['model_paths']['text_generation'])
        
        # Initialize asset generation model (if GPU available)
        if torch.cuda.is_available():
            self.asset_model = StableDiffusionPipeline.from_pretrained(
                self.config['model_paths']['asset_generation']
            ).to("cuda")
        else:
            self.asset_model = None
            print("Warning: GPU not available, asset generation disabled")
            
        # Load templates
        self.templates = self._load_templates()
        
        print("NLP Game Generator initialized successfully")
    
    def _load_templates(self):
        """Load game templates from the templates directory"""
        templates = {}
        templates_dir = self.config['paths']['templates']
        
        for category in os.listdir(templates_dir):
            category_path = os.path.join(templates_dir, category)
            if os.path.isdir(category_path):
                templates[category] = []
                for template_file in os.listdir(category_path):
                    if template_file.endswith('.json'):
                        with open(os.path.join(category_path, template_file), 'r') as f:
                            templates[category].append(json.load(f))
        
        return templates
    
    def generate_game(self, description, category="auto", complexity=0.7):
        """
        Generate a complete game from natural language description
        
        Args:
            description: User's natural language description of desired game
            category: Game category or "auto" for detection
            complexity: Complexity level (0.0-1.0)
            
        Returns:
            Dictionary containing MCP module components
        """
        print(f"Generating game from description: {description}")
        
        # Detect category if set to auto
        if category == "auto":
            category = self._detect_category(description)
            print(f"Detected category: {category}")
        
        # Generate game structure
        game_structure = self._generate_game_structure(description, category, complexity)
        
        # Generate game elements
        terrain = self._generate_terrain(game_structure['terrain_description'])
        characters = self._generate_characters(game_structure['character_descriptions'])
        logic = self._generate_logic(game_structure['logic_description'], complexity)
        interactions = self._generate_interactions(game_structure['interaction_description'])
        
        # Combine into MCP module format
        module = {
            "name": game_structure['name'],
            "description": game_structure['description'],
            "category": category,
            "layers": {
                "terrain": terrain,
                "character": characters,
                "logic": logic,
                "interaction": interactions
            },
            "metadata": {
                "tags": game_structure['tags'],
                "complexity": complexity,
                "generated": True,
                "originalDescription": description
            }
        }
        
        return module
    
    def _detect_category(self, description):
        """Detect game category from description"""
        # Simplified detection logic - would use more sophisticated NLP in production
        categories = ["rpg", "sandbox", "adventure", "competitive", "educational", "misc"]
        
        # Generate category prediction using text model
        inputs = self.tokenizer(f"Game category for: {description}\nCategory:", return_tensors="pt")
        outputs = self.text_model.generate(
            inputs["input_ids"],
            max_length=inputs["input_ids"].shape[1] + 10,
            temperature=0.7,
            num_return_sequences=1
        )
        
        prediction = self.tokenizer.decode(outputs[0][inputs["input_ids"].shape[1]:])
        
        # Match with valid categories
        for category in categories:
            if category.lower() in prediction.lower():
                return category
        
        # Default to misc if no match
        return "misc"
    
    def _generate_game_structure(self, description, category, complexity):
        """
        Generate overall game structure from description
        """
        prompt = f"""
        Generate a game design based on this description: "{description}"
        Category: {category}
        Complexity: {complexity}
        
        Game Design:
        """
        
        inputs = self.tokenizer(prompt, return_tensors="pt")
        outputs = self.text_model.generate(
            inputs["input_ids"],
            max_length=inputs["input_ids"].shape[1] + 500,
            temperature=0.8,
            num_return_sequences=1
        )
        
        generated_text = self.tokenizer.decode(outputs[0][inputs["input_ids"].shape[1]:])
        
        # Parse the generated text into structured format
        # In production, this would use more sophisticated parsing
        lines = generated_text.split('\n')
        
        structure = {
            "name": lines[0] if lines and lines[0] else f"Generated {category.capitalize()} Game",
            "description": description,
            "terrain_description": "A vibrant game world",
            "character_descriptions": ["Main character", "NPC 1", "Enemy 1"],
            "logic_description": "Basic game mechanics",
            "interaction_description": "Simple UI and interactions",
            "tags": [category, "generated", "ai"]
        }
        
        return structure
    
    def _generate_terrain(self, terrain_description):
        """Generate terrain layer components"""
        # Simplified implementation - would be more sophisticated in production
        return [
            {
                "id": "terrain_main",
                "type": "mesh",
                "data": "terrain_main.glb",
                "metadata": {
                    "description": terrain_description
                }
            },
            {
                "id": "skybox",
                "type": "mesh",
                "data": "skybox.glb",
                "metadata": {
                    "description": "Game skybox"
                }
            }
        ]
    
    def _generate_characters(self, character_descriptions):
        """Generate character layer components"""
        characters = []
        
        for i, description in enumerate(character_descriptions):
            characters.append({
                "id": f"character_{i}",
                "type": "model" if i == 0 else "npc",
                "data": f"character_{i}.glb",
                "metadata": {
                    "description": description
                }
            })
            
        return characters
    
    def _generate_logic(self, logic_description, complexity):
        """Generate logic layer components"""
        # Complexity affects the number and sophistication of logic elements
        num_elements = max(1, int(5 * complexity))
        
        logic_elements = []
        
        for i in range(num_elements):
            logic_elements.append({
                "id": f"logic_{i}",
                "type": "script",
                "data": f"logic_{i}.js",
                "language": "javascript",
                "metadata": {
                    "description": f"Game logic component {i}"
                }
            })
            
        return logic_elements
    
    def _generate_interactions(self, interaction_description):
        """Generate interaction layer components"""
        return [
            {
                "id": "main_ui",
                "type": "ui",
                "data": "main_ui.json",
                "metadata": {
                    "description": "Main game UI"
                }
            },
            {
                "id": "dialog_system",
                "type": "dialog",
                "data": "dialogs.json",
                "metadata": {
                    "description": "Character dialog system"
                }
            }
        ]

# Example usage
if __name__ == "__main__":
    generator = NLPGameGenerator()
    game = generator.generate_game("A cyberpunk city with hoverbike racing")
    print(json.dumps(game, indent=2)) 