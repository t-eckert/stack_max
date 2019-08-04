/// # StackMax
///
/// A stack of `i32` that provides O(1) access to the maximum number in the stack.
///
/// Example:
///
/// ``` rust
/// let mut stack_max = StackMax::new();
/// stack_max.push(9);
/// assert_eq!(&9, stack_max.peek().unwrap());
/// assert_eq!(&9, stack_max.get_max().unwrap());
///
/// stack_max.push(4);
/// assert_eq!(&4, stack_max.peek().unwrap());
/// assert_eq!(&9, stack_max.get_max().unwrap());
///
/// stack_max.push(14);
/// assert_eq!(&14, stack_max.peek().unwrap());
/// assert_eq!(&14, stack_max.get_max().unwrap());
///
/// stack_max.pop();
/// assert_eq!(&4, stack_max.peek().unwrap());
/// assert_eq!(&9, stack_max.get_max().unwrap());
///
/// stack_max.pop();
/// assert_eq!(&9, stack_max.peek().unwrap());
/// assert_eq!(&9, stack_max.get_max().unwrap());
/// ```
///
pub struct StackMax {
    numbers: Vec<i32>,
    maximums: Vec<i32>,
}

impl StackMax {
    /// Returns a new instance of StackMax
    pub fn new() -> StackMax {
        StackMax {
            numbers: vec![],
            maximums: vec![],
        }
    }

    /// Appends integer to the top of StackMax.
    /// If int is larger than current max, update max.
    pub fn push(&mut self, number: i32) {
        self.numbers.push(number);

        // Push number to maximums
        //  if maximums is empty
        //  or maximum is less than number
        match self.get_max() {
            None => self.maximums.push(number),
            Some(maximum) => {
                if number >= *maximum {
                    self.maximums.push(number)
                }
            }
        };
    }

    /// Returns `Option<&i32>`  
    /// Some containing last value pushed to stack without removing value   
    /// None if stack is empty  
    pub fn peek(&self) -> Option<&i32> {
        self.numbers.last()
    }

    /// Returns `Option<i32>`  
    /// Some containing last value pushed to stack removing value
    /// None if stack is empty
    ///
    /// Updates max value if popped value is max
    pub fn pop(&mut self) -> Option<i32> {
        let last_number = self.numbers.pop();

        if last_number.is_some() && last_number.unwrap() == *self.get_max().unwrap() {
            self.maximums.pop();
        }

        last_number
    }

    /// Returns `usize` representing the length of the stack
    pub fn len(&self) -> usize {
        self.numbers.len()
    }

    /// Returns `Option<&i32>`
    /// Some containing max value in stack
    /// None if stack is empty
    pub fn get_max(&self) -> Option<&i32> {
        self.maximums.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_constructs() {
        let _stack_max = StackMax::new();
    }

    #[test]
    fn can_push() {
        // Given
        let test_number = 5;
        let expected_length = 1;

        // When
        let mut stack_max = StackMax::new();
        stack_max.push(test_number);

        // Then
        assert_eq!(expected_length, stack_max.len());
    }

    #[test]
    fn can_peek() {
        // Given
        let test_number = 5;

        // When
        let mut stack_max = StackMax::new();
        stack_max.push(test_number);

        // Then
        assert_eq!(&test_number, stack_max.peek().unwrap());
    }

    #[test]
    fn can_pop() {
        // Given
        let test_number = 5;

        // When
        let mut stack_max = StackMax::new();
        stack_max.push(test_number);

        // Then
        assert_eq!(test_number, stack_max.pop().unwrap());
        assert_eq!(0, stack_max.len());
    }

    #[test]
    fn it_updates_max() {
        // Given
        let first = 9;
        let second = 4;
        let third = 11;
        let fourth = 8;

        // When
        let mut stack_max = StackMax::new();

        // Then
        assert_eq!(0, stack_max.len(), "not empty at construction");

        // When
        stack_max.push(first);

        // Then
        assert_eq!(1, stack_max.len(), "length not 1 after 9 pushed");
        assert_eq!(
            &first,
            stack_max.peek().unwrap(),
            "peek not 9 after 9 pushed"
        );
        assert_eq!(
            &first,
            stack_max.get_max().unwrap(),
            "max not 9 after 9 pushed"
        );

        // When
        stack_max.push(second);

        // Then
        assert_eq!(2, stack_max.len(), "length not 2 after 9, 4 pushed");
        assert_eq!(
            &second,
            stack_max.peek().unwrap(),
            "peek not 4 after 9, 4 pushed"
        );
        assert_eq!(
            &first,
            stack_max.get_max().unwrap(),
            "max not 9 after 9, 4 pushed"
        );

        // When
        stack_max.push(third);

        // Then
        assert_eq!(3, stack_max.len(), "length not 3 after 3 pushes");
        assert_eq!(
            &third,
            stack_max.peek().unwrap(),
            "peek not 11 after 9, 4, 11 pushed"
        );
        assert_eq!(
            &third,
            stack_max.get_max().unwrap(),
            "max not 11 after 9, 4, 11 pushed"
        );

        // When
        stack_max.push(fourth);

        // Then
        assert_eq!(4, stack_max.len(), "length not 4 after 9, 4, 11, 8 pushed");
        assert_eq!(
            &fourth,
            stack_max.peek().unwrap(),
            "peek not 8 after 9, 4, 11, 8 pushed"
        );
        assert_eq!(
            &third,
            stack_max.get_max().unwrap(),
            "max not 11 after 9, 4, 11, 8 pushed"
        );

        // When
        let first_popped = stack_max.pop().unwrap();

        // Then
        assert_eq!(
            3,
            stack_max.len(),
            "length not 3 after 9, 4, 11, 8 pushed 8 popped"
        );
        assert_eq!(fourth, first_popped, "first_popped not 8");
        assert_eq!(
            &third,
            stack_max.get_max().unwrap(),
            "max not 11 after 9, 4, 11, 8 pushed 8 popped"
        );

        // When
        let second_popped = stack_max.pop().unwrap();

        // Then
        assert_eq!(
            2,
            stack_max.len(),
            "length not 2 after 9, 4, 11, 8 pushed 8, 11 popped"
        );
        assert_eq!(third, second_popped, "second_popped not 11");
        assert_eq!(
            &first,
            stack_max.get_max().unwrap(),
            "max not 9 after 9, 4, 11, 8 pushed 8, 11 popped"
        );

        // When
        let third_popped = stack_max.pop().unwrap();

        // Then
        assert_eq!(
            1,
            stack_max.len(),
            "length not 1 after 9, 4, 11, 8 pushed 8, 11, 4 popped"
        );
        assert_eq!(second, third_popped, "third_popped not 4");
        assert_eq!(
            &first,
            stack_max.get_max().unwrap(),
            "max not 9 after 9, 4, 11, 8 pushed 8, 11, 4 popped"
        );

        // When
        let fourth_popped = stack_max.pop().unwrap();

        // Then
        assert_eq!(
            0,
            stack_max.len(),
            "length not 0 after 9, 4, 11, 8 pushed 8, 11, 4, 9 popped"
        );
        assert_eq!(first, fourth_popped, "fourth_popped not 9");
    }
}
