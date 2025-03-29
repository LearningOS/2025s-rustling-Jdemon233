/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T>(array: &mut [T])
where T:Ord+Clone,{
	//TODO
    q_sort(array);
}
fn partition<T>(array: &mut [T]) -> usize
where T:Ord+Clone,
{
    let pivot = array[0].clone();
    // 8,4,9,5,7,9
    let mut i = 0;
    for j in 1..array.len(){
        if array[j] < pivot{
            i += 1;
            array.swap(i,j)
        }
    }
    array.swap(0,i);
    i

}
fn q_sort<T>(array: &mut [T])
where T:Ord+Clone,{
    if array.len()<=1{
        return;
    }

    let pivot_i = partition(array);
    q_sort(&mut array[0..pivot_i]);
    q_sort(&mut array[pivot_i+1..]);

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
    }
}