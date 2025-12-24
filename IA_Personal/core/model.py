"""
Modelo Transformer Ligero para IA-Personal
===========================================
Author: Eddi Andreé Salazar Matos
Made with ❤️ in Peru 🇵🇪
"""

from typing import List, Dict, Tuple
from dataclasses import dataclass

import numpy as np


@dataclass
class ModelConfig:
    """Configuración del modelo."""
    vocab_size: int = 10000
    embed_dim: int = 128
    num_heads: int = 8
    hidden_dim: int = 256
    num_layers: int = 2
    max_seq_len: int = 256
    use_float16: bool = True


class LightTransformer:
    """Transformer ligero optimizado para bajo RAM."""
    
    def __init__(self, config: ModelConfig, vocab_size: int):
        self.config = config
        self.vocab_size = vocab_size
        self.dtype = np.float16 if config.use_float16 else np.float32
        
        # Embeddings
        self.embeddings = np.random.randn(vocab_size, config.embed_dim).astype(self.dtype) * 0.02
        
        # Capas transformer
        self.layers = []
        for _ in range(config.num_layers):
            layer = {
                "W_q": np.random.randn(config.embed_dim, config.embed_dim).astype(self.dtype) * 0.02,
                "W_k": np.random.randn(config.embed_dim, config.embed_dim).astype(self.dtype) * 0.02,
                "W_v": np.random.randn(config.embed_dim, config.embed_dim).astype(self.dtype) * 0.02,
                "W_o": np.random.randn(config.embed_dim, config.embed_dim).astype(self.dtype) * 0.02,
                "W1": np.random.randn(config.embed_dim, config.hidden_dim).astype(self.dtype) * 0.02,
                "W2": np.random.randn(config.hidden_dim, config.embed_dim).astype(self.dtype) * 0.02,
            }
            self.layers.append(layer)
        
        # Capa de salida
        self.output_proj = np.random.randn(config.embed_dim, vocab_size).astype(self.dtype) * 0.02
        
        # Cache para KV
        self.kv_cache: Dict[int, Tuple[np.ndarray, np.ndarray]] = {}
        
        self._calc_ram()
    
    def _calc_ram(self):
        """Calcula RAM usada."""
        bytes_per = 2 if self.config.use_float16 else 4
        
        embed_ram = self.vocab_size * self.config.embed_dim * bytes_per
        layer_ram = self.config.num_layers * (
            4 * self.config.embed_dim * self.config.embed_dim +
            2 * self.config.embed_dim * self.config.hidden_dim
        ) * bytes_per
        output_ram = self.config.embed_dim * self.vocab_size * bytes_per
        
        self.ram_mb = (embed_ram + layer_ram + output_ram) / (1024 * 1024)
    
    def forward(self, token_ids: List[int]) -> np.ndarray:
        """Forward pass."""
        # Embeddings
        safe_ids = [min(max(0, t), self.vocab_size - 1) for t in token_ids]
        x = self.embeddings[safe_ids]
        
        head_dim = self.config.embed_dim // self.config.num_heads
        
        for layer in self.layers:
            # Atención
            Q = x @ layer["W_q"]
            K = x @ layer["W_k"]
            V = x @ layer["W_v"]
            
            scores = Q @ K.T / np.sqrt(head_dim)
            
            # Máscara causal
            seq_len = len(token_ids)
            mask = np.triu(np.ones((seq_len, seq_len)) * -1e9, k=1)
            scores = scores + mask
            
            # Softmax
            exp_scores = np.exp(scores - np.max(scores, axis=-1, keepdims=True))
            weights = exp_scores / (np.sum(exp_scores, axis=-1, keepdims=True) + 1e-8)
            
            attn_out = weights @ V @ layer["W_o"]
            x = x + attn_out
            
            # FFN con GELU
            hidden = x @ layer["W1"]
            hidden = hidden * 0.5 * (1 + np.tanh(np.sqrt(2 / np.pi) * (hidden + 0.044715 * hidden**3)))
            ffn_out = hidden @ layer["W2"]
            x = x + ffn_out
        
        # Logits
        logits = x[-1] @ self.output_proj
        return logits
    
    def generate_token(self, token_ids: List[int], temperature: float = 0.7,
                       top_k: int = 50, top_p: float = 0.9,
                       repetition_penalty: float = 1.1) -> int:
        """Genera el siguiente token."""
        logits = self.forward(token_ids)
        
        # Penalización por repetición
        for tid in set(token_ids[-20:]):
            if 0 <= tid < len(logits):
                logits[tid] /= repetition_penalty
        
        # Temperatura
        logits = logits / max(temperature, 0.1)
        
        # Top-k
        if top_k > 0:
            indices = np.argsort(logits)[-top_k:]
            mask = np.ones_like(logits) * -1e9
            mask[indices] = 0
            logits = logits + mask
        
        # Softmax
        exp_logits = np.exp(logits - np.max(logits))
        probs = exp_logits / np.sum(exp_logits)
        
        # Top-p (nucleus sampling)
        if top_p < 1.0:
            sorted_indices = np.argsort(probs)[::-1]
            cumsum = np.cumsum(probs[sorted_indices])
            cutoff_idx = np.searchsorted(cumsum, top_p) + 1
            keep_indices = sorted_indices[:cutoff_idx]
            
            new_probs = np.zeros_like(probs)
            new_probs[keep_indices] = probs[keep_indices]
            probs = new_probs / (np.sum(new_probs) + 1e-8)
        
        # Muestrear
        return int(np.random.choice(len(probs), p=probs.astype(np.float64)))
    
    def get_embedding(self, token_ids: List[int]) -> np.ndarray:
        """Obtiene embeddings para tokens."""
        safe_ids = [min(max(0, t), self.vocab_size - 1) for t in token_ids]
        return self.embeddings[safe_ids]
    
    def similarity(self, tokens1: List[int], tokens2: List[int]) -> float:
        """Calcula similitud entre dos secuencias de tokens."""
        emb1 = np.mean(self.get_embedding(tokens1), axis=0)
        emb2 = np.mean(self.get_embedding(tokens2), axis=0)
        
        dot = np.dot(emb1, emb2)
        norm = np.linalg.norm(emb1) * np.linalg.norm(emb2)
        return float(dot / (norm + 1e-8))
