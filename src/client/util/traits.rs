pub trait WithEnrichment<T, OUTPUT> {
    fn into_domain(self, enrichment_data: T) -> OUTPUT;
}