pub mod calc1;
pub mod calc2;

#[cfg(test)]
mod tests {
    use super::calc1::{add, sub};
    use super::calc2::{multiply, rate};

    #[test]
    fn teste_add() {
        assert_eq!(add(10, 20), 30);
    }

    #[test]
    fn teste_add_zero() {
        assert_eq!(add(0, 20), 20);
        assert_eq!(add(20, 0), 20);
    }

    #[test]
    fn teste_add_grandes() {
        assert_eq!(add(1_000_000_000, 1_000_000_000), 2_000_000_000);
    }

    #[test]
    fn teste_sub() {
        assert_eq!(sub(20, 10), 10);
    }

    #[test]
    fn teste_sub_zero() {
        assert_eq!(sub(20, 0), 20);
        assert_eq!(sub(0, 20), 0);
    }

    #[test]
    fn teste_multiply() {
        assert_eq!(multiply(2, 3), 6);
    }
    
    #[test]
    fn teste_multiply_zero() {
        assert_eq!(multiply(0, 5), 0);
        assert_eq!(multiply(5, 0), 0);
    }

    #[test]
    fn teste_multiply_grandes() {
        assert_eq!(multiply(1_000_000, 1_000), 1_000_000_000);
    }

    #[test]
    fn teste_rate() {
        assert_eq!(rate(100, 5), 20);
    }

    #[test]
    fn teste_rate_zero() {
        assert_eq!(rate(100, 0), 0);
        assert_eq!(rate(0, 5), 0);
    }

    #[test]
    fn teste_rate_fracionario() {
        assert_eq!(rate(105, 2), 52);
    }
}
