// Day 730: Closure-in-a-loop late binding (C++ analogue).
// Capturing the loop variable by reference -> all closures see its final value (3,3,3).
// Fix: capture by VALUE so each closure keeps its own copy (1,2,3).
#include <bits/stdc++.h>
using namespace std;

int main() {
    // Buggy: shared variable captured by reference
    vector<function<void()>> buggy;
    int i = 0;
    for (int v : {1, 2, 3}) { i = v; buggy.push_back([&i]() { cout << i << "\n"; }); }
    for (auto& f : buggy) f();   // 3, 3, 3

    // Fixed: capture by value
    vector<function<void()>> fixed;
    for (int v : {1, 2, 3}) fixed.push_back([v]() { cout << v << "\n"; });
    for (auto& f : fixed) f();   // 1, 2, 3
    return 0;
}
