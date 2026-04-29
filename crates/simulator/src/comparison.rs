#[derive(Copy, Clone)]
pub enum Operation {
    GreaterOrEqual,
    Greater,
    Equal,
    SmallerOrEqual,
    Smaller,
}

pub fn compare(_result: u32, _op: Operation, _target: u32) -> bool {
    match _op {
        Operation::Greater => _result > _target,
        Operation::GreaterOrEqual => _result >= _target,
        Operation::Equal => _result == _target,
        Operation::Smaller => _result < _target,
        Operation::SmallerOrEqual => _result <= _target,
    }
}
