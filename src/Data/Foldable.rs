pub fn Data_Foldable_foldrArray(f: std::rc::Rc<dyn Fn(crate::UnknownType) -> std::rc::Rc<dyn Fn(crate::UnknownType) -> crate::UnknownType>>, mut init: crate::UnknownType, mut xs: crate::UnknownType) -> crate::UnknownType {
    let arr = xs.unwrap_array();
    let mut acc = init;
    for item in arr.iter().rev() {
        acc = (f(item.clone()))(acc);
    }
    acc
}

pub fn Data_Foldable_foldlArray(f: std::rc::Rc<dyn Fn(crate::UnknownType) -> std::rc::Rc<dyn Fn(crate::UnknownType) -> crate::UnknownType>>, mut init: crate::UnknownType, mut xs: crate::UnknownType) -> crate::UnknownType {
    let arr = xs.unwrap_array();
    let mut acc = init;
    for item in arr.iter() {
        acc = (f(acc))(item.clone());
    }
    acc
}
