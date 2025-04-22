/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

use std::fmt::Debug;

fn quick_sort<T: Ord + Copy + Debug>(arr: &mut [T]) {
    fn pivot<T: Ord + Copy + Debug>(arr: &mut [T], s: usize, e: usize) -> usize {
        let v = arr[s];
        let (mut l, mut r) = (s+1, e);
        while l < r {
            while l < r && arr[r] > v {
                r-=1;
            }
            while l < r && arr[l] <= v {
                l+=1;
            }
            (arr[l], arr[r]) = (arr[r], arr[l]);
        }
        if arr[r] < arr[s] {
            (arr[s], arr[r]) = (arr[r], arr[s]);
            return r;
        }
        r - 1
    } 
    fn _quick_sort<T: Ord + Copy + Debug>(arr: &mut [T], s: usize, e: usize) {
        if s >= e {
            return ;
        }
        let m = pivot(arr, s, e);
        if m > 0 {
            _quick_sort(arr, s, m-1);
        }
        _quick_sort(arr, m+1, e);   
    }
    
    _quick_sort(arr, 0, arr.len() - 1);
}


fn sort<T: Ord + Copy + Debug>(array: &mut [T]){
    quick_sort(array);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
        // let mut vec = vec![99, 88, 77, 66];
        // println!("origin  vec: {:?}", vec);
        // sort(&mut vec);
        // assert_eq!(vec, vec![66, 77, 88, 99]);
    }
}