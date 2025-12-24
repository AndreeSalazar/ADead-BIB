"""
Tokenizador Inteligente para IA-Personal
=========================================
Author: Eddi Andreé Salazar Matos
Made with ❤️ in Peru 🇵🇪
"""

import re
from typing import List, Dict, Tuple
from collections import Counter


class SmartTokenizer:
    """Tokenizador inteligente con vocabulario expandible."""
    
    PAD, EOS, UNK, BOS, SEP = 0, 1, 2, 3, 4
    
    def __init__(self, vocab_size: int = 15000):
        self.vocab_size = vocab_size
        self.vocab: Dict[str, int] = {}
        self.inv_vocab: Dict[int, str] = {}
        self.word_freq: Counter = Counter()
        
        self._init_vocab()
    
    def _init_vocab(self):
        """Inicializa vocabulario base."""
        # Tokens especiales
        special = ["<PAD>", "<EOS>", "<UNK>", "<BOS>", "<SEP>", "<MASK>", "<USER>", "<AI>"]
        
        # Caracteres
        chars = list("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")
        chars += list(".,!?;:'-\"()[]{}@#$%^&*+=<>/\\|`~_ \n\t")
        chars += list("áéíóúñüÁÉÍÓÚÑÜ¿¡")
        
        # Palabras comunes (español + inglés + tech)
        common_words = [
            # Español
            "hola", "mundo", "gracias", "por", "favor", "bien", "mal", "si", "no",
            "que", "como", "cuando", "donde", "quien", "porque", "para", "con", "sin",
            "el", "la", "los", "las", "un", "una", "unos", "unas", "de", "en", "a",
            "es", "son", "ser", "estar", "tener", "hacer", "poder", "decir", "ir",
            "yo", "tu", "el", "ella", "nosotros", "ustedes", "ellos", "mi", "tu", "su",
            "bueno", "malo", "grande", "pequeño", "nuevo", "viejo", "mejor", "peor",
            "hoy", "ayer", "mañana", "ahora", "siempre", "nunca", "todo", "nada",
            "ayuda", "necesito", "quiero", "puedo", "debo", "tengo", "soy", "estoy",
            # Inglés
            "hello", "world", "thanks", "please", "good", "bad", "yes", "no",
            "what", "how", "when", "where", "who", "why", "which", "that", "this",
            "the", "a", "an", "is", "are", "was", "were", "be", "been", "being",
            "have", "has", "had", "do", "does", "did", "will", "would", "could",
            "i", "you", "he", "she", "it", "we", "they", "my", "your", "his", "her",
            "help", "need", "want", "can", "must", "should", "am", "im",
            # Tech/IA
            "ai", "ia", "python", "code", "data", "model", "train", "learn",
            "neural", "network", "machine", "learning", "deep", "algorithm",
            "function", "class", "variable", "array", "list", "dict", "string",
            "input", "output", "process", "compute", "memory", "fast", "slow",
            "adead", "bib", "compiler", "binary", "opcode", "cpu", "gpu",
            # Emociones
            "feliz", "triste", "enojado", "sorprendido", "asustado", "confundido",
            "happy", "sad", "angry", "surprised", "scared", "confused",
        ]
        
        # Construir vocabulario
        idx = 0
        for token in special:
            self.vocab[token] = idx
            self.inv_vocab[idx] = token
            idx += 1
        
        for char in chars:
            if char not in self.vocab:
                self.vocab[char] = idx
                self.inv_vocab[idx] = char
                idx += 1
        
        for word in common_words:
            word = word.lower()
            if word not in self.vocab:
                self.vocab[word] = idx
                self.inv_vocab[idx] = word
                idx += 1
        
        self.base_size = idx
    
    def encode(self, text: str, add_special: bool = True) -> List[int]:
        """Tokeniza texto a IDs."""
        tokens = []
        if add_special:
            tokens.append(self.BOS)
        
        # Dividir en palabras y puntuación
        parts = re.findall(r'\w+|[^\w\s]|\s+', text.lower())
        
        for part in parts:
            if part in self.vocab:
                tokens.append(self.vocab[part])
            else:
                # Fallback a caracteres
                for char in part:
                    tokens.append(self.vocab.get(char, self.UNK))
        
        if add_special:
            tokens.append(self.EOS)
        
        return tokens
    
    def decode(self, tokens: List[int], skip_special: bool = True) -> str:
        """Decodifica IDs a texto."""
        result = []
        special_ids = {self.PAD, self.EOS, self.BOS, self.SEP} if skip_special else set()
        
        for t in tokens:
            if t in special_ids:
                continue
            if t == self.EOS and skip_special:
                break
            result.append(self.inv_vocab.get(t, ""))
        
        return "".join(result)
    
    def learn_word(self, word: str) -> int:
        """Aprende una nueva palabra."""
        word = word.lower().strip()
        if word and word not in self.vocab and len(self.vocab) < self.vocab_size:
            idx = len(self.vocab)
            self.vocab[word] = idx
            self.inv_vocab[idx] = word
            return idx
        return self.vocab.get(word, self.UNK)
    
    def learn_from_text(self, text: str, min_freq: int = 2):
        """Aprende palabras frecuentes de un texto."""
        words = re.findall(r'\w+', text.lower())
        self.word_freq.update(words)
        
        for word, freq in self.word_freq.items():
            if freq >= min_freq and word not in self.vocab:
                self.learn_word(word)
    
    def get_vocab_size(self) -> int:
        """Retorna el tamaño actual del vocabulario."""
        return len(self.vocab)
    
    def __len__(self):
        return len(self.vocab)
