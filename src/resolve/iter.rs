use kdl::KdlNode;

type BoxIter<'a> = Box<dyn Iterator<Item = &'a KdlNode> + 'a>;

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
}

impl<'a, I> Iterator for AnywhereIter<'a, I>
where
    I: Iterator<Item = &'a KdlNode>,
{
    type Item = &'a KdlNode;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(current) = self.current.as_mut() else {
            let (knode, it) = loop {
                let Some(knode) = self.it.next() else {
                    return None;
                };
                let it = knode
                    .children()
                    .map(|kdoc| kdoc.nodes().iter())
                    .map(|it| AnywhereIter::new(it));
                // let Some(kdoc) = knode.children() else {
                //     continue;
                // };
                break (knode, it);
            };
            self.current = match it {
                Some(it) => Some(Box::new(it)),
                None => None,
            };
            //it.map(|it| Box::new(it));
            return Some(knode);
        };
        let Some(next_node) = current.next() else {
            self.current = None;
            return self.next();
        };
        Some(next_node)
    }
}

pub trait AnywhereIterator<'a>
where
    Self: Iterator<Item = &'a KdlNode> + Sized,
{
    fn anywhere_nodes(self) -> impl Iterator<Item = &'a KdlNode> {
        AnywhereIter::new(self)
    }
}
impl<'a, I> AnywhereIterator<'a> for I where I: Iterator<Item = &'a KdlNode> {}
