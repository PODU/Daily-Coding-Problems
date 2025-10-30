// Branchless select: mask = -b (all 1s if b=1, all 0s if b=0); pick x or y. O(1).
#include <bits/stdc++.h>
using namespace std;

int select_(int x, int y, int b) {
    int mask = -b;                 // 0xFFFFFFFF if b==1, 0x00000000 if b==0
    return (x & mask) | (y & ~mask);
}

int main() {
    cout << select_(42, 17, 1) << "\n"; // 42
    cout << select_(42, 17, 0) << "\n"; // 17
    return 0;
}
