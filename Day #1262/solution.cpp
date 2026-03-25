// Day 1262: Closure-in-a-loop (late binding) demonstrated in C++.
// Capturing by reference [&] all closures share i and print its final value (3).
// Fix: capture by value [i] so each closure stores its own copy.
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<function<void()>> buggy, fixed;
    int shared = 0;
    for (int i = 1; i <= 3; ++i) {
        shared = i;
        buggy.push_back([&shared]() { cout << shared << "\n"; }); // all share `shared` -> final value
        fixed.push_back([i]() { cout << i << "\n"; });            // each copies its own i
    }
    cout << "Buggy output:\n";
    for (auto& f : buggy) f();
    cout << "Fixed output:\n";
    for (auto& f : fixed) f();
    return 0;
}
