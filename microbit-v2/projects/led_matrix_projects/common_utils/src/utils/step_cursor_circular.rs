use crate::frame::Direction;

pub struct StepCursorCircular {
    initial_value: i16,
    step_size: i16,
    current_value: i16,
}

impl StepCursorCircular {
    pub fn new(initial_value: i16, step_size: i16) -> Self {
        assert!(
            initial_value > 0,
            "initial_value should be greater than zero"
        );
        assert!(step_size > 0, "step_size should be greater than zero");
        Self {
            initial_value,
            step_size,
            current_value: initial_value,
        }
    }

    pub fn current_value(&self) -> i16 {
        self.current_value
    }

    pub fn move_step(&mut self, direction: Direction) {
        match direction {
            Direction::Left => {
                self.current_value -= self.step_size;
                if self.current_value < 0 {
                    self.current_value = self.initial_value;
                }
            }
            Direction::Right => {
                self.current_value += self.step_size;
                if self.current_value > self.initial_value {
                    self.current_value = 0;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_circular_step_cursor(initial_value: i16, step_size: i16) -> StepCursorCircular {
        StepCursorCircular::new(initial_value, step_size)
    }

    #[test]
    #[should_panic(expected = "initial_value should be greater than zero")]
    fn should_panic_when_initial_value_less_than_or_equal_zero() {
        get_circular_step_cursor(-1, 100);
    }

    #[test]
    #[should_panic(expected = "step_size should be greater than zero")]
    fn should_panic_when_step_size_value_less_than_or_equal_zero() {
        get_circular_step_cursor(1000, 0);
    }

    #[test]
    fn should_return_current_value_for_circular_cursor() {
        let step_cursor = get_circular_step_cursor(1000, 200);
        let current_value = step_cursor.current_value();
        assert_eq!(1000, current_value);
    }

    #[test]
    fn should_step_down_current_value_for_circular_cursor() {
        let mut step_cursor = get_circular_step_cursor(1000, 200);
        for _ in 0..2 {
            step_cursor.move_step(Direction::Left);
        }
        let current_value = step_cursor.current_value();
        assert_eq!(600, current_value);
    }

    #[test]
    fn should_circular_rotate_current_value_when_step_down_past_zero() {
        let mut step_cursor = get_circular_step_cursor(1000, 200);
        for _ in 0..7 {
            step_cursor.move_step(Direction::Left);
        }
        let current_value = step_cursor.current_value();
        assert_eq!(800, current_value);
    }

    #[test]
    fn should_step_up_current_value_for_circular_cursor() {
        let mut step_cursor = get_circular_step_cursor(1000, 200);
        for _ in 0..2 {
            step_cursor.move_step(Direction::Left);
        }
        for _ in 0..2 {
            step_cursor.move_step(Direction::Right);
        }
        let current_value = step_cursor.current_value();
        assert_eq!(1000, current_value);
    }

    #[test]
    fn should_circular_rotate_current_value_when_step_up_past_initial_value() {
        let mut step_cursor = get_circular_step_cursor(1000, 200);
        for _ in 0..2 {
            step_cursor.move_step(Direction::Right);
        }
        let current_value = step_cursor.current_value();
        assert_eq!(200, current_value);
    }
}
