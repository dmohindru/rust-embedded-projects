pub struct BouncingBallSnapshot<'a> {
    ball_coordinates: (&'a Ball, &'a Ball),
    radius: usize,
}

pub struct Ball {
    x: i32,
    y: i32,
    x_dir: i32,
    y_dir: i32,
}
pub struct BouncingBallCore<const WIDTH: usize, const HEIGHT: usize> {
    radius: usize,
    first_ball: Ball,
    second_ball: Ball,
    step_size: usize,
}

impl<const WIDTH: usize, const HEIGHT: usize> BouncingBallCore<WIDTH, HEIGHT> {
    pub fn new(radius: usize, step_size: usize) -> Self {
        let center_y: i32 = HEIGHT as i32 / 2;
        let center_one_x = WIDTH as i32 / 4;
        let center_two_x = center_one_x * 3;
        let ball_one = Ball {
            x: center_one_x,
            y: center_y,
            x_dir: 1,
            y_dir: -1,
        };

        let ball_two = Ball {
            x: center_two_x,
            y: center_y,
            x_dir: -1,
            y_dir: 1,
        };

        Self {
            radius,
            first_ball: ball_one,
            second_ball: ball_two,
            step_size,
        }
    }

    pub fn tick(&mut self) {
        let ball_one_y_edge = Self::get_ball_y_edge(&self.first_ball, self.radius);
        let ball_two_y_edge = Self::get_ball_y_edge(&self.second_ball, self.radius);
        let ball_one_x_edge = Self::get_ball_x_edge(&self.first_ball, self.radius);
        let ball_two_x_edge = Self::get_ball_x_edge(&self.second_ball, self.radius);

        Self::handle_edge_bounce(&mut self.first_ball, ball_one_x_edge, ball_one_y_edge);
        Self::handle_edge_bounce(&mut self.second_ball, ball_two_x_edge, ball_two_y_edge);

        Self::handle_edge_over_flow(&mut self.first_ball, self.radius, self.step_size);
        Self::handle_edge_over_flow(&mut self.second_ball, self.radius, self.step_size);

        // if ball_one_y_edge < 0 {
        //     self.first_ball.y = self.radius as i32;
        // } else if ball_one_y_edge > (HEIGHT - 1) as i32 {
        //     self.first_ball.y = (HEIGHT - 1 - self.radius) as i32;
        // } else {
        //     self.first_ball.y += self.first_ball.y_dir * self.step_size as i32;
        // }

        // if ball_one_x_edge < 0 {
        //     self.first_ball.x = self.radius as i32;
        // } else if ball_one_x_edge > (WIDTH - 1) as i32 {
        //     self.first_ball.x = (WIDTH - 1 - self.radius) as i32;
        // } else {
        //     self.first_ball.x += self.first_ball.x_dir * self.step_size as i32;
        // }

        // if ball_two_y_edge < 0 {
        //     self.second_ball.y = self.radius as i32;
        // } else if ball_two_y_edge > (HEIGHT - 1) as i32 {
        //     self.second_ball.y = (HEIGHT - 1 - self.radius) as i32;
        // } else {
        //     self.second_ball.y += self.second_ball.y_dir * self.step_size as i32;
        // }

        // if ball_two_x_edge < 0 {
        //     self.second_ball.x = self.radius as i32;
        // } else if ball_two_x_edge > (WIDTH - 1) as i32 {
        //     self.second_ball.x = (WIDTH - 1 - self.radius) as i32;
        // } else {
        //     self.second_ball.x += self.second_ball.x_dir * self.step_size as i32;
        // }
    }

    fn get_ball_y_edge(ball: &Ball, radius: usize) -> i32 {
        ball.y + ball.y_dir * radius as i32
    }

    fn get_ball_x_edge(ball: &Ball, radius: usize) -> i32 {
        ball.x + ball.x_dir * radius as i32
    }

    fn handle_edge_bounce(ball: &mut Ball, ball_x_edge: i32, ball_y_edge: i32) {
        if ball_y_edge <= 0 || ball_y_edge >= (HEIGHT - 1) as i32 {
            ball.y_dir *= -1;
        }

        if ball_x_edge <= 0 || ball_x_edge >= (WIDTH - 1) as i32 {
            ball.x_dir *= -1;
        }
    }

    fn handle_edge_over_flow(ball: &mut Ball, radius: usize, step_size: usize) {
        let ball_y_edge = ball.y + 2 * ball.y_dir * radius as i32;
        let ball_x_edge = ball.x + 2 * ball.x_dir * radius as i32;
        if ball_y_edge < 0 {
            ball.y = radius as i32;
        } else if ball_y_edge > (HEIGHT - 1) as i32 {
            ball.y = (HEIGHT - 1 - radius) as i32;
        } else {
            ball.y += ball.y_dir * step_size as i32;
        }

        if ball_x_edge < 0 {
            ball.x = radius as i32;
        } else if ball_x_edge > (WIDTH - 1) as i32 {
            ball.x = (WIDTH - 1 - radius) as i32;
        } else {
            ball.x += ball.x_dir * step_size as i32;
        }
    }

    pub fn snapshot(&self) -> BouncingBallSnapshot<'_> {
        BouncingBallSnapshot {
            ball_coordinates: (&self.first_ball, &self.second_ball),
            radius: self.radius,
        }
    }
}

#[cfg(test)]
impl<const WIDTH: usize, const HEIGHT: usize> BouncingBallCore<WIDTH, HEIGHT> {
    pub fn set_ball_coordinates(&mut self, ball_one: Ball, ball_two: Ball) {
        self.first_ball = ball_one;
        self.second_ball = ball_two;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BALL_RADIUS: usize = 5;
    const STEP_SIZE: usize = 5;

    #[test]
    fn should_have_balls_center_at_right_location_for_new_board() {
        let board = get_new_board();
        let board_snapshot = board.snapshot();
        let coordinates = get_start_coordinates();

        // First ball
        assert_eq!(&coordinates.0, &board_snapshot.ball_coordinates.0.x);
        assert_eq!(&coordinates.1, &board_snapshot.ball_coordinates.0.y);

        // Second ball
        assert_eq!(&(coordinates.0 * 3), &board_snapshot.ball_coordinates.1.x);
        assert_eq!(&coordinates.1, &board_snapshot.ball_coordinates.1.y);
    }

    #[test]
    fn should_move_balls_in_required_direction() {
        let mut board = get_new_board();
        for _ in 0..5 {
            board.tick();
        }
        /*
        Starting point for single ball
        x: 32, y: 32
        Next point calculation
        x1 = x + (x_dir * step_size)
        y1 = y + (y_dir * step_size)
        */
        let expected_ball_one_x = 57;
        let expected_ball_one_y = 7;

        let expected_ball_two_x = 71;
        let expected_ball_two_y = 57;

        assert_ball_coordinates(
            board,
            expected_ball_one_x,
            expected_ball_one_y,
            expected_ball_two_x,
            expected_ball_two_y,
        );
    }

    #[test]
    fn should_bounce_ball_off_top_and_bottom_edge() {
        let mut board = get_new_board();

        let ball_one_start = Ball {
            x: 30,
            y: 10,
            x_dir: 1,
            y_dir: -1,
        };

        let ball_two_start = Ball {
            x: 50,
            y: 53,
            x_dir: -1,
            y_dir: 1,
        };
        board.set_ball_coordinates(ball_one_start, ball_two_start);
        for _ in 0..3 {
            board.tick();
        }

        let expected_ball_one_x = 45;
        let expected_ball_one_y = 15;

        let expected_ball_two_x = 35;
        let expected_ball_two_y = 48;

        assert_ball_coordinates(
            board,
            expected_ball_one_x,
            expected_ball_one_y,
            expected_ball_two_x,
            expected_ball_two_y,
        );
    }

    #[test]
    fn should_bounce_ball_off_left_and_right_edge() {
        let mut board = get_new_board();

        let ball_one_start = Ball {
            x: 10,
            y: 30,
            x_dir: -1,
            y_dir: 1,
        };

        let ball_two_start = Ball {
            x: 117,
            y: 30,
            x_dir: 1,
            y_dir: 1,
        };
        board.set_ball_coordinates(ball_one_start, ball_two_start);
        for _ in 0..3 {
            board.tick();
        }

        let expected_ball_one_x = 15;
        let expected_ball_one_y = 45;

        let expected_ball_two_x = 112;
        let expected_ball_two_y = 45;

        assert_ball_coordinates(
            board,
            expected_ball_one_x,
            expected_ball_one_y,
            expected_ball_two_x,
            expected_ball_two_y,
        );
    }

    #[test]
    fn should_move_to_top_bottom_edge_if_step_size_crosses_boundary() {
        let mut board = get_new_board();

        let ball_one_start = Ball {
            x: 30,
            y: 8,
            x_dir: 1,
            y_dir: -1,
        };

        let ball_two_start = Ball {
            x: 30,
            y: 55,
            x_dir: -1,
            y_dir: 1,
        };
        board.set_ball_coordinates(ball_one_start, ball_two_start);
        for _ in 0..2 {
            board.tick();
        }

        let expected_ball_one_x = 40;
        let expected_ball_one_y = 10;

        let expected_ball_two_x = 20;
        let expected_ball_two_y = 53;

        assert_ball_coordinates(
            board,
            expected_ball_one_x,
            expected_ball_one_y,
            expected_ball_two_x,
            expected_ball_two_y,
        );
    }

    #[test]
    fn should_move_to_left_right_edge_if_step_size_crosses_boundary() {
        let mut board = get_new_board();

        let ball_one_start = Ball {
            x: 8,
            y: 30,
            x_dir: -1,
            y_dir: 1,
        };

        let ball_two_start = Ball {
            x: 119,
            y: 30,
            x_dir: 1,
            y_dir: 1,
        };
        board.set_ball_coordinates(ball_one_start, ball_two_start);
        for _ in 0..2 {
            board.tick();
        }

        let expected_ball_one_x = 10;
        let expected_ball_one_y = 40;

        let expected_ball_two_x = 117;
        let expected_ball_two_y = 40;

        assert_ball_coordinates(
            board,
            expected_ball_one_x,
            expected_ball_one_y,
            expected_ball_two_x,
            expected_ball_two_y,
        );
    }

    #[test]
    fn should_bounce_ball_off_top_left_and_bottom_right_corner() {
        let mut board = get_new_board();

        let ball_one_start = Ball {
            x: 10,
            y: 10,
            x_dir: -1,
            y_dir: -1,
        };

        let ball_two_start = Ball {
            x: 117,
            y: 53,
            x_dir: 1,
            y_dir: 1,
        };
        board.set_ball_coordinates(ball_one_start, ball_two_start);
        for _ in 0..3 {
            board.tick();
        }

        let expected_ball_one_x = 15;
        let expected_ball_one_y = 15;

        let expected_ball_two_x = 112;
        let expected_ball_two_y = 48;

        assert_ball_coordinates(
            board,
            expected_ball_one_x,
            expected_ball_one_y,
            expected_ball_two_x,
            expected_ball_two_y,
        );
    }

    #[test]
    fn should_bounce_ball_off_top_right_and_bottom_left_corner() {
        let mut board = get_new_board();

        let ball_one_start = Ball {
            x: 117,
            y: 10,
            x_dir: 1,
            y_dir: -1,
        };

        let ball_two_start = Ball {
            x: 10,
            y: 53,
            x_dir: -1,
            y_dir: 1,
        };
        board.set_ball_coordinates(ball_one_start, ball_two_start);
        for _ in 0..3 {
            board.tick();
        }

        let expected_ball_one_x = 112;
        let expected_ball_one_y = 15;

        let expected_ball_two_x = 15;
        let expected_ball_two_y = 48;

        assert_ball_coordinates(
            board,
            expected_ball_one_x,
            expected_ball_one_y,
            expected_ball_two_x,
            expected_ball_two_y,
        );
    }

    #[test]
    fn should_bounce_off_each_other_on_collision() {
        todo!()
    }

    fn get_new_board() -> BouncingBallCore<128, 64> {
        BouncingBallCore::<128, 64>::new(BALL_RADIUS, STEP_SIZE)
    }

    fn get_start_coordinates() -> (i32, i32) {
        let x = 128 / 4;
        let y = 64 / 2;
        (x, y)
    }

    fn assert_ball_coordinates(
        board: BouncingBallCore<128, 64>,
        expected_ball_one_x: i32,
        expected_ball_one_y: i32,
        expected_ball_two_x: i32,
        expected_ball_two_y: i32,
    ) {
        let board_snapshot = board.snapshot();

        // First ball
        assert_eq!(expected_ball_one_x, board_snapshot.ball_coordinates.0.x);
        assert_eq!(expected_ball_one_y, board_snapshot.ball_coordinates.0.y);

        // Second ball
        assert_eq!(expected_ball_two_x, board_snapshot.ball_coordinates.1.x);
        assert_eq!(expected_ball_two_y, board_snapshot.ball_coordinates.1.y);
    }
}
