#[allow(unused)]
pub trait Err<E> {
    #[allow(non_snake_case)]
    fn Err(&self, error: E) -> Result<(), E>;
}

impl<E> Err<E> for bool {
    #[inline(always)]
    fn Err(&self, error: E) -> Result<(), E> {
        if *self { Err(error) } else { Ok(()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // A dummy custom error type to ensure the trait works with non-primitive types
    #[derive(Debug, PartialEq, Eq)]
    struct CustomError(&'static str);

    #[test]
    fn test_err_when_true() {
        let condition = true;
        let result = condition.Err(CustomError("Something went wrong"));

        // When true, it must return the Err variant holding our error
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CustomError("Something went wrong"));
    }

    #[test]
    fn test_ok_when_false() {
        let condition = false;
        // Specifying the error type via generic parameter so type inference succeeds
        let result = condition.Err(CustomError("No error should happen"));

        // When false, it must return Ok(())
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ());
    }

    // A helper function simulating real-world usage of the ? operator
    fn check_age(age: u32) -> Result<&'static str, &'static str> {
        // If true, this returns early with Err("Too young")
        (age < 18).Err("Too young")?;

        // If the above didn't exit early, execution continues normally
        Ok("Access granted")
    }

    #[test]
    fn test_early_exit_control_flow() {
        // Test early exit path triggering
        let under_age_result = check_age(16);
        assert_eq!(under_age_result, Err("Too young"));

        // Test normal path continuation
        let adult_result = check_age(21);
        assert_eq!(adult_result, Ok("Access granted"));
    }

    #[test]
    fn test_inline_expression_usage() {
        // Verifies that inline logic works seamlessly without creating a variable first
        let x = 10;
        let res: Result<(), &str> = (x > 5).Err("x is too big");
        assert_eq!(res, Err("x is too big"));
    }
}
