// Curried add_subtract: each call alternates +/- on the running total.
// operator() returns a chainable struct implicitly convertible to long.
// Time: O(1) per call. Space: O(1).
#include <bits/stdc++.h>
using namespace std;

struct AddSub {
    long total;
    int sign; // sign applied to the NEXT argument
    AddSub operator()(long y) const { return {total + sign * y, -sign}; }
    operator long() const { return total; }
};

AddSub add_subtract(long x) { return {x, 1}; }

int main() {
    cout << (long)add_subtract(7) << "\n";           // 7
    cout << (long)add_subtract(1)(2)(3) << "\n";     // 0
    cout << (long)add_subtract(-5)(10)(3)(9) << "\n"; // 11
    return 0;
}
