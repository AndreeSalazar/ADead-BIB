#include <header_main.h>
#include <iostream>
#include <thread>
#include <mutex>
#include <atomic>
#include <future>
#include <condition_variable>

int main() {
    std::mutex mtx;
    std::atomic<int> counter(0);
    std::thread t1([&]() {
        std::lock_guard<std::mutex> lock(mtx);
        counter.fetch_add(1);
    });
    std::thread t2([&]() {
        std::lock_guard<std::mutex> lock(mtx);
        counter.fetch_add(1);
    });
    t1.join();
    t2.join();
    printf("counter=%d\n", counter.load());
    auto fut = std::async(std::launch::async, []() { return 42; });
    int result = fut.get();
    printf("async result=%d\n", result);
    std::promise<int> prom;
    std::future<int> f = prom.get_future();
    prom.set_value(100);
    printf("promise=%d\n", f.get());
    std::condition_variable cv;
    bool ready = false;
    printf("concurrency OK\n");
    return 0;
}
