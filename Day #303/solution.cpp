// Day 303: Angle between clock hands. O(1).
// Bonus: hands overlap (angle 0) every 12/11 hours (~65.45 min apart).
#include <bits/stdc++.h>
using namespace std;
int clockAngle(int h, int m) {
    double hr = (h % 12) * 30.0 + m * 0.5;
    double mn = m * 6.0;
    double diff = fabs(hr - mn);
    diff = min(diff, 360.0 - diff);
    return (int)llround(diff);
}
int main() {
    cout << clockAngle(3, 30) << "\n"; // 75
}
