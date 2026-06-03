// 2xN tiling with dominoes + L-trominoes (LeetCode 790). DP recurrence
// f(n)=2*f(n-1)+f(n-3). O(N) time, O(1) space. Mod 1e9+7 for large N.
#include <iostream>
using namespace std;
const long long MOD = 1000000007;

long long numTilings(int n) {
    if (n == 0) return 1;
    if (n == 1) return 1;
    if (n == 2) return 2;
    long long a = 1, b = 1, c = 2; // f(0),f(1),f(2)
    for (int i = 3; i <= n; i++) {
        long long f = (2 * c + a) % MOD;
        a = b; b = c; c = f;
    }
    return c;
}

int main() {
    cout << "N=4 -> " << numTilings(4) << endl; // 11
    cout << "Table N=1..5: ";
    for (int n = 1; n <= 5; n++)
        cout << numTilings(n) << (n < 5 ? " " : "\n"); // 1 2 5 11 24
    return 0;
}
