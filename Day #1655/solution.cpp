// Closure capture demo: buggy lambdas capture a shared variable by reference (final value);
// fix captures a per-iteration copy by value. O(n) time/space.
#include <bits/stdc++.h>
using namespace std;

int main() {
    cout << "Late binding (buggy):\n";
    vector<function<int()>> funcs;
    int shared = 0;
    for (int v : {1, 2, 3}) {
        shared = v;
        funcs.push_back([&shared]() { return shared; });
    }
    for (auto &f : funcs) cout << f() << "\n";
    cout << "Fixed (capture value):\n";
    funcs.clear();
    for (int v : {1, 2, 3}) {
        funcs.push_back([v]() { return v; });
    }
    for (auto &f : funcs) cout << f() << "\n";
    return 0;
}
