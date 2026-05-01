// Day 1450: Tower of Hanoi - print each single-disk move from rod 1 to rod 3.
// Classic recursion. Time O(2^n) moves, Space O(n) recursion depth.
#include <bits/stdc++.h>
using namespace std;

void hanoi(int n, int from, int to, int aux) {
    if (n == 0) return;
    hanoi(n - 1, from, aux, to);
    cout << "Move " << from << " to " << to << "\n";
    hanoi(n - 1, aux, to, from);
}

int main() {
    hanoi(3, 1, 3, 2);
    return 0;
}
