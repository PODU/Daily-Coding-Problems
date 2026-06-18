// cons returns a closure taking a selector(a,b)->int; car/cdr pass selectors
// returning first/second arg. All O(1).
#include <iostream>
#include <functional>
using namespace std;

using Selector = function<int(int, int)>;
using Pair = function<int(Selector)>;

Pair cons(int a, int b) {
    return [a, b](Selector f) { return f(a, b); };
}

int car(Pair p) { return p([](int a, int) { return a; }); }
int cdr(Pair p) { return p([](int, int b) { return b; }); }

int main() {
    cout << car(cons(3, 4)) << '\n';
    cout << cdr(cons(3, 4)) << '\n';
    return 0;
}
