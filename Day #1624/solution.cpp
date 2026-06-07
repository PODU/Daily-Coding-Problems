// Day 1624: Steps of Kaprekar's routine to reach 6174.
// Iterate sort-desc minus sort-asc until 6174. Time O(1) (bounded ~7 iters).
#include <bits/stdc++.h>
using namespace std;

int kaprekarSteps(int n) {
    int steps = 0;
    while (n != 6174) {
        char d[5];
        snprintf(d, sizeof(d), "%04d", n);
        string s(d);
        string asc = s, desc = s;
        sort(asc.begin(), asc.end());
        sort(desc.rbegin(), desc.rend());
        n = stoi(desc) - stoi(asc);
        steps++;
        if (n == 0) break; // repdigit, never reaches 6174
    }
    return steps;
}

int main() {
    cout << kaprekarSteps(1234) << endl;
    return 0;
}
