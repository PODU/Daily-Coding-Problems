// Clock angle: minute=mm*6, hour=(hh%12)*30+mm*0.5, take min(diff,360-diff), round. O(1) time/space.
// Bonus: hands overlap (angle 0) 11 times per 12 hours, at t = (12/11)*k hours for k=0..10.
#include <bits/stdc++.h>
using namespace std;

int clockAngle(int hh, int mm) {
    double minuteAngle = mm * 6.0;
    double hourAngle = (hh % 12) * 30.0 + mm * 0.5;
    double diff = fabs(hourAngle - minuteAngle);
    double angle = min(diff, 360.0 - diff);
    return (int)llround(angle);
}

int main() {
    string t = "3:15";
    int hh = 0, mm = 0;
    sscanf(t.c_str(), "%d:%d", &hh, &mm);
    cout << clockAngle(hh, mm) << "\n";
    return 0;
}
