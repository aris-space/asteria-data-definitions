use core::mem::MaybeUninit;

/// Concatenates slices at compile time, with final length `N`. Gives an owned array back.
/// Panics if `N` does not match the sum of the lengths of `sets`,
/// but you can easily compute this in a const fashion as well.
pub const fn concat_slices<T: Copy, const N: usize>(sets: &[&[T]]) -> [T; N] {
    let mut x = [core::mem::MaybeUninit::uninit(); N];
    let mut rest: &mut [MaybeUninit<T>] = &mut x;

    let mut i = 0;
    while i < sets.len() {
        let set = sets[i];
        let len = set.len();

        // iterators aren't const
        let (this, new_rest) = rest.split_at_mut(len);
        rest = new_rest;

        // SAFETY
        // - `MaybeUninit` takes up same amount of memory as `T`
        // - Length of slices is the same
        // - Thus we can just treat the same slice pointer as
        //   an `MaybeUninit<T>` slice instead of a `T` slice
        this.copy_from_slice(unsafe {
            core::slice::from_raw_parts(set.as_ptr().cast::<MaybeUninit<T>>(), len)
        });
        i += 1;
    }

    assert!(rest.is_empty());

    // SAFETY
    // - We checked that the we wrote exactly `N` elements
    // - `MaybeUninit<T>` and `T` have the same memory layouts and thus we can just
    //   treat it as an `[T; N]`
    let res: [T; N] = unsafe {
        let ptr = &raw const x;
        let ptr = ptr.cast::<[T; N]>();
        ptr.read()
    };
    res
}
