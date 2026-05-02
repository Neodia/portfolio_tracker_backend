TODO: Make a proper file out of this

## Considerations

- Scheduled jobs: Went with in-code scheduled jobs because this will be a single instance for my needs. If multiple
  instances for a prod context, could switch to cron at os level (or k8s) into http call
- Outbox table: Because the needs are small, I went with the outbox pattern to once-delivery for rates fetched ->
  portfolio snapshots taken. This could be switched to Debezium -> Kafka -> Consumer but I deemed it overkill for this
  project
- Repositories: Some calls are standalone and some require the transaction to be committed elsewhere. This is properly
  defined by the signature needing `tx: &mut PgTransaction`
- "Enums" present in the DB are handled directly in code, so there aren't enum types defined in the DB directly. This
  makes manual updating of rows more fragile, but
    1. this shouldn't happen
    2. write access would be given only to SREs in specific cases which would prevent accidental updates with wrong
       values
- `PgPool::clone` uses `Arc` internally so it's safe to clone, same for the `CGClient` with `reqwest::Client`
- Rate limiting isn't in this app as I would put in at deployment level behind a WAF (With a K8S service for example)
- As a purely personal project, there's no SSL as that's an infra issue I don't have as I use my app through SSH. In
  prod, I would've a Let's Encrypt + Certbot setup
- Testing Strategy: Where relevant I test the whole behavior directly with IT tests (api/service) with Repositories
  tests to ensure the queries are doing what they are supposed to. Unit tests are only used for specific parts of the
  code for specific behaviors, like mappers.