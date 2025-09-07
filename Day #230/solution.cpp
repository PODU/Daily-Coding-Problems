// Egg drop: find min trials t such that maxFloors(t, eggs) >= k, where
// f(t,e) = f(t-1,e-1) + f(t-1,e) + 1 (floors distinguishable). Time: O(eggs * answer), Space: O(eggs).
#include <bits/stdc++.h>
using namespace std;

int eggDrop(int eggs, int k) {
    vector<long long> f(eggs + 1, 0);
    int t = 0;
    while (f[eggs] < k) {
        t++;
        for (int e = eggs; e >= 1; e--) f[e] = f[e] + f[e - 1] + 1;
    }
    return t;
}

int main() {
    cout << eggDrop(1, 5) << "\n"; // 5
}
