// Day 363: Curried add_subtract that alternately adds/subtracts arguments.
// A small callable object accumulates a running value; chaining () adds args.
// Time O(k) per chain of k args, Space O(1).
#include <bits/stdc++.h>
using namespace std;

struct AddSubtract {
    long value;
    int count;            // number of args consumed so far
    explicit AddSubtract(long first) : value(first), count(1) {}
    AddSubtract operator()(long x) const {
        AddSubtract r = *this;
        r.value += (count % 2 == 1) ? x : -x; // index 1 adds, 2 subtracts, ...
        r.count++;
        return r;
    }
    operator long() const { return value; }
};

AddSubtract add_subtract(long first) { return AddSubtract(first); }

int main() {
    cout << (long)add_subtract(7) << "\n";
    cout << "1 + 2 - 3 -> " << (long)add_subtract(1)(2)(3) << "\n";
    cout << "-5 + 10 - 3 + 9 -> " << (long)add_subtract(-5)(10)(3)(9) << "\n";
    return 0;
}
