/**
 * ADead-BIB Universal Runtime - C++ FFI
 * ======================================
 * Author: Eddi Andreé Salazar Matos
 * Email: eddi.salazar.dev@gmail.com
 * Made with love in Peru
 * 
 * Header para usar el runtime desde C++.
 */

#ifndef ADEAD_CPP_H
#define ADEAD_CPP_H

#include "../../core/runtime.h"

#ifdef __cplusplus

#include <vector>
#include <string>
#include <memory>
#include <stdexcept>

namespace adead {

/* ============================================================
 * Exception
 * ============================================================ */

class RuntimeError : public std::runtime_error {
public:
    RuntimeError(ADeadError code, const std::string& msg)
        : std::runtime_error(msg), error_code(code) {}
    
    ADeadError code() const { return error_code; }
    
private:
    ADeadError error_code;
};

/* ============================================================
 * Tensor Wrapper
 * ============================================================ */

class Tensor {
public:
    Tensor() : tensor_{}, runtime_(nullptr) {}
    
    Tensor(ADeadRuntime* rt, const std::vector<uint64_t>& shape, ADeadDType dtype = ADEAD_DTYPE_F32)
        : runtime_(rt) {
        ADeadError err = adead_tensor_create(rt, &tensor_, shape.data(), 
                                             static_cast<u32>(shape.size()), dtype);
        if (err != ADEAD_OK) {
            throw RuntimeError(err, "Failed to create tensor");
        }
    }
    
    ~Tensor() {
        if (runtime_ && tensor_.data) {
            adead_tensor_destroy(runtime_, &tensor_);
        }
    }
    
    /* Move semantics */
    Tensor(Tensor&& other) noexcept 
        : tensor_(other.tensor_), runtime_(other.runtime_) {
        other.tensor_ = {};
        other.runtime_ = nullptr;
    }
    
    Tensor& operator=(Tensor&& other) noexcept {
        if (this != &other) {
            if (runtime_ && tensor_.data) {
                adead_tensor_destroy(runtime_, &tensor_);
            }
            tensor_ = other.tensor_;
            runtime_ = other.runtime_;
            other.tensor_ = {};
            other.runtime_ = nullptr;
        }
        return *this;
    }
    
    /* No copy */
    Tensor(const Tensor&) = delete;
    Tensor& operator=(const Tensor&) = delete;
    
    /* Data access */
    template<typename T>
    void copy_from(const std::vector<T>& data) {
        ADeadError err = adead_tensor_copy_from(runtime_, &tensor_, 
                                                data.data(), data.size() * sizeof(T));
        if (err != ADEAD_OK) {
            throw RuntimeError(err, "Failed to copy data to tensor");
        }
    }
    
    template<typename T>
    std::vector<T> to_vector() const {
        size_t numel = static_cast<size_t>(adead_tensor_numel(&tensor_));
        std::vector<T> result(numel);
        ADeadError err = adead_tensor_copy_to(runtime_, &tensor_,
                                              result.data(), result.size() * sizeof(T));
        if (err != ADEAD_OK) {
            throw RuntimeError(err, "Failed to copy data from tensor");
        }
        return result;
    }
    
    /* Properties */
    std::vector<uint64_t> shape() const {
        return std::vector<uint64_t>(tensor_.shape, tensor_.shape + tensor_.ndim);
    }
    
    uint32_t ndim() const { return tensor_.ndim; }
    ADeadDType dtype() const { return tensor_.dtype; }
    size_t size_bytes() const { return tensor_.size_bytes; }
    
    /* Internal access */
    ADeadTensor* ptr() { return &tensor_; }
    const ADeadTensor* ptr() const { return &tensor_; }
    
private:
    ADeadTensor tensor_;
    ADeadRuntime* runtime_;
};

/* ============================================================
 * Runtime Wrapper
 * ============================================================ */

class Runtime {
public:
    explicit Runtime(ADeadBackend backend = ADEAD_BACKEND_AUTO) {
        ADeadError err = adead_init(&runtime_, backend);
        if (err != ADEAD_OK) {
            throw RuntimeError(err, "Failed to initialize runtime");
        }
    }
    
    ~Runtime() {
        adead_shutdown(&runtime_);
    }
    
    /* No copy/move */
    Runtime(const Runtime&) = delete;
    Runtime& operator=(const Runtime&) = delete;
    Runtime(Runtime&&) = delete;
    Runtime& operator=(Runtime&&) = delete;
    
    /* Backend info */
    ADeadBackend backend() const { return adead_get_backend(&runtime_); }
    std::string backend_name() const { return adead_get_backend_name(&runtime_); }
    
    /* Memory info */
    size_t memory_available() const { return adead_memory_available(&runtime_); }
    size_t memory_used() const { return adead_memory_used(&runtime_); }
    
    /* Tensor creation */
    Tensor tensor(const std::vector<uint64_t>& shape, ADeadDType dtype = ADEAD_DTYPE_F32) {
        return Tensor(&runtime_, shape, dtype);
    }
    
    /* Operations */
    void matmul(const Tensor& a, const Tensor& b, Tensor& c) {
        ADeadError err = adead_matmul(&runtime_, a.ptr(), b.ptr(), c.ptr());
        if (err != ADEAD_OK) {
            throw RuntimeError(err, "MatMul failed");
        }
    }
    
    void add(const Tensor& a, const Tensor& b, Tensor& c) {
        ADeadError err = adead_add(&runtime_, a.ptr(), b.ptr(), c.ptr());
        if (err != ADEAD_OK) {
            throw RuntimeError(err, "Add failed");
        }
    }
    
    void relu(const Tensor& in, Tensor& out) {
        ADeadError err = adead_relu(&runtime_, in.ptr(), out.ptr());
        if (err != ADEAD_OK) {
            throw RuntimeError(err, "ReLU failed");
        }
    }
    
    void softmax(const Tensor& in, Tensor& out) {
        ADeadError err = adead_softmax(&runtime_, in.ptr(), out.ptr());
        if (err != ADEAD_OK) {
            throw RuntimeError(err, "Softmax failed");
        }
    }
    
    void sync() {
        adead_sync(&runtime_);
    }
    
    /* Internal access */
    ADeadRuntime* ptr() { return &runtime_; }
    
private:
    ADeadRuntime runtime_;
};

/* ============================================================
 * Convenience Functions
 * ============================================================ */

inline std::string version() {
    return adead_version_string();
}

} // namespace adead

#endif /* __cplusplus */

#endif /* ADEAD_CPP_H */
