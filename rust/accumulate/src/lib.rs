/// What should the type of _function be?
pub fn map<T, U, F>(input: Vec<T>, mut function: F) -> Vec<U>
where
    F: FnMut(T) -> U,
{
    let mut rel = Vec::<U>::new();

    for val in input {
        rel.push(function(val));
    }

    rel
}
