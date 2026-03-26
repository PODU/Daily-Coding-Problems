// Day 1268: Select x if b==1 else y using only arithmetic/bit ops (no branches).
// y ^ ((x ^ y) & -b): -b is all-ones when b==1, all-zeros when b==0. O(1).
#include <bits/stdc++.h>
using namespace std;

int32_t select(int32_t x, int32_t y, int32_t b) {
    return y ^ ((x ^ y) & (-b));
}

int main() {
    int32_t x = 5, y = 10;
    cout << "b=1 -> " << select(x, y, 1) << endl; // 5
    cout << "b=0 -> " << select(x, y, 0) << endl; // 10
    return 0;
}
