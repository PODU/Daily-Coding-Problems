// Day 1277: Curried add_subtract — alternately + then - successive args.
// Functor whose operator() returns the next stage; implicit conv yields result. O(1)/call.
#include <bits/stdc++.h>
using namespace std;

struct AddSubtract {
    long long value;
    int idx; // number of args consumed so far
    AddSubtract operator()(long long x) const {
        long long nv = (idx % 2 == 1) ? value + x : value - x;
        return AddSubtract{nv, idx + 1};
    }
    operator long long() const { return value; }
};

AddSubtract add_subtract(long long x) { return AddSubtract{x, 1}; }

int main() {
    cout << (long long)add_subtract(7) << "\n";            // 7
    cout << (long long)add_subtract(1)(2)(3) << "\n";       // 0
    cout << (long long)add_subtract(-5)(10)(3)(9) << "\n";  // 11
    return 0;
}
