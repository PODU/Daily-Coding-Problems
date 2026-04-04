// Day 1298: Count Kaprekar steps to reach 6174.
// Repeatedly sort digits desc - asc until 6174. Converges in <=7 steps. O(steps) time.
#include <bits/stdc++.h>
using namespace std;

int kaprekarSteps(int n) {
    int steps = 0;
    while (n != 6174) {
        char d[5];
        sprintf(d, "%04d", n); // pad to 4 digits
        string s(d);
        string asc = s, desc = s;
        sort(asc.begin(), asc.end());
        sort(desc.rbegin(), desc.rend());
        n = stoi(desc) - stoi(asc);
        steps++;
        if (n == 0) break; // all digits equal -> never reaches 6174
    }
    return steps;
}

int main() {
    cout << kaprekarSteps(1234) << endl; // 3
    return 0;
}
