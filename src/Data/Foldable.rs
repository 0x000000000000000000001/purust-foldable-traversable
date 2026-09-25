#[inline]
pub fn Data_Foldable_foldrArray(
    f: purust_core::Func2<UnknownType, UnknownType, UnknownType>,
    init: UnknownType,
    xs: UnknownType,
) -> UnknownType {
    let arr = xs.unwrap_array();
    let mut acc = init;
    for item in arr.iter().rev() {
        acc = f(item.clone(), acc);
    }
    acc
}

#[inline]
pub fn Data_Foldable_foldlArray(
    f: purust_core::Func2<UnknownType, UnknownType, UnknownType>,
    init: UnknownType,
    xs: UnknownType,
) -> UnknownType {
    let arr = xs.unwrap_array();
    let mut acc = init;
    for item in arr.iter() {
        acc = f(acc, item.clone());
    }
    acc
}
