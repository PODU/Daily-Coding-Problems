// Quxes minimization: count R/G/B; if two counts are zero -> n;
// else if all three parities equal -> 2; else -> 1. Time O(n), Space O(1).
#include <iostream>
#include <vector>
#include <string>
using namespace std;

int minQuxes(const vector<char>& a) {
    int r = 0, g = 0, b = 0;
    for (char c : a) {
        if (c == 'R') r++;
        else if (c == 'G') g++;
        else b++;
    }
    int n = (int)a.size();
    int zeros = (r == 0) + (g == 0) + (b == 0);
    if (zeros >= 2) return n;
    if ((r % 2) == (g % 2) && (g % 2) == (b % 2)) return 2;
    return 1;
}

int main() {
    vector<char> demo = {'R', 'G', 'B', 'G', 'B'};
    cout << minQuxes(demo) << endl;
    return 0;
}
