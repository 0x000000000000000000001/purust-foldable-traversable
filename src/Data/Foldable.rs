pub fn Data_Foldable_foldrArray(mut f: crate::UnknownType, mut init: crate::UnknownType, mut xs: crate::UnknownType) -> crate::UnknownType {
    let arr = xs.init_array.as_ref().unwrap();
    let mut acc = init;
    for item in arr.iter().rev() {
        acc = f.call.as_ref().unwrap()(item.clone()).call.as_ref().unwrap()(acc);
    }
    acc
}

pub fn Data_Foldable_foldlArray(mut f: crate::UnknownType, mut init: crate::UnknownType, mut xs: crate::UnknownType) -> crate::UnknownType {
    let arr = xs.init_array.as_ref().unwrap();
    let mut acc = init;
    for item in arr.iter() {
        acc = f.call.as_ref().unwrap()(acc).call.as_ref().unwrap()(item.clone());
    }
    acc
}
