// Day 1028: Kaprekar's routine. Repeatedly subtract ascending- from descending-
// digit arrangement until 6174; count steps. Time O(steps), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int kaprekarSteps(int n) {
    int steps = 0;
    while (n != 6174) {
        char d[5];
        sprintf(d, "%04d", n);
        string s(d);
        sort(s.begin(), s.end());
        int asc = stoi(s);
        reverse(s.begin(), s.end());
        int desc = stoi(s);
        n = desc - asc;
        ++steps;
    }
    return steps;
}

int main() {
    cout << kaprekarSteps(1234) << "\n";
    return 0;
}
