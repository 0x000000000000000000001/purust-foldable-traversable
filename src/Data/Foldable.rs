pub fn Data_Foldable_foldrArray(mut f: purust_core::Func2<crate::UnknownType, crate::UnknownType, crate::UnknownType>, mut init: crate::UnknownType, mut xs: crate::UnknownType) -> crate::UnknownType {
    let arr = xs.unwrap_array();
    let mut acc = init;
    for item in arr.iter().rev() {
        acc = f(item.clone(), acc);
    }
    acc
}

pub fn Data_Foldable_foldlArray(mut f: purust_core::Func2<crate::UnknownType, crate::UnknownType, crate::UnknownType>, mut init: crate::UnknownType, mut xs: crate::UnknownType) -> crate::UnknownType {
    let arr = xs.unwrap_array();
    let mut acc = init;
    for item in arr.iter() {
        acc = f(acc, item.clone());
    }
    acc
}
