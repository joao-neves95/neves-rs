/// Splits the string on the first occurrence of the specified delimiter and
/// returns prefix before delimiter and suffix after delimiter.
///
/// # Example:
/// ```rust
/// let source_bytes: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9][..];
/// let delimiter: &[u8] = &[3, 4][..];
///
/// assert_eq!(
///     split_slice_once(source_bytes, delimiter),
///     Some((&[1, 2][..], &[5, 6, 7, 8, 9][..]))
/// );
/// ```
pub fn split_slice_once<'a, T>(source: &'a [T], delimiter: &'a [T]) -> Option<(&'a [T], &'a [T])>
where
    T: PartialEq,
{
    match find_first_index_in_slice(source, delimiter) {
        None => None,
        Some(first_idx) => Some((
            &source[0..first_idx],
            &source[first_idx + delimiter.len()..],
        )),
    }
}

/// If not found it returns `source.len()`.
pub fn find_first_index_in_slice<T>(source: &[T], query: &[T]) -> Option<usize>
where
    T: PartialEq,
{
    for i in 0..source.len() {
        if &source[i..i + query.len()] == query {
            return Some(i);
        }
    }

    None
}

pub fn copy_slice_into<T>(source: &[T], target: &mut [T])
where
    T: Copy,
{
    map_slice_into(source, target, |item| item)
}

pub fn map_slice_into<TIn, TOut, F>(source: &[TIn], target: &mut [TOut], converter: F)
where
    TIn: Copy,
    F: Fn(TIn) -> TOut,
{
    for i in 0..source.len() {
        target[i] = converter(source[i]);
    }
}

/// Copies `source` from index 0 into `target` from the inclusive `start_idx` index and until `end_idx`. <br/>
/// This function does not offer overflow protection, so the caller has to account for that and choose an appropriate `end` smaller than `source`.
pub fn copy_into_slice<T>(
    target: &mut [T],
    source: &[T],
    target_start_idx: usize,
    target_end_idx: usize,
) where
    T: Copy,
{
    let mut i_source: usize = 0;

    for i_target in target_start_idx..(target_end_idx + 1) {
        target[i_target] = source[i_source];
        i_source += 1;
    }
}

/// Copies `source` from index 0 into `target` from the inclusive `start` and stops when the result of the `until` closure is true. <br/>
/// This function does not offer overflow protection, so the caller has to account for that.
///
/// `target_start_idx`: start index of target to copy into.
/// `until`: closure with a predicate to stop the copy. Receives (current item, current target index, current source index).
pub fn copy_into_slice_until<T, F>(
    target: &mut [T],
    source: &[T],
    target_start_idx: usize,
    until: F,
) where
    T: Copy,
    F: Fn(T, usize, usize) -> bool,
{
    let mut i_target: usize = target_start_idx;
    let mut i_source: usize = 0;

    loop {
        let item = source[i_source];

        if until(item, i_target, i_source) {
            break;
        }

        target[i_target] = item;
        i_target += 1;
        i_source += 1;
    }
}

pub fn u8_slice_into_char_slice(source: &[u8], target: &mut [char]) {
    for i in 0..source.len() {
        target[i] = source[i] as char;
    }
}

#[cfg(test)]
mod tests {
    use crate::slice::{copy_into_slice, copy_into_slice_until, find_first_index_in_slice, split_slice_once};

    #[test]
    fn find_first_index_in_slice_passes() {
        let source_bytes = b"first\r\nsecond\r\nthird";
        let query = b"\r\n";

        assert_eq!(find_first_index_in_slice(source_bytes, query), Some(5));
    }

    #[test]
    fn split_slice_once_passes() {
        let source_bytes = b"first\r\nsecond\r\nthird";
        let delimiter = b"\r\n";

        assert_eq!(
            split_slice_once(source_bytes, delimiter),
            Some((b"first" as &[u8], b"second\r\nthird" as &[u8]))
        );

        let source_bytes: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9][..];
        let delimiter: &[u8] = &[3, 4][..];

        assert_eq!(
            split_slice_once(source_bytes, delimiter),
            Some((&[1, 2][..], &[5, 6, 7, 8, 9][..]))
        );
    }

    #[test]
    fn copy_into_slice_passes() {
        let mut target = [0; 10];
        let source = [1; 5];

        copy_into_slice(&mut target, &source, 1, 4);

        assert_eq!(target[0], 0);
        assert_eq!(target[1], 1);
        assert_eq!(target[2], 1);
        assert_eq!(target[3], 1);
        assert_eq!(target[4], 1);
        assert_eq!(target[5], 0);
        assert_eq!(target[6], 0);
    }

    #[test]
    fn copy_into_slice_until_passes() {
        let mut target = [0; 10];

        let mut source = [1; 10];
        for i in 7..10 {
            source[i] = 0;
        }
        let source = source;

        copy_into_slice_until(&mut target, &source, 2, |item, _, _| item == 0);

        assert_eq!(target[0], 0);
        assert_eq!(target[1], 0);
        assert_eq!(target[2], 1);
        assert_eq!(target[3], 1);
        assert_eq!(target[6], 1);
        assert_eq!(target[7], 1);
        assert_eq!(target[8], 1);
        assert_eq!(target[9], 0);
    }
}
