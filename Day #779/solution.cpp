// Day 779: Egg drop - minimum worst-case trials for N eggs, k floors.
// Find smallest m with sum_{i=1..N} C(m,i) >= k. O(N * answer) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

int eggDrop(int eggs, int floors) {
    if (floors == 0) return 0;
    int m = 0;
    while (true) {
        m++;
        long long reach = 0, c = 1;          // c = C(m, i)
        for (int i = 1; i <= eggs; i++) {
            c = c * (m - i + 1) / i;          // C(m,i)
            reach += c;
            if (reach >= floors) break;
        }
        if (reach >= floors) return m;
    }
}

int main() {
    cout << eggDrop(1, 5) << endl;  // 5
    cout << eggDrop(2, 100) << endl; // 14
    return 0;
}
