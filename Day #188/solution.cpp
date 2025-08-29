// Day 188: Closure-in-a-loop late binding (C++ analog of the Python snippet).
// Capturing by reference [&i] shares one variable -> all read 3 after the loop.
// Fix: capture by value [i] to snapshot each iteration. Time/Space O(n).
#include <bits/stdc++.h>
using namespace std;

vector<function<int()>> makeFunctionsBuggy() {
    vector<function<int()>> funcs;
    int i = 0;
    for (int v : {1, 2, 3}) { i = v; funcs.push_back([&i]() { return i; }); }
    return funcs;
}

vector<function<int()>> makeFunctionsFixed() {
    vector<function<int()>> funcs;
    for (int i : {1, 2, 3}) funcs.push_back([i]() { return i; });
    return funcs;
}

int main() {
    cout << "Late binding prints:\n";
    for (auto& f : makeFunctionsBuggy()) cout << f() << "\n";
    cout << "Fixed prints:\n";
    for (auto& f : makeFunctionsFixed()) cout << f() << "\n";
    return 0;
}
