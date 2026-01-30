// Day 991: Closure late-binding demonstration.
// Original Python prints 3,3,3 because closures capture the loop variable by
// reference. Here we emulate: buggy captures a shared cell (all read 3),
// fixed captures by value (1,2,3). O(n) time/space.
#include <bits/stdc++.h>
using namespace std;

vector<function<int()>> makeBuggy() {
    vector<function<int()>> v;
    auto shared = make_shared<int>(0);
    for (int val : {1, 2, 3}) {
        *shared = val;                       // late binding: all share one cell
        v.push_back([shared]() { return *shared; });
    }
    return v;
}

vector<function<int()>> makeFixed() {
    vector<function<int()>> v;
    for (int val : {1, 2, 3}) {
        v.push_back([val]() { return val; }); // capture by value
    }
    return v;
}

int main() {
    cout << "Buggy:";
    for (auto& f : makeBuggy()) cout << ' ' << f();
    cout << "\nFixed:";
    for (auto& f : makeFixed()) cout << ' ' << f();
    cout << "\n";
    return 0;
}
