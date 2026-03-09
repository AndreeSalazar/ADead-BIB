#include <chrono>
#include <thread>
#include <mutex>
#include <atomic>
#include <condition_variable>
#include <future>
#include <iostream>

int main() {
    // chrono
    auto start = std::chrono::high_resolution_clock::now();
    auto dur = std::chrono::duration<double>(1.5);
    auto ms = std::chrono::milliseconds(100);
    auto us = std::chrono::microseconds(1000);
    auto ns = std::chrono::nanoseconds(1000000);
    auto end = std::chrono::high_resolution_clock::now();
    auto elapsed = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);

    // thread
    std::thread t([]() {
        std::cout << "Hello from thread" << std::endl;
    });
    t.join();

    // mutex
    std::mutex mtx;
    std::lock_guard<std::mutex> lock(mtx);

    // atomic
    std::atomic<int> counter(0);
    counter.fetch_add(1);
    counter.store(42);
    int val = counter.load();

    // future/promise
    std::promise<int> prom;
    std::future<int> fut = prom.get_future();
    prom.set_value(42);
    int result = fut.get();

    // async
    auto async_result = std::async(std::launch::async, []() { return 100; });

    std::cout << "elapsed=" << elapsed.count() << "ms" << std::endl;
    return 0;
}
