// Clock angle: O(1) time, O(1) space.
// minute=mm*6, hour=(hh%12)*30+mm*0.5, diff=|h-m|, angle=min(diff,360-diff), rounded.
#include <bits/stdc++.h>
using namespace std;

int clockAngle(int hh, int mm) {
    double minute = mm * 6.0;
    double hour = (hh % 12) * 30.0 + mm * 0.5;
    double diff = fabs(hour - minute);
    double angle = min(diff, 360.0 - diff);
    return (int)llround(angle);
}

int main() {
    printf("%02d:%02d -> %d\n", 3, 30, clockAngle(3, 30));
    printf("%02d:%02d -> %d\n", 12, 0, clockAngle(12, 0));
    return 0;
}
