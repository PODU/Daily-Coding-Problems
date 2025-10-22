// Closure capture: capturing a shared variable by reference makes every lambda read its final
// value (9). Fix by capturing by value [j] so each lambda keeps its own copy -> prints 0..9.
// (Python analogue: lambda: i prints 9 ten times; fix is lambda i=i: i.)
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<function<int()>> buggy;
    int shared = 0;
    for (int i = 0; i < 10; i++) {
        shared = i;
        buggy.push_back([&shared]() { return shared; }); // by reference: all share `shared`
    }
    cout << "Buggy:\n";
    for (auto& f : buggy) cout << f() << "\n";

    vector<function<int()>> fixed;
    for (int j = 0; j < 10; j++) {
        fixed.push_back([j]() { return j; });            // by value: own copy each iteration
    }
    cout << "Fixed:\n";
    for (auto& f : fixed) cout << f() << "\n";
}
