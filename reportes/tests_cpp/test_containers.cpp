#include <set>
#include <unordered_set>
#include <list>
#include <deque>
#include <queue>
#include <stack>
#include <array>
#include <iostream>

int main() {
    // set
    std::set<int> s = {3, 1, 4, 1, 5};
    s.insert(9);
    s.erase(1);
    auto it = s.find(4);
    int count = s.count(3);

    // unordered_set
    std::unordered_set<int> us = {10, 20, 30};
    us.insert(40);

    // list
    std::list<int> lst = {1, 2, 3};
    lst.push_front(0);
    lst.push_back(4);
    lst.sort();
    lst.reverse();
    lst.unique();

    // deque
    std::deque<int> dq = {1, 2, 3};
    dq.push_front(0);
    dq.push_back(4);

    // stack
    std::stack<int> stk;
    stk.push(1);
    stk.push(2);
    int top = stk.top();
    stk.pop();

    // queue
    std::queue<int> q;
    q.push(1);
    q.push(2);
    int front = q.front();
    q.pop();

    // priority_queue
    std::priority_queue<int> pq;
    pq.push(3);
    pq.push(1);
    pq.push(4);
    int ptop = pq.top();

    // array
    std::array<int, 5> arr = {1, 2, 3, 4, 5};
    int sz = arr.size();
    int first = arr.front();

    std::cout << "set size=" << s.size() << std::endl;
    return 0;
}
