pub struct NEArray(Vec<UnknownType>);

pub fn Test_Main_arrayFrom1UpTo(count: i64) -> UnknownType {
    mk_array((1..=count).map(mk_int).collect())
}

pub fn Test_Main_arrayReplicate(count: i64, value: UnknownType) -> UnknownType {
    mk_array(vec![value; count.max(0) as usize])
}

pub fn Test_Main_mkNEArray(
    nothing: UnknownType,
    just: purust_core::Func1<std::rc::Rc<NEArray>, UnknownType>,
    array: UnknownType,
) -> UnknownType {
    let values = array.unwrap_array();
    if values.is_empty() {
        nothing
    } else {
        just(std::rc::Rc::new(NEArray(values.to_vec())))
    }
}

pub fn Test_Main_foldMap1NEArray(
    append: purust_core::Func2<UnknownType, UnknownType, UnknownType>,
    function: purust_core::Func1<UnknownType, UnknownType>,
    array: std::rc::Rc<NEArray>,
) -> UnknownType {
    let mut result = function(array.0[0].clone());
    for value in &array.0[1..] {
        result = append(result, function(value.clone()));
    }
    result
}
