// Day 1118 - Late-binding closure pitfall
// Capturing the loop variable by reference makes every lambda see its final
// value (9). Fix: capture by value so each lambda binds its own copy.
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<function<int()>> buggy, fixed;
    int holder = 0;
    for (int k = 0; k < 10; ++k) {
        holder = k;
        buggy.push_back([&holder]() { return holder; }); // shared reference
        fixed.push_back([k]() { return k; });             // own copy
    }

    cout << "Buggy output (all 9):\n";
    for (auto& f : buggy) cout << f() << "\n";
    cout << "Fixed output (0-9):\n";
    for (auto& f : fixed) cout << f() << "\n";
    return 0;
}
