use std::collections::HashMap;
use itertools::Itertools;

#[derive(Debug)]
pub struct StatisticalInfo<T> {
    pub mo: Vec<T>, // trends
    pub me: f64, // median
    pub xmin: T, // minimum value
    pub xmax: T, // maximum value
    pub xsum: T, // sum
    pub xavg: f64, // average
    pub q1: f64, // first quartille
    pub q3: f64, // second quartille
    pub r: T, // scope
    pub qr: f64, // quartille scope
}

impl<'a, T> StatisticalInfo<T>
where
    T: 'a
        + Sized
        + Copy
        + Ord
        + Eq
        + Default
        + std::hash::Hash
        + std::iter::Sum<&'a T>
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::convert::Into<f64>,
{
    pub fn new(nums: &'a [T]) -> Self {
        let mut nums = nums.to_vec();
        nums.sort();
        let (min, max) = (Self::get_min(&nums), Self::get_max(&nums));
        let (q1, q3) = (Self::calc_first_quartille(&nums), Self::calc_third_quartille(&nums));
        let xsum = Self::calc_sum(&nums);

        Self {
            mo: Self::get_most_frequent_number(&nums),
            xavg: Self::calc_average(&xsum, nums.len()),
            xsum,
            me: Self::get_median(&nums),
            xmin: min, xmax: max,
            q1, q3,
            r: max - min,
            qr: q3 - q1,
        }
    }

    fn get_most_frequent_number(nums: &[T]) -> Vec<T> {
        // credit to Nikola Peshev 2023 ©® Mafs™
        // all rights reserved
        nums.iter()
            .fold(HashMap::<T, usize>::new(), |mut m, x| {
                *m.entry(*x).or_default() += 1;
                m
            })
            .iter()
            .max_set_by_key(|(_, v)| *v)
            .iter()
            .map(|(k, _)| **k)
            .collect()
    }

    fn calc_sum(nums: &[T]) -> T {
        // impossible to use .sum due to:
        // https://stackoverflow.com/questions/29459738/how-to-define-sum-over-vectors-or-iterators-in-a-generic-way
        nums.iter().fold(T::default(), |acc, &item| item + acc)
    }

    fn calc_average(sum: &T, len: usize) -> f64 {
        Into::<f64>::into(*sum) / len as f64
    }

    fn get_median(nums: &[T]) -> f64 {
        let len = nums.len();
        let half = len / 2;
        let is_even = |n| n % 2 == 0;

        match is_even(len) {
            true => (nums[half - 1] + nums[half]).into() / 2.0,
            false => nums[half].into(),
        }
    }

    fn get_max(nums: &[T]) -> T {
        *nums.last().expect("Sorted and not empty vec")
    }

    fn get_min(nums: &[T]) -> T {
        *nums.first().expect("Sorted and not empty vec")
    }

    fn calc_first_quartille(nums: &[T]) -> f64 {
        let half = nums.len() / 2;
        Self::get_median(&nums[0..half])
    }

    fn calc_third_quartille(nums: &[T]) -> f64 {
        let ceiled_half = (nums.len() as f64 / 2.0).ceil() as usize;
        Self::get_median(&nums[ceiled_half..nums.len()])
    }
}

#[cfg(test)]
mod tests {
    use crate::StatisticalInfo;

    #[test]
    fn test_one() {
        const DATA: [i32; 14] = [7, 6, 6, 5, 5, 5, 5, 4, 4, 4, 2, 2, 2, 1];
        let mut stat_info = StatisticalInfo::new(&DATA);
        stat_info.mo.sort();
        
        assert_eq!(stat_info.mo, vec![5]);
        assert_eq!(stat_info.xavg, 4.142857142857143);
        assert_eq!(stat_info.xsum, 58);
        assert_eq!(stat_info.me, 4.5);
        assert_eq!(stat_info.xmin, 1);
        assert_eq!(stat_info.xmax, 7);
        assert_eq!(stat_info.q1, 2f64);
        assert_eq!(stat_info.q3, 5f64);
        assert_eq!(stat_info.r, 6);
        assert_eq!(stat_info.qr, 3f64);
    }
}
