use std::marker::PhantomData;

fn main() {
    let echo1 = Box::new(Echo { config: PhantomData, data_type: PhantomData });
    let echo2 = Box::new(Echo { config: PhantomData, data_type: PhantomData });
    let inc1 = Box::new(Increment { config: PhantomData, by: 1 });
    let inc2 = Box::new(Increment { config: PhantomData, by: 2 });
    let workflow = compose(echo1, echo2);
    let workflow = compose(workflow, inc1);
    let workflow = compose(workflow, inc2);

    let config = Config {};
    let data = 1;
    println!("{:?}", workflow.run(&config, &data));
}

struct Config {
}

trait Pipeable<C> {
    type Input;
    type Output;

    fn run(&self, config: &C, input: &Self::Input) -> Result<Self::Output, String>;
}

struct Echo<C, T> {
    config: PhantomData<C>,
    data_type: PhantomData<T>,
}

impl<C, T> Pipeable<C> for Echo<C, T>
where T: Clone
{
    type Input = T;
    type Output = T;

    fn run(&self, _config: &C, input: &T) -> Result<T, String> {
        Ok(input.clone())
    }
}

struct Increment<C> {
    config: PhantomData<C>,
    by: u64,
}

impl<C> Pipeable<C> for Increment<C> {
    type Input = u64;
    type Output = u64;

    fn run(&self, _config: &C, input: &u64) -> Result<u64, String> {
        match input.checked_add(self.by) {
            Some(value) => Ok(value),
            None => Err("".to_string()),
        }
    }
}

struct ComposedPipe<C, U, V, W> {
    left: Box<dyn Pipeable<C, Input = U, Output = V>>,
    right: Box<dyn Pipeable<C, Input = V, Output = W>>
}

impl<C, U, V, W> Pipeable<C> for ComposedPipe<C, U, V, W> {
    type Input = U;
    type Output = W;

    fn run(&self, config: &C, input: &U) -> Result<W, String> {
        let input = self.left.run(config, input)?;
        self.right.run(config, &input)
    }
}

fn compose<C: 'static, U: 'static, V: 'static, W: 'static>(
    a: Box<dyn Pipeable<C, Input = U, Output = V> + 'static>,
    b: Box<dyn Pipeable<C, Input = V, Output = W> + 'static>
) -> Box<dyn Pipeable<C, Input = U, Output = W> + 'static> {
    Box::new(ComposedPipe {left: a, right: b})
}
