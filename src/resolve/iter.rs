use kdl::KdlNode;
use std::slice::Iter as SliceIter;

type BoxIter<'a> = Box<AnywhereIter<'a, SliceIter<'a, KdlNode>>>;

pub struct AnywhereIter<'a, I>
where
    I: Iterator<Item = &'a KdlNode>,
{
    it: I,
    current: Option<BoxIter<'a>>,
}

impl<'a, I> AnywhereIter<'a, I>
where
    I: Iterator<Item = &'a KdlNode>,
{
    pub fn new(it: I) -> Self {
        Self { it, current: None }
    }

    fn next_from_origin(&mut self) -> Option<<Self as Iterator>::Item> {
        let (knode, new_iter) = {
            let knode = self.it.next()?;
            let it = knode
                .children()
                .map(|kdoc| kdoc.nodes().iter())
                .map(|it| Box::new(AnywhereIter::new(it)));
            (knode, it)
        };
        self.current = new_iter;
        Some(knode)
    }
}

impl<'a, I> Iterator for AnywhereIter<'a, I>
where
    I: Iterator<Item = &'a KdlNode>,
{
    type Item = &'a KdlNode;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current.as_mut().and_then(|current| current.next()) {
            None => self.next_from_origin(),
            some_node => some_node,
        }
    }
}

pub trait AnywhereIterator<'a>
where
    Self: Iterator<Item = &'a KdlNode> + Sized,
{
    #[inline]
    fn anywhere_nodes(self) -> impl Iterator<Item = &'a KdlNode> {
        AnywhereIter::new(self)
    }
}
impl<'a, I> AnywhereIterator<'a> for I where I: Iterator<Item = &'a KdlNode> {}
