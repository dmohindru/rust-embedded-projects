use crate::frame::Direction;

pub struct StepCursorCircular {
    max_value: i16,
    step_size: i16,
    num_steps: i16,
    current_step: i16,
}

impl StepCursorCircular {
    pub fn new(max_value: i16, min_value: i16, step_size: i16, num_steps: i16) -> Self {
        assert!(
            max_value - step_size * (num_steps - 1) == min_value,
            "Invalid values for step size and num_steps to reach min_value"
        );

        Self {
            max_value,
            step_size,
            num_steps,
            current_step: 0,
        }
    }

    pub fn current_value(&self) -> i16 {
        self.max_value - self.current_step * self.step_size
    }

    pub fn move_step(&mut self, direction: Direction) {
        match direction {
            Direction::Left => {
                self.current_step = (self.current_step + 1) % self.num_steps;
            }
            Direction::Right => {
                self.current_step = (self.current_step + self.num_steps - 1) % self.num_steps;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_circular_step_cursor(
        max_value: i16,
        min_value: i16,
        step_size: i16,
        num_steps: i16,
    ) -> StepCursorCircular {
        StepCursorCircular::new(max_value, min_value, step_size, num_steps)
    }

    #[test]
    #[should_panic(expected = "Invalid values for step size and num_steps to reach min_value")]
    fn should_panic_when_invalid_parameters_passed() {
        get_circular_step_cursor(1000, 500, 250, 4);
    }

    #[test]
    fn should_return_current_value_for_circular_cursor() {
        let step_cursor = get_circular_step_cursor(1000, 400, 200, 4);
        let current_value = step_cursor.current_value();
        assert_eq!(1000, current_value);
    }

    #[test]
    fn should_step_down_current_value_for_circular_cursor() {
        let mut step_cursor = get_circular_step_cursor(1000, 400, 200, 4);
        for _ in 0..2 {
            step_cursor.move_step(Direction::Left);
        }
        let current_value = step_cursor.current_value();
        assert_eq!(600, current_value);
    }

    #[test]
    fn should_step_down_min_value_for_circular_cursor() {
        let mut step_cursor = get_circular_step_cursor(1000, 400, 200, 4);
        for _ in 0..3 {
            step_cursor.move_step(Direction::Left);
        }
        let current_value = step_cursor.current_value();
        assert_eq!(400, current_value);
    }

    #[test]
    fn should_circular_rotate_current_value_when_step_down_past_zero() {
        let mut step_cursor = get_circular_step_cursor(1000, 400, 200, 4);
        for _ in 0..5 {
            step_cursor.move_step(Direction::Left);
        }
        let current_value = step_cursor.current_value();
        assert_eq!(800, current_value);
    }

    #[test]
    fn should_step_up_current_value_for_circular_cursor() {
        let mut step_cursor = get_circular_step_cursor(1000, 400, 200, 4);
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
        let mut step_cursor = get_circular_step_cursor(1000, 400, 200, 4);
        for _ in 0..2 {
            step_cursor.move_step(Direction::Right);
        }
        let current_value = step_cursor.current_value();
        assert_eq!(600, current_value);
    }
}
