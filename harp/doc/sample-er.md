```mermaid
erDiagram

users ||--o{ articles: ""

users {
  integer id PK
  varchar_32 name "not_null"
  varchar_32 email "not_null"
  integer age "default_null"
  datetime create_at "default_current_timestamp"
  datetime update_at "not_null default_current_timestamp"
}

articles {
  varchar_64 title "not_null"
  text text
}
```
