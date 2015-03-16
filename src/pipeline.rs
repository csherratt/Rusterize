

pub trait Fragment<T> {
    type Color;
    fn fragment(&self, pos: T) -> Self::Color;

    fn blend(&self, _: Self::Color, new: Self::Color) -> Self::Color { new }
}

pub trait Vertex<T> {
    type Out;
    fn vertex(&self, v: T) -> Self::Out;
}

pub trait Mapping<T> {
    type Out;
    fn mapping(&self, pixel: T) -> Self::Out;
}

