// Clock angle: minute=mm*6, hour=(hh%12)*30+mm*0.5, take min(diff,360-diff), round. O(1) time/space.
// Bonus: hands overlap (angle 0) 11 times per 12 hours, at t = (12/11)*k hours for k=0..10.
public class Solution {
    static int clockAngle(int hh, int mm) {
        double minuteAngle = mm * 6.0;
        double hourAngle = (hh % 12) * 30.0 + mm * 0.5;
        double diff = Math.abs(hourAngle - minuteAngle);
        double angle = Math.min(diff, 360.0 - diff);
        return (int) Math.round(angle);
    }

    public static void main(String[] args) {
        String t = "3:15";
        String[] parts = t.split(":");
        int hh = Integer.parseInt(parts[0].trim());
        int mm = Integer.parseInt(parts[1].trim());
        System.out.println(clockAngle(hh, mm));
    }
}
