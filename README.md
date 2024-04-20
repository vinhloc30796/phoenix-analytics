# phoenix-analytics

## Commands

### Load data

```bash
$ RUST_LOG=info cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/indexer`
[2024-04-20T08:04:20Z INFO  indexer] Market: "SOL"-"USDC" with address "CS2H8nbAVVEUHWPF5extCSymqheQdkd4d7thik6eet9N"
[2024-04-20T08:04:22Z INFO  indexer::rpc] 10 signatures found
[2024-04-20T08:04:28Z INFO  indexer] Found 1 instructions
[2024-04-20T08:04:28Z INFO  indexer] Found 1 instructions
[2024-04-20T08:04:28Z INFO  indexer] Found 3 instructions
[2024-04-20T08:04:28Z INFO  indexer] Found 0 instructions
[2024-04-20T08:04:28Z INFO  indexer] Found 1 instructions
[2024-04-20T08:04:28Z INFO  indexer] Found 1 instructions
[2024-04-20T08:04:28Z INFO  indexer] Found 1 instructions
[2024-04-20T08:04:28Z INFO  indexer] Found 1 instructions
[2024-04-20T08:04:28Z INFO  indexer] Found 1 instructions
[2024-04-20T08:04:28Z INFO  indexer] Found 3 instructions
[2024-04-20T08:04:28Z INFO  indexer] Finished processing instructions for 10 transactions
[2024-04-20T08:04:28Z INFO  indexer::txn] Published transaction: 2WDgiH7AhWrssZ9omKAzFtgEdwPrhHcZrFA1pGNQW78yDZa2PmZ8K2Nc4PUZTD32SXpz3akKLr7LPVP6wNgx9aTh
[2024-04-20T08:04:28Z INFO  indexer::txn] Published transaction: c4n9ttuQ4WzenUAqiTXZc3zjgQYHBRYKvW9f5aPYpoimjwPfLPKTKFXnm5ctJfNhaszyV6BVbkkTg6eLmmaPeQc
[2024-04-20T08:04:28Z INFO  indexer::txn] Published transaction: 2RXHLoaAjjP1gDTYFmG8D1UTHN6cxXR3Ho6Up5Toh1DPE1kEEJDtuCMVsbW1p87C2SHfTMwRBNEruYsmXbxJMFm4
[2024-04-20T08:04:28Z INFO  indexer::txn] Published transaction: 36KyGGD9XpERbANCFEa3xuBYiaXJxvD1HTXHtMHH9C5eDsLeTUqkVYpiZS7AnmJtvTLed4ZWgSnEHqjnuL6LEYGv
[2024-04-20T08:04:28Z INFO  indexer::txn] Published transaction: Msdc19y9sVerKa342eefpduEn34dzcAJgqrc4CPggd8F4HyWRsjq8cmoZMrUYSae7U97EFs7pap3eqUP5dGGfyq
[2024-04-20T08:04:28Z INFO  indexer::txn] Published transaction: 2VjF9yu34pzgUS6o9PHUzpyT8EAYsty1ihW9Kv6paemX5FKXjc8RWJi5nC4bYkkjVHDx2bcyRpTpAGbXP7P4pmmH
[2024-04-20T08:04:28Z INFO  indexer::txn] Published transaction: 63qVE25P4PC2mP8HKo4YX6QdP5TiEWZBrAc8GuJvGmLGQ8udjmkKDngQcPi6nUL46SbPEeZAjVfF4d7mio9XKwpM
[2024-04-20T08:04:28Z INFO  indexer::txn] Published transaction: 2Rq7WZ9dGjVDR79AMPmxCkq1M38JXVGy1fq4ELBALjF1XgcKua5upA3e5Y9qyiqvTAABECwJoCG3HYsaa9WZhG1F
[2024-04-20T08:04:28Z INFO  indexer::txn] Published transaction: 59G7yYdtNgB7P78aBTgVr8zrxEMoxWxRo6YCcaXEXeKPwXQeSzkaK5Q3p86mvMNP1FJ4eegNyoGyUBaAtC235sV6
[2024-04-20T08:04:28Z INFO  indexer::txn] Published transaction: 5C117f2E6KTkGUMq8TFzH9q441b7Ue4CsyD9e95zY8jUQn2XC8Am2e6qdAmKVGQLuTbXDHH3UvY4pymhfV2jEWHG
```

Alternatively:

```bash
RUST_LOG=debug cargo run
```

### Query data

```bash
docker exec -it trino trino
```

```sql
select * from iceberg.phoenix.transaction;
select * from iceberg.phoenix.event;
```

## References

### Business Logic a la Ellipsis Labs & Phoenix

- [Phoenix DEX - Market Addresses](https://ellipsis-labs.gitbook.io/phoenix-dex/tRIkEFlLUzWK9uKO3W2V/getting-started/technical-overview/market-addresses)
- [Phoenix DEX - Events](https://ellipsis-labs.gitbook.io/phoenix-dex/tRIkEFlLUzWK9uKO3W2V/getting-started/technical-overview/events)

### Data Platform

- [Apache Iceberg + Trino + iceberg-kafka-connector](https://hendoxc.substack.com/p/apache-iceberg-trino-iceberg-kafka) by [Hagen](https://substack.com/profile/173721939-hagen)
- [Wuerike/kafka-iceberg-streaming](https://github.com/Wuerike/kafka-iceberg-streaming)
- [Apache Iceberg's REST Catalog](https://www.apachecon.com/acna2022/slides/02_Redai_Apache_Icebergs_REST.pdf) by [samredai](https://www.github.com/samredai/)
- [How to query data and write data in Apache Iceberg using StarRocks #23427](https://github.com/StarRocks/starrocks/discussions/23427)
- [Create an Iceberg Sink Connector - Troubleshoot](https://docs.redpanda.com/current/deploy/deployment-option/cloud/managed-connectors/create-iceberg-sink-connector/#troubleshoot) by [Redpanda Docs](https://docs.redpanda.com/)
