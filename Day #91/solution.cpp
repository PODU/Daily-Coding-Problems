// Day 91: Closure-in-loop. Capturing by reference shares one variable (prints 9
// ten times); capture by value to fix so each lambda keeps its own i. O(n).
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<function<int()>> broken, fixed_;
    int shared = 0;
    for (int i = 0; i < 10; i++) broken.push_back([&shared]() { return shared; });
    shared = 9; // mirrors Python: all see final value
    for (int i = 0; i < 10; i++) fixed_.push_back([i]() { return i; }); // by value

    cout << "Broken (prints 9 ten times):";
    for (auto& f : broken) cout << ' ' << f();
    cout << "\nFixed:";
    for (auto& f : fixed_) cout << ' ' << f();
    cout << "\n";
    return 0;
}
