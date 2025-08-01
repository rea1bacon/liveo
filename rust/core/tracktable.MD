# SQL
## Workspace

columns | type | description |
--- | --- | --- |
| id | uuid v4 | Unique identifier for the track table |
| name | text | The name of the track table (unique) |
| on | enum | The event that triggers the track table (e.g., `INSERT`, `UPDATE`, `DELETE`) |
| p_id | (name, type) (optional) | The primary key of the row being tracked |
| table | text | The name of the table being tracked |
| track | text[] | The columns being tracked |
| old | bool | Whether to track old values |
| new | bool | Whether to track new values |
| created_at | timestamp | The timestamp when the track table was created |
| updated_at | timestamp | The timestamp when the track table was last updated |

## TrackTable_uuid

columns | type | description |
--- | --- | --- |
| id | uuid v4 | Unique identifier for the track |
| p_id | dynamic | the primary key of the row being tracked |
| old_#1 |  text | The old value of the first tracked column |
| old_#2 |  text | The old value of the second tracked column |
| ... | text | Additional old values for other tracked columns |
| new_#1 |  text | The new value of the first tracked column |
| new_#2 |  text | The new value of the second tracked column |

# Struct

```rust
type Col = (String, String);

struct TrackTable {
    id: Uuid,
    name: String,
    on: On,
    p_id: Option<Col>,
    table: String,
    track: Vec<Col>,
    old: bool,
    new: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

struct Workspace {
    track_tables: Vec<TrackTable>,
    operations: Vec<Operation>,
}
```