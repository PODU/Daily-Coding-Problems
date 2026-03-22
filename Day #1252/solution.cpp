// Clock angle: minute=mm*6, hour=(hh%12)*30+mm*0.5, take min(diff,360-diff), round. O(1) time/space.
#include <bits/stdc++.h>
using namespace std;

int clockAngle(int hh, int mm) {
    double minute = mm * 6.0;
    double hour = (hh % 12) * 30.0 + mm * 0.5;
    double diff = fabs(hour - minute);
    diff = min(diff, 360.0 - diff);
    return (int)floor(diff + 0.5);
}

int main() {
    cout << clockAngle(12, 30) << "\n";
    cout << clockAngle(3, 15) << "\n";
    return 0;
}
