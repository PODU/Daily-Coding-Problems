// Clock angle: minute=mm*6, hour=(hh%12)*30+mm*0.5, take min(diff,360-diff), round. O(1) time/space.
public class Solution {
    static int clockAngle(int hh, int mm) {
        double minute = mm * 6.0;
        double hour = (hh % 12) * 30.0 + mm * 0.5;
        double diff = Math.abs(hour - minute);
        diff = Math.min(diff, 360.0 - diff);
        return (int) Math.floor(diff + 0.5);
    }

    public static void main(String[] args) {
        System.out.println(clockAngle(12, 30));
        System.out.println(clockAngle(3, 15));
    }
}
