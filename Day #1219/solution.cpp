// Closure late-binding: capturing by reference [&] makes all closures read one shared
// variable (ends at 9); fix = capture by value [i]. O(n) build/call, O(n) space.
#include <bits/stdc++.h>
using namespace std;

int main() {
    // Buggy: every lambda captures the SAME variable by reference; it ends at 9.
    vector<function<int()>> buggy;
    int shared = 0;
    for (int t = 0; t < 10; t++) {
        shared = t;
        buggy.push_back([&shared]() { return shared; });
    }
    for (auto& f : buggy) cout << f() << "\n";

    cout << "---" << "\n";

    // Fixed: capture by value so each lambda keeps its own copy of i.
    vector<function<int()>> fixed;
    for (int i = 0; i < 10; i++) {
        fixed.push_back([i]() { return i; });
    }
    for (auto& f : fixed) cout << f() << "\n";
    return 0;
}
