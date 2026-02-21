pub enum Operation {
    GreaterOrEqual,
    Greater,
    Equal,
    SmallerOrEqual,
    Smaller,
}

pub fn compare(_result: i32, _op: Operation, _target: i32) -> bool {
    match _op {
        Operation::Greater => _result > _target,
        Operation::GreaterOrEqual => _result >= _target,
        Operation::Equal => _result == _target,
        Operation::Smaller => _result < _target,
        Operation::SmallerOrEqual => _result <= _target,
    }
}
