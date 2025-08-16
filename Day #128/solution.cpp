// Day 128: Tower of Hanoi - print all moves.
// Classic recursion. O(2^n) moves (optimal), O(n) recursion depth.
#include <bits/stdc++.h>
using namespace std;

void hanoi(int n, int from, int via, int to) {
    if (n == 0) return;
    hanoi(n - 1, from, to, via);
    cout << "Move " << from << " to " << to << endl;
    hanoi(n - 1, via, from, to);
}

int main() {
    hanoi(3, 1, 2, 3);
    return 0;
}
