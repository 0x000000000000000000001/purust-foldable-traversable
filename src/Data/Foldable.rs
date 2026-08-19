pub fn Data_Foldable_foldrArray(mut f: crate::UnknownType, mut init: crate::UnknownType, mut xs: crate::UnknownType) -> crate::UnknownType {
    let arr = xs.unwrap_array();
    let mut acc = init;
    for item in arr.iter().rev() {
        acc = f.unwrap_func()(item.clone()).unwrap_func()(acc);
    }
    acc
}

pub fn Data_Foldable_foldlArray(mut f: crate::UnknownType, mut init: crate::UnknownType, mut xs: crate::UnknownType) -> crate::UnknownType {
    let arr = xs.unwrap_array();
    let mut acc = init;
    for item in arr.iter() {
        acc = f.unwrap_func()(acc).unwrap_func()(item.clone());
    }
    acc
}
