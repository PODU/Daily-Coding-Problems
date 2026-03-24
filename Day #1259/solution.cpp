// Min flips so all x before all y: single-pass DP. flips=min(flips+1, yCount) on 'x', yCount++ on 'y'. O(n) time, O(1) space.
#include <iostream>
#include <string>
#include <algorithm>
using namespace std;

int minFlips(const string& s) {
    int flips = 0, yCount = 0;
    for (char c : s) {
        if (c == 'y') yCount++;
        else flips = min(flips + 1, yCount);
    }
    return flips;
}

int main() {
    cout << minFlips("xyxxxyxyy") << "\n";
    return 0;
}
