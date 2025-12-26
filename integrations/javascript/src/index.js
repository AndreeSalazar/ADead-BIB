/**
 * ADead-BIB JavaScript Integration
 * =================================
 * Entry point for the JavaScript binding
 */

const { ADeadEngine } = require('./adead-core');

// Re-export
module.exports = {
    ADeadEngine,
    Engine: ADeadEngine,
    
    // Factory function
    createEngine: (config) => new ADeadEngine(config),
    
    // Quick utilities
    matmul: (a, b) => new ADeadEngine().matmul(a, b),
    attention: (q, k, v, config) => new ADeadEngine().attention(q, k, v, config),
    softmax: (arr) => new ADeadEngine().softmax(arr),
    relu: (arr) => new ADeadEngine().relu(arr),
};
