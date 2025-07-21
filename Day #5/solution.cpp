// Closure-based pair: cons stores (a,b) in a lambda; car/cdr apply a selector.
// Time: O(1), Space: O(1) per pair.
#include <bits/stdc++.h>
using namespace std;

using Pair = function<int(function<int(int, int)>)>;

Pair cons(int a, int b) {
    return [a, b](function<int(int, int)> f) { return f(a, b); };
}
int car(Pair p) { return p([](int a, int) { return a; }); }
int cdr(Pair p) { return p([](int, int b) { return b; }); }

int main() {
    cout << car(cons(3, 4)) << "\n";
    cout << cdr(cons(3, 4)) << "\n";
    return 0;
}
