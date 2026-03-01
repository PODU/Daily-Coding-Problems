// cons returns a closure pair(f)=f(a,b); car/cdr apply a selector. O(1).
#include <bits/stdc++.h>
using namespace std;

using Selector = function<int(int, int)>;
using Pair = function<int(Selector)>;

Pair cons(int a, int b) {
    return [a, b](Selector f) { return f(a, b); };
}
int car(Pair p) { return p([](int a, int b) { return a; }); }
int cdr(Pair p) { return p([](int a, int b) { return b; }); }

int main() {
    cout << car(cons(3, 4)) << endl;
    cout << cdr(cons(3, 4)) << endl;
}
