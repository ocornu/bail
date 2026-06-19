#[allow(unused)]
pub trait None {
    #[allow(non_snake_case)]
    fn None(&self) -> Option<()>;
}

impl None for bool {
    #[inline(always)]
    fn None(&self) -> Option<()> {
        if *self { None } else { Some(()) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn none_when_true() {
        let condition = true;
        let result = condition.None();

        // When true, it must return the None variant
        assert!(result.is_none());
    }

    #[test]
    fn some_when_false() {
        let condition = false;
        let result = condition.None();

        // When false, it must return Some(())
        assert!(result.is_some());
        assert_eq!(result.unwrap(), ());
    }

    // A helper function simulating real-world usage of ? with Option
    fn find_item(id: u32) -> Option<&'static str> {
        // If true, this returns early from the function with None
        (id == 0).None()?;

        // If execution continues, return the valid item
        Some("Valid Item")
    }

    #[test]
    fn early_exit_control_flow() {
        // Test early exit path for Option
        let invalid_item = find_item(0);
        assert_eq!(invalid_item, None);

        // Test normal path continuation for Option
        let valid_item = find_item(42);
        assert_eq!(valid_item, Some("Valid Item"));
    }
}
