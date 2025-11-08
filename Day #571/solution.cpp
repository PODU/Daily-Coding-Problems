// Tower of Hanoi: classic recursion. Move n disks from rod `from` to `to` using `via`.
// Prints 2^n - 1 moves. O(2^n) time, O(n) recursion depth.
#include <bits/stdc++.h>
using namespace std;

void hanoi(int n, int from, int to, int via) {
    if (n == 0) return;
    hanoi(n - 1, from, via, to);
    cout << "Move " << from << " to " << to << "\n";
    hanoi(n - 1, via, to, from);
}

int main() {
    hanoi(3, 1, 3, 2);
    return 0;
}
