use purust_core::*;
use std::cell::RefCell;
use std::rc::Rc;
use Purs_Data_Traversable as traversable;

fn identity(value: Value) -> Value {
    value
}
fn pure_apply(function: Value, value: Value) -> Value {
    function.unwrap_func1()(value)
}
fn pure_map(function: Func1<Value, Value>, value: Value) -> Value {
    function(value)
}
fn effect(action: impl Fn() -> Value + 'static) -> Value {
    Value::Func1(Func1::Shared(Rc::new(move |_| action())))
}
fn effect_pure(value: Value) -> Value {
    effect(move || value.clone())
}
fn effect_map(function: Func1<Value, Value>, action: Value) -> Value {
    effect(move || function(action.unwrap_func1()(Value::Unit)))
}
fn effect_apply(function: Value, argument: Value) -> Value {
    effect(move || {
        let function = function.unwrap_func1()(Value::Unit);
        let argument = argument.unwrap_func1()(Value::Unit);
        function.unwrap_func1()(argument)
    })
}
fn choice_pure(value: Value) -> Value {
    mk_array(vec![value])
}
fn choice_map(function: Func1<Value, Value>, values: Value) -> Value {
    mk_array(
        values
            .unwrap_array()
            .iter()
            .cloned()
            .map(|x| function(x))
            .collect(),
    )
}
fn choice_apply(functions: Value, values: Value) -> Value {
    let values = values.unwrap_array();
    let mut result = Vec::new();
    for function in functions.unwrap_array().iter() {
        for value in values.iter() {
            result.push(function.unwrap_func1()(value.clone()));
        }
    }
    mk_array(result)
}
fn integers(value: Value) -> Vec<i64> {
    value.unwrap_array().iter().map(Value::unwrap_int).collect()
}
#[test]
fn generic_traversal_order_replay_and_100000_elements() {
    for length in [0, 1, 2, 3, 4, 5, 10, 100000] {
        let order = Rc::new(RefCell::new(Vec::new()));
        let seen = order.clone();
        let input = mk_array((0..length).map(mk_int).collect());
        let result = traversable::Data_Traversable_traverseArrayImpl(
            Func2::Static(pure_apply),
            Func2::Static(pure_map),
            Func1::Static(identity),
            Func1::Shared(Rc::new(move |value: Value| {
                let value = value.unwrap_int();
                seen.borrow_mut().push(value);
                mk_int(value * 2)
            })),
            input.clone(),
        );
        assert_eq!(
            integers(result),
            (0..length).map(|n| n * 2).collect::<Vec<_>>()
        );
        assert_eq!(*order.borrow(), (0..length).collect::<Vec<_>>());
        order.borrow_mut().clear();
        let seen = order.clone();
        let action = traversable::Data_Traversable_traverseArrayImpl(
            Func2::Static(effect_apply),
            Func2::Static(effect_map),
            Func1::Static(effect_pure),
            Func1::Shared(Rc::new(move |value: Value| {
                let seen = seen.clone();
                let value = value.unwrap_int();
                effect(move || {
                    seen.borrow_mut().push(value);
                    mk_int(value * 2)
                })
            })),
            input,
        );
        assert!(
            order.borrow().is_empty(),
            "effects ran while constructing traversal"
        );
        for _ in 0..2 {
            let result = action.unwrap_func1()(Value::Unit);
            assert_eq!(
                integers(result),
                (0..length).map(|n| n * 2).collect::<Vec<_>>()
            );
            assert_eq!(*order.borrow(), (0..length).collect::<Vec<_>>());
            order.borrow_mut().clear();
        }
    }
    let choices = traversable::Data_Traversable_traverseArrayImpl(
        Func2::Static(choice_apply),
        Func2::Static(choice_map),
        Func1::Static(choice_pure),
        Func1::Static(|value| {
            let n = value.unwrap_int();
            mk_array(vec![mk_int(n), mk_int(n + 10)])
        }),
        mk_array(vec![mk_int(1), mk_int(2), mk_int(3)]),
    );
    let actual: Vec<_> = choices
        .unwrap_array()
        .iter()
        .cloned()
        .map(integers)
        .collect();
    assert_eq!(
        actual,
        vec![
            vec![1, 2, 3],
            vec![1, 2, 13],
            vec![1, 12, 3],
            vec![1, 12, 13],
            vec![11, 2, 3],
            vec![11, 2, 13],
            vec![11, 12, 3],
            vec![11, 12, 13]
        ]
    );
}

#[test]
fn fold_callbacks_preserve_order_arguments_and_empty_values() {
    use Purs_Data_Foldable::{Data_Foldable_foldlArray, Data_Foldable_foldrArray};
    let input = mk_array(vec![mk_int(1), mk_int(2), mk_int(3)]);
    let seen = Rc::new(RefCell::new(Vec::new()));
    let trace = seen.clone();
    let left = Func2::Shared(Rc::new(move |acc: Value, n: Value| {
        trace.borrow_mut().push(n.unwrap_int());
        mk_int(acc.unwrap_int() - n.unwrap_int())
    }));
    assert_eq!(
        Data_Foldable_foldlArray(left.clone(), mk_int(10), input.clone()).unwrap_int(),
        4
    );
    assert_eq!(*seen.borrow(), vec![1, 2, 3]);
    seen.borrow_mut().clear();
    assert_eq!(
        Data_Foldable_foldlArray(left, mk_int(7), mk_array(vec![])).unwrap_int(),
        7
    );
    assert!(seen.borrow().is_empty());

    let trace = seen.clone();
    let right = Func2::Shared(Rc::new(move |n: Value, acc: Value| {
        trace.borrow_mut().push(n.unwrap_int());
        mk_int(n.unwrap_int() - acc.unwrap_int())
    }));
    assert_eq!(
        Data_Foldable_foldrArray(right.clone(), mk_int(10), input).unwrap_int(),
        -8
    );
    assert_eq!(*seen.borrow(), vec![3, 2, 1]);
    seen.borrow_mut().clear();
    assert_eq!(
        Data_Foldable_foldrArray(right, mk_int(7), mk_array(vec![])).unwrap_int(),
        7
    );
    assert!(seen.borrow().is_empty());
}

#[test]
fn indexed_mapping_calls_once_in_order_and_preserves_input() {
    use Purs_Data_FunctorWithIndex::Data_FunctorWithIndex_mapWithIndexArray;
    let seen = Rc::new(RefCell::new(Vec::new()));
    let trace = seen.clone();
    let function = Func2::Shared(Rc::new(move |i: i64, value: Value| {
        trace.borrow_mut().push((i, value.unwrap_int()));
        mk_int(i * 10 + value.unwrap_int())
    }));
    let input = mk_array(vec![mk_int(3), mk_int(4), mk_int(5)]);
    for _ in 0..2 {
        seen.borrow_mut().clear();
        assert_eq!(
            integers(Data_FunctorWithIndex_mapWithIndexArray(
                function.clone(),
                input.clone()
            )),
            vec![3, 14, 25]
        );
        assert_eq!(*seen.borrow(), vec![(0, 3), (1, 4), (2, 5)]);
        assert_eq!(integers(input.clone()), vec![3, 4, 5]);
    }
    seen.borrow_mut().clear();
    assert!(integers(Data_FunctorWithIndex_mapWithIndexArray(
        function,
        mk_array(vec![])
    ))
    .is_empty());
    assert!(seen.borrow().is_empty());
}
