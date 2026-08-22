pub(crate) mod gen_implementations;

pub(crate) trait TackyGen<T> {
    fn generate_tacky(&self) -> T;
}
