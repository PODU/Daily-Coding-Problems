// Clock hand angle: minute=mm*6, hour=(hh%12)*30+mm*0.5, take abs diff, fold >180, round. O(1) time/space.
// Fun fact: the hands overlap (angle 0) 22 times a day, not 24.
public class Solution {
    static int clockAngle(String t) {
        String[] p = t.split(":");
        int hh = Integer.parseInt(p[0]);
        int mm = Integer.parseInt(p[1]);
        double minute = mm * 6.0;
        double hour = (hh % 12) * 30.0 + mm * 0.5;
        double diff = Math.abs(hour - minute);
        if (diff > 180.0) diff = 360.0 - diff;
        return (int) Math.round(diff);
    }

    public static void main(String[] args) {
        System.out.println(clockAngle("12:00"));
        System.out.println(clockAngle("3:30"));
        System.out.println(clockAngle("9:00"));
    }
}
