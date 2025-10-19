// Day 459: Fewest perfect squares summing to N.
// Lagrange/Legendre theorem -> answer in {1,2,3,4}, O(sqrt N).
// Reconstruct one decomposition guided by the count.
#include <iostream>
#include <vector>
#include <cmath>
using namespace std;

bool isSquare(long n) {
    if (n < 0) return false;
    long r = (long)sqrtl((long double)n);
    while (r * r < n) r++;
    while (r * r > n) r--;
    return r * r == n;
}

int minSquares(long n) {
    if (isSquare(n)) return 1;
    long m = n;
    while (m % 4 == 0) m /= 4;
    if (m % 8 == 7) return 4;
    for (long i = 1; i * i <= n; i++)
        if (isSquare(n - i * i)) return 2;
    return 3;
}

vector<long> decompose(long n) {
    int k = minSquares(n);
    vector<long> res;
    while (k > 0) {
        if (k == 1) { res.push_back(n); break; }
        for (long i = (long)sqrtl((long double)n); i >= 1; i--) {
            if (minSquares(n - i * i) == k - 1) {
                res.push_back(i * i);
                n -= i * i;
                k--;
                break;
            }
        }
    }
    return res;
}

int main() {
    long n = 17;
    auto sq = decompose(n);
    cout << minSquares(n) << " (";
    for (size_t i = 0; i < sq.size(); i++) cout << sq[i] << (i + 1 < sq.size() ? " + " : "");
    cout << ")" << endl; // 2 (16 + 1)
    return 0;
}
