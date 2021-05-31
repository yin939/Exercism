#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Default, Debug)]
pub struct BowlingGame {
    frame: u16,              // 第几轮
    frame_first_throw: bool, // 是否是每轮第一个球
    bonus: (u16, u16),       // 当投中 10 分，记录是否 double 分数
    pins_up: u16,            // 每轮最高得分
    fill_balls: u16,         // 记录第 10 轮剩余投球次数（主要用于判断比赛是否完成）
    points: u16,             // 总分
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            pins_up: 10,
            frame_first_throw: true,
            ..Default::default()
        }
    }

    fn finished(&self) -> bool {
        self.frame >= 10 && self.fill_balls == 0
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.finished() {
            return Err(Error::GameComplete);
        } else if self.pins_up < pins {
            return Err(Error::NotEnoughPinsLeft);
        }

        self.pins_up -= pins;
        self.points += pins * (1 + self.bonus.0);
        if self.fill_balls > 0 {
            self.fill_balls -= 1;
        }
        self.bonus = (self.bonus.1, 0);

        // 每轮第 1 球
        if self.frame_first_throw {
            // 第一次是否投了 10 分
            if self.pins_up == 0 {
                self.pins_up = 10;

                if self.frame == 9 {
                    self.fill_balls = 2;
                } else if self.frame < 9 {
                    // 记录后两次分数需要 double
                    // +1 我实验了是处理连续 10 分的情况
                    self.bonus = (self.bonus.0 + 1, 1);
                }
                self.frame += 1;
            } else {
                if self.frame == 10 {
                    self.frame += 1;
                }
                self.frame_first_throw = false;
            }
        }
        // 每轮第 2/3? 球
        else {
            if self.pins_up == 0 {
                if self.frame == 9 {
                    self.fill_balls = 1;
                } else if self.frame < 9 {
                    self.bonus = (self.bonus.0 + 1, 0);
                }
            }
            self.pins_up = 10;
            self.frame += 1;
            self.frame_first_throw = true;
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.finished() {
            return None;
        }

        Some(self.points)
    }
}
