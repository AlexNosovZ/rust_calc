const EPS: f64 = 10e-10;
#[derive(Debug)]
pub enum OpError {
    Overflow,
    Zerodivide,
}

pub fn add<T, U>(left: T, right: U) -> f64
where
    T: Into<f64>,
    U: Into<f64>,
{
    left.into() + right.into()
}
pub fn sub<T, U>(left: T, right: U) -> f64
where
    T: Into<f64>,
    U: Into<f64>,
{
    left.into() - right.into()
}
pub fn mul<T, U>(left: T, right: U) -> Result<f64, OpError>
where
    T: Into<f64> + Clone,
    U: Into<f64> + Clone,
{
    if f64::MAX - left.clone().into() < EPS || f64::MAX - right.clone().into() < EPS {
        return Err(OpError::Overflow);
    }
    Ok(left.into() * right.into())
}
pub fn div<T, U>(left: T, right: U) -> Result<f64, OpError>
where
    T: Into<f64> + Clone,
    U: Into<f64> + Clone,
{
    if left.clone().into() - 0.0 < EPS {
        return Err(OpError::Zerodivide);
    }
    Ok(left.into() / right.into())
}

pub fn is_oper(c: char) -> bool {
    matches!(c, '+' | '-' | '*' | '/')
}

#[cfg(test)]
mod tests {
    const EPS: f64 = 10e-10;
    use crate::{add, div, mul, sub, is_oper};
    #[test]
    fn add_works() {
        let result = add(2, 2);
        assert!(result - 4.0 < EPS);
        let result = add(2.1, 2.2);
        assert!(result - 4.3 < EPS);
        let result = add(1.1, 2);
        assert!(result - 3.3 < EPS);
    }
    #[test]
    fn sub_works() {
        let result = sub(5, 3);
        assert!(result - 2.0 < EPS);
        assert!(2.1 - sub(5.2, 3.1) < EPS);
    }
    #[test]
    fn mul_works() {
        let result = mul(5, 3).unwrap();
        assert!(result - 15.0 < EPS);
        assert!(mul(4, 5).unwrap() - 20.0 < EPS);
    }
    #[test]
    fn div_works() {
        let result = div(10.0, 5.0).unwrap();
        assert!(result - 2.0 < EPS);
    }
    #[test]
    fn is_oper_works(){
        assert_eq!(is_oper('+'), true);
        assert_eq!(is_oper('-'), true);
        assert_eq!(is_oper('*'), true);
        assert_eq!(is_oper('/'), true);
    }
}
