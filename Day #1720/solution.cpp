// Min perfect squares: Legendre/Lagrange three-square theorem gives the count in
// O(sqrt N); decomposition found greedily largest-square-first. Time O(sqrt N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

bool isSquare(long long n) {
    long long r = (long long)sqrtl((long double)n);
    while (r * r > n) --r;
    while ((r + 1) * (r + 1) <= n) ++r;
    return r * r == n;
}

int minSquaresCount(long long n) {
    if (isSquare(n)) return 1;
    for (long long a = 1; a * a <= n; ++a)
        if (isSquare(n - a * a)) return 2;
    long long m = n;
    while (m % 4 == 0) m /= 4;
    if (m % 8 == 7) return 4;
    return 3;
}

bool find(long long n, int count, vector<long long>& out) {
    if (count == 1) {
        if (isSquare(n)) { out.push_back(n); return true; }
        return false;
    }
    for (long long a = (long long)sqrtl((long double)n) + 1; a >= 1; --a) {
        if (a * a > n) continue;
        if (find(n - a * a, count - 1, out)) { out.push_back(a * a); return true; }
    }
    return false;
}

void demo(long long n) {
    int count = minSquaresCount(n);
    vector<long long> parts;
    find(n, count, parts);
    sort(parts.rbegin(), parts.rend());
    cout << count << " (";
    for (size_t i = 0; i < parts.size(); ++i) {
        if (i) cout << " + ";
        cout << parts[i];
    }
    cout << ")\n";
}

int main() {
    demo(4);
    demo(17);
    demo(18);
    return 0;
}
