pub trait VersionRequirement<V> {
    fn satisfies(&self, version: &V) -> bool;
}
