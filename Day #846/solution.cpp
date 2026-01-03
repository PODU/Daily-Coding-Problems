// Day 846: implement car/cdr for closure-based cons.
// cons stores a pair as a function taking a selector; car selects first, cdr selects second. O(1).
#include <bits/stdc++.h>
using namespace std;

// A pair is a function that, given a selector f(a,b), returns f(a,b).
using Selector = function<int(int,int)>;
using Pair = function<int(Selector)>;

Pair cons(int a, int b){
    return [a,b](Selector f){ return f(a, b); };
}
int car(const Pair& p){ return p([](int a, int){ return a; }); }
int cdr(const Pair& p){ return p([](int, int b){ return b; }); }

int main(){
    cout << car(cons(3,4)) << "\n"; // 3
    cout << cdr(cons(3,4)) << "\n"; // 4
    return 0;
}
