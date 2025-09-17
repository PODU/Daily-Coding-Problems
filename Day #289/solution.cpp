// Misere Nim (3 heaps): first player wins iff
// (some heap>1 && xor!=0) || (all heaps<=1 && xor==0). Time: O(1), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

bool firstPlayerWins(int a, int b, int c) {
    int x = a ^ b ^ c;
    bool anyBig = a > 1 || b > 1 || c > 1;
    if (anyBig) return x != 0;
    return x == 0;
}

int main() {
    cout << boolalpha << firstPlayerWins(3, 4, 5) << endl;
    return 0;
}
