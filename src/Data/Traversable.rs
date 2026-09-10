use std::rc::Rc;

type TraversalApply = purust_core::Func2<UnknownType, UnknownType, UnknownType>;
type TraversalMap =
    purust_core::Func2<purust_core::Func1<UnknownType, UnknownType>, UnknownType, UnknownType>;
type TraversalFunction = purust_core::Func1<UnknownType, UnknownType>;

fn traversal_array_one(first: UnknownType) -> UnknownType {
    mk_array(vec![first])
}

fn traversal_array_two(first: UnknownType) -> UnknownType {
    Value::Func1(purust_core::Func1::Shared(Rc::new(move |second| {
        mk_array(vec![first.clone(), second])
    })))
}

fn traversal_array_three(first: UnknownType) -> UnknownType {
    Value::Func1(purust_core::Func1::Shared(Rc::new(move |second| {
        let first = first.clone();
        Value::Func1(purust_core::Func1::Shared(Rc::new(move |third| {
            mk_array(vec![first.clone(), second.clone(), third])
        })))
    })))
}

fn traversal_concat(left: UnknownType) -> UnknownType {
    let left = left.unwrap_array();
    Value::Func1(purust_core::Func1::Shared(Rc::new(
        move |right: UnknownType| {
            let right = right.unwrap_array();
            let mut result = Vec::with_capacity(left.len() + right.len());
            result.extend(left.iter().cloned());
            result.extend(right.iter().cloned());
            mk_array(result)
        },
    )))
}

fn traverse_chunk(
    apply: &TraversalApply,
    map: &TraversalMap,
    pure: &TraversalFunction,
    function: &TraversalFunction,
    values: &[UnknownType],
) -> UnknownType {
    match values.len() {
        0 => pure(mk_array(Vec::new())),
        1 => map(
            purust_core::Func1::Static(traversal_array_one),
            function(values[0].clone()),
        ),
        2 | 3 => {
            let make = if values.len() == 2 {
                traversal_array_two
            } else {
                traversal_array_three
            };
            let first = map(
                purust_core::Func1::Static(make),
                function(values[0].clone()),
            );
            let second = apply(first, function(values[1].clone()));
            if values.len() == 2 {
                second
            } else {
                apply(second, function(values[2].clone()))
            }
        }
        length => {
            // The same balanced partitions as upstream keep both this traversal
            // and the resulting applicative expression logarithmic in depth.
            let pivot = length / 4 * 2;
            let left = traverse_chunk(apply, map, pure, function, &values[..pivot]);
            let joined = map(purust_core::Func1::Static(traversal_concat), left);
            let right = traverse_chunk(apply, map, pure, function, &values[pivot..]);
            apply(joined, right)
        }
    }
}

pub fn Data_Traversable_traverseArrayImpl(
    apply: TraversalApply,
    map: TraversalMap,
    pure: TraversalFunction,
    function: TraversalFunction,
    array: UnknownType,
) -> UnknownType {
    traverse_chunk(&apply, &map, &pure, &function, &array.unwrap_array())
}
