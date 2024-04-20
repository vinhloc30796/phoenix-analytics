# phoenix-analytics

## Commands

### Load data

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
