pub struct TableMetadata {

	// The user-friendly name for the table.
	pub name: String,
	// Output-only location of the table, based on the encapsulating dataset.
	pub location: String,
	// The user-friendly description of the table.
	pub description: String,
	// The table schema. If provided on create, ViewQuery must be empty.
	pub schema: Schema,
	// If non-nil, this table is a materialized view.
	pub materialized_view: MaterializedViewDefinition,
	// The query to use for a logical view. If provided on create, Schema must be nil.
	pub view_query: String,
	// Use Legacy SQL for the view query.
	// At most one of UseLegacySQL and UseStandardSQL can be true.
	pub use_legacy_sql: bool,
	// Use Standard SQL for the view query. The default.
	// At most one of UseLegacySQL and UseStandardSQL can be true.
	// Deprecated: use UseLegacySQL.
	pub use_standard_sql: bool,
	// If non-nil, the table is partitioned by time. Only one of
	// time partitioning or range partitioning can be specified.
	pub time_partitioning: TimePartitioning,
	// If non-nil, the table is partitioned by integer range.  Only one of
	// time partitioning or range partitioning can be specified.
	pub range_partitioning: RangePartitioning,
	// If set to true, queries that reference this table must specify a
	// partition filter (e.g. a WHERE clause) that can be used to eliminate
	// partitions. Used to prevent unintentional full data scans on large
	// partitioned tables.
	pub require_partition_filter: bool,

	// Clustering specifies the data clustering configuration for the table.
	pub clustering: Clustering,

	// The time when this table expires. If set, this table will expire at the
	// specified time. Expired tables will be deleted and their storage
	// reclaimed. The zero value is ignored.
	pub expiration_time chrono::

	// User-provided labels.
	pub Labels map[string]string,

	// Information about a table stored outside of BigQuery.
	pub ExternalDataConfig *ExternalDataConfig,

	// Custom encryption configuration (e.g., Cloud KMS keys).
	pub EncryptionConfig *EncryptionConfig,

	pub FullID           string, // An opaque ID uniquely identifying the table.
	pub Type             TableType,
	pub CreationTime     time.Time,
	pub LastModifiedTime time.Time,

	// The size of the table in bytes.
	// This does not include data that is being buffered during a streaming insert.
	pub NumBytes int64,

	// The number of bytes in the table considered "long-term storage" for reduced
	// billing purposes.  See https://cloud.google.com/bigquery/pricing#long-term-storage
	// for more information.
	pub NumLongTermBytes int64,

	// The number of rows of data in this table.
	// This does not include data that is being buffered during a streaming insert.
	pub NumRows uint64,

	// Contains information regarding this table's streaming buffer, if one is
	// present. This field will be nil if the table is not being streamed to or if
	// there is no data in the streaming buffer.
	pub StreamingBuffer *StreamingBuffer,

	// ETag is the ETag obtained when reading metadata. Pass it to Table.Update to
	// ensure that the metadata hasn't changed since it was read.
	pub ETag string,
}
