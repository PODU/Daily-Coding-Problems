// Count lattice paths in N x M grid via combinatorics C(n+m-2, n-1).
// Time O(min(n,m)) multiplicative, Space O(1).
#include <iostream>
using namespace std;

long long paths(int n, int m) {
    int down = n - 1, right = m - 1;
    int k = min(down, right), total = down + right;
    long long res = 1;
    for (int i = 1; i <= k; ++i) {
        res = res * (total - k + i) / i;
    }
    return res;
}

int main() {
    cout << "2 by 2 matrix -> " << paths(2, 2) << "\n";
    cout << "5 by 5 matrix -> " << paths(5, 5) << "\n";
    return 0;
}
