#include <header_main.h>
#include <iostream>
#include <vector>
#include <array>
#include <list>
#include <forward_list>
#include <deque>
#include <set>
#include <unordered_set>
#include <map>
#include <unordered_map>
#include <stack>
#include <queue>

int main() {
    std::vector<int> vec = {1, 2, 3};
    vec.push_back(4);
    printf("vector size=%d\n", vec.size());
    std::array<int, 5> arr = {10, 20, 30, 40, 50};
    printf("array front=%d back=%d\n", arr.front(), arr.back());
    std::list<int> lst = {1, 2, 3};
    lst.push_front(0);
    lst.push_back(4);
    lst.sort();
    printf("list size=%d\n", lst.size());
    std::forward_list<int> fl = {3, 1, 2};
    fl.push_front(0);
    fl.sort();
    std::deque<int> dq = {1, 2, 3};
    dq.push_front(0);
    dq.push_back(4);
    printf("deque size=%d front=%d back=%d\n", dq.size(), dq.front(), dq.back());
    std::set<int> s = {5, 3, 1, 4, 2};
    s.insert(6);
    printf("set size=%d count(3)=%d\n", s.size(), s.count(3));
    std::unordered_set<int> us = {10, 20, 30};
    us.insert(40);
    printf("uset size=%d\n", us.size());
    std::map<std::string, int> mp;
    mp["a"] = 1; mp["b"] = 2;
    printf("map size=%d\n", mp.size());
    std::unordered_map<std::string, int> um;
    um["x"] = 10; um["y"] = 20;
    printf("umap size=%d\n", um.size());
    std::stack<int> stk;
    stk.push(1); stk.push(2);
    printf("stack top=%d\n", stk.top());
    stk.pop();
    std::queue<int> q;
    q.push(10); q.push(20);
    printf("queue front=%d\n", q.front());
    q.pop();
    std::priority_queue<int> pq;
    pq.push(3); pq.push(1); pq.push(4);
    printf("pq top=%d\n", pq.top());
    return 0;
}
