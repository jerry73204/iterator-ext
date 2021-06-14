mod and_then;
mod common;
mod map_err;
mod trait_;
mod try_filter;
mod try_filter_map;
mod try_flat_map;
mod try_flat_map_results;
mod try_flatten;
mod try_flatten_results;

pub use and_then::*;
pub use map_err::*;
pub use trait_::*;
pub use try_filter::*;
pub use try_filter_map::*;
pub use try_flat_map::*;
pub use try_flat_map_results::*;
pub use try_flatten::*;
pub use try_flatten_results::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_flatten_test() {
        let input = vec![Ok(vec![1usize, 2]), Err("err"), Ok(vec![3])];
        let output: Vec<_> = input.into_iter().try_flatten().collect();
        assert_eq!(output, vec![Ok(1), Ok(2), Err("err")]);
    }

    #[test]
    fn try_flat_map_test() {
        let input = vec![Ok(vec![1, 2]), Err("err"), Ok(vec![3])];
        let output: Vec<_> = input.into_iter().try_flat_map(Ok).collect();
        assert_eq!(output, vec![Ok(1usize), Ok(2), Err("err")]);
    }

    #[test]
    fn try_flatten_results_test_1() {
        let input = vec![
            Ok(vec![Ok(1usize), Ok(2)]),
            Ok(vec![Err("err"), Ok(3)]),
            Ok(vec![Ok(4)]),
        ];
        let output: Vec<_> = input.into_iter().try_flatten_results().collect();
        assert_eq!(output, vec![Ok(1usize), Ok(2), Err("err")]);
    }

    #[test]
    fn try_flatten_results_test_2() {
        let input = vec![Ok(vec![Ok(1usize), Ok(2)]), Err("err"), Ok(vec![Ok(3)])];
        let output: Vec<_> = input.into_iter().try_flatten_results().collect();
        assert_eq!(output, vec![Ok(1usize), Ok(2), Err("err")]);
    }

    #[test]
    fn try_flat_map_results_test_1() {
        let input = vec![
            Ok(vec![Ok(1usize), Ok(2)]),
            Ok(vec![Err("err"), Ok(3)]),
            Ok(vec![Ok(4)]),
        ];
        let output: Vec<_> = input.into_iter().try_flat_map_results(Ok).collect();
        assert_eq!(output, vec![Ok(1usize), Ok(2), Err("err")]);
    }

    #[test]
    fn try_flat_map_results_test_2() {
        let input = vec![Ok(vec![Ok(1usize), Ok(2)]), Err("err"), Ok(vec![Ok(3)])];
        let output: Vec<_> = input.into_iter().try_flat_map_results(Ok).collect();
        assert_eq!(output, vec![Ok(1usize), Ok(2), Err("err")]);
    }

    #[test]
    fn try_filter_test() {
        let input = vec![Ok(1usize), Ok(2), Ok(3), Err("err"), Ok(4)];
        let output: Vec<_> = input
            .into_iter()
            .try_filter(|val| Ok(val % 2 == 1))
            .collect();
        assert_eq!(output, vec![Ok(1usize), Ok(3), Err("err")]);
    }

    #[test]
    fn try_filter_map_test() {
        let input = vec![
            Ok(Some(1usize)),
            Ok(None),
            Ok(Some(3usize)),
            Err("err"),
            Ok(Some(4usize)),
        ];
        let output: Vec<_> = input.into_iter().try_filter_map(Ok).collect();
        assert_eq!(output, vec![Ok(1usize), Ok(3), Err("err")]);
    }

    #[test]
    fn and_then_test() {
        let input = vec![Ok(1isize), Ok(2), Err("err"), Ok(3)];
        let output: Vec<_> = input.into_iter().and_then(|val| Ok(-val)).collect();
        assert_eq!(output, vec![Ok(-1), Ok(-2), Err("err")]);
    }
}
