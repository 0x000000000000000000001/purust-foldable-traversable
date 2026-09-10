pub fn Data_FunctorWithIndex_mapWithIndexArray(
    function: purust_core::Func2<i64, UnknownType, UnknownType>,
    array: UnknownType,
) -> UnknownType {
    mk_array(
        array
            .unwrap_array()
            .iter()
            .enumerate()
            .map(|(index, value)| function(index as i64, value.clone()))
            .collect(),
    )
}
