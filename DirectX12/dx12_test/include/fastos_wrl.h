// fastos_wrl.h — ADead-BIB ComPtr<T> template
// Minimal wrl.h replacement for DirectX 12 compilation
#pragma once

namespace Microsoft {
namespace WRL {

template<typename T>
class ComPtr {
public:
    T* ptr;

    ComPtr() : ptr(NULL) {}
    ComPtr(T* p) : ptr(p) { if (ptr) ptr->AddRef(); }

    ~ComPtr() {
        if (ptr) {
            ptr->Release();
            ptr = NULL;
        }
    }

    T* Get() const { return ptr; }
    T** GetAddressOf() { return &ptr; }
    T* operator->() const { return ptr; }
    T** operator&() { return &ptr; }

    void Reset() {
        if (ptr) {
            ptr->Release();
            ptr = NULL;
        }
    }

    T* Detach() {
        T* tmp = ptr;
        ptr = NULL;
        return tmp;
    }

    void Attach(T* p) {
        if (ptr) ptr->Release();
        ptr = p;
    }

    T** ReleaseAndGetAddressOf() {
        Reset();
        return &ptr;
    }

    template<typename U>
    HRESULT As(ComPtr<U>* other) {
        return ptr->QueryInterface(&other->ptr);
    }

    operator bool() const { return ptr != NULL; }
};

} // namespace WRL
} // namespace Microsoft

using Microsoft::WRL::ComPtr;
