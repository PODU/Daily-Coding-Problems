// Day 1856: Collatz conjecture test + longest chain under 1,000,000.
// Memoized step counts (cache values <= LIMIT). ~O(LIMIT * avg-chain) time, O(LIMIT) space.
#include <bits/stdc++.h>
using namespace std;

int main() {
    const int LIMIT = 1000000;
    vector<int> steps(LIMIT + 1, 0); // steps[n] = steps to reach 1 (0 = unknown, steps[1]=0 sentinel handled)
    steps[1] = 0;
    bool allReach1 = true;
    int bestN = 1, bestSteps = 0;

    for (int i = 2; i <= LIMIT; i++) {
        long long n = i;
        int cnt = 0;
        while (n != 1 && (n > LIMIT || steps[n] == 0)) {
            n = (n % 2 == 0) ? n / 2 : 3 * n + 1;
            cnt++;
        }
        int total = cnt + (n == 1 ? 0 : steps[n]);
        steps[i] = total;
        if (total > bestSteps) { bestSteps = total; bestN = i; }
    }

    cout << "All sequences for n in [1, " << LIMIT << "] reach 1: "
         << (allReach1 ? "true" : "false") << "\n";
    cout << "Longest sequence under " << LIMIT << ": n = " << bestN
         << " with " << (bestSteps + 1) << " terms\n"; // 837799, 525 terms
    return 0;
}
