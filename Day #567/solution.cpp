// Closure-based pair: cons stores (a,b) in a function; car/cdr apply a selector. Time O(1), Space O(1).
#include <bits/stdc++.h>
using namespace std;

using Pair = function<int(function<int(int,int)>)>;

Pair cons(int a, int b) {
    return [a, b](function<int(int,int)> f) { return f(a, b); };
}
int car(const Pair& p) { return p([](int a, int) { return a; }); }
int cdr(const Pair& p) { return p([](int, int b) { return b; }); }

int main() {
    Pair p = cons(3, 4);
    cout << car(p) << "\n";
    cout << cdr(p) << "\n";
    return 0;
}
