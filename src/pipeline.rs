

pub trait Fragment<T> {
    type Color;
    fn fragment(&self, pos: T) -> Self::Color;
}

pub trait Vertex<T> {
    type Out;
    fn vertex(&self, v: T) -> Self::Out;
}