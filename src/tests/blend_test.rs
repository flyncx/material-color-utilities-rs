#[cfg(test)]
mod harmonize {
    use crate::blend::blend::Blend;

    const RED: i64 = 0xffff0000;
    const BLUE: i64 = 0xff0000ff;
    const GREEN: i64 = 0xff00ff00;
    const YELLOW: i64 = 0xffffff00;

    #[test]
    fn red_to_blue() {
        let answer = Blend::harmonize(RED, BLUE);
        assert_eq!(answer, 0xffFB0057)
    }
    #[test]
    fn red_to_green() {
        let answer = Blend::harmonize(RED, GREEN);
        assert_eq!(answer, (0xffD85600));
    }
    #[test]
    fn red_to_yellow() {
        let answer = Blend::harmonize(RED, YELLOW);
        assert_eq!(answer, (0xffD85600));
    }
    #[test]
    fn blue_to_green() {
        let answer = Blend::harmonize(BLUE, GREEN);
        assert_eq!(answer, (0xff0047A3));
    }
    #[test]
    fn blue_to_red() {
        let answer = Blend::harmonize(BLUE, RED);
        assert_eq!(answer, (0xff5700DC));
    }
    #[test]
    fn blue_to_yellow() {
        let answer = Blend::harmonize(BLUE, YELLOW);
        assert_eq!(answer, (0xff0047A3));
    }
    #[test]
    fn green_to_blue() {
        let answer = Blend::harmonize(GREEN, BLUE);
        assert_eq!(answer, (0xff00FC94));
    }
    #[test]
    fn green_to_red() {
        let answer = Blend::harmonize(GREEN, RED);
        assert_eq!(answer, (0xffB1F000));
    }
    #[test]
    fn green_to_yellow() {
        let answer = Blend::harmonize(GREEN, YELLOW);
        assert_eq!(answer, (0xffB1F000));
    }
    #[test]
    fn yellow_to_blue() {
        let answer = Blend::harmonize(YELLOW, BLUE);
        assert_eq!(answer, (0xffEBFFBA));
    }
    #[test]
    fn yellow_to_green() {
        let answer = Blend::harmonize(YELLOW, GREEN);
        assert_eq!(answer, (0xffEBFFBA));
    }
    #[test]
    fn yellow_to_red() {
        let answer = Blend::harmonize(YELLOW, RED);
        assert_eq!(answer, (0xffFFF6E3));
    }
}
