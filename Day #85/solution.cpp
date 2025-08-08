// Day 85: Select x if b==1 else y using only bit ops. mask = -b (all 1s or all 0s).
// Time O(1), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int select(int x, int y, int b) {
    int32_t mask = -b;                 // b=1 -> 0xFFFFFFFF, b=0 -> 0x00000000
    return (x & mask) | (y & ~mask);
}

int main() {
    cout << select(5, 10, 1) << "\n"; // 5
    cout << select(5, 10, 0) << "\n"; // 10
    return 0;
}
