use sqlx::{postgres::PgRow, Row};
use url::Host;
use crate::{data_store::postgres::PostgresDatabase, nodes::Node};
use super::{NodeStore, error::{GetNodeError, DeleteNodeError, CreateNodeError}};

impl NodeStore for PostgresDatabase {
    async fn create(&self, name: String, host: url::Host, port: u16) -> Result<i32, CreateNodeError> {
        const CREATE_QUERY: &'static str = r#"
            INSERT INTO nodes (name, host, port)
            VALUES ($1, $2, $3)
            RETURNING id
        "#;
        
        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(CREATE_QUERY)
            .bind(name)
            .bind(host.to_string())
            .bind(port.to_string())
            .map(|row: PgRow| row.get("id"))
            .fetch_one(conn.as_mut())
            .await
            .map_err(Into::into)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Node>, GetNodeError> {
        const GET_BY_ID_QUERY: &'static str = r#"
            SELECT id, name, host, port
            FROM nodes
            WHERE id = $1
        "#;
        
        let mut conn = self.connection_pool
            .get()
            .await?;

        let row = sqlx::query(GET_BY_ID_QUERY)
            .bind(id)
            .fetch_optional(conn.as_mut())
            .await?;

        let row = match row {
            Some(row) => row,
            None => return Ok(None),
        };

        let host_str: &str = row.get("host");
        let port_str: &str = row.get("port");

        let node = Node {
            id: row.get("id"),
            name: row.get("name"),
            host: Host::parse(host_str)?,
            port: port_str.parse().unwrap(),
        };

        Ok(Some(node))
    }

    async fn get_by_name<'a>(&self, name: &'a str) -> Result<Option<Node>, GetNodeError> {
        const GET_BY_NAME_QUERY: &'static str = r#"
            SELECT id, name, host, port
            FROM nodes
            WHERE name = $1
        "#;
        
        let mut conn = self.connection_pool
            .get()
            .await?;

        let row = sqlx::query(GET_BY_NAME_QUERY)
            .bind(name)
            .fetch_optional(conn.as_mut())
            .await?;

        let row = match row {
            Some(row) => row,
            None => return Ok(None),
        };

        let host_str: &str = row.get("host");
        let port_str: &str = row.get("port");

        let node = Node {
            id: row.get("id"),
            name: row.get("name"),
            host: Host::parse(host_str)?,
            port: port_str.parse().unwrap(),
        };

        Ok(Some(node))
    }

    async fn get_by_address<'a>(&self, host: &'a Host, port: u16) -> Result<Option<Node>, GetNodeError> {
        const GET_BY_ADDRESS_QUERY: &'static str = r#"
            SELECT id, name, host, port
            FROM nodes
            WHERE address = $1 AND port = $2
        "#;
        
        let mut conn = self.connection_pool
            .get()
            .await?;

        let row = sqlx::query(GET_BY_ADDRESS_QUERY)
            .bind(host.to_string())
            .bind(port.to_string())
            .fetch_optional(conn.as_mut())
            .await?;

        let row = match row {
            Some(row) => row,
            None => return Ok(None),
        };

        let host_str: &str = row.get("host");
        let port_str: &str = row.get("port");

        let node = Node {
            id: row.get("id"),
            name: row.get("name"),
            host: Host::parse(host_str)?,
            port: port_str.parse().unwrap(),
        };

        Ok(Some(node))
    }

    async fn delete(&self, id: i32) -> Result<bool, DeleteNodeError> {
        const DELETE_BY_ID: &'static str = r#"
            WITH deleted 
            AS (
                DELETE 
                FROM users 
                WHERE id = $1
                RETURNING id
            ) 
            SELECT 
            COUNT(id) AS delete_count
            FROM deleted
        "#;
        
        let mut conn = self.connection_pool
            .get()
            .await?;

        let delete_count: i32 = sqlx::query(DELETE_BY_ID)
            .bind(id)
            .map(|row: PgRow| row.get("delete_count"))
            .fetch_one(conn.as_mut())
            .await?;

        Ok(delete_count == 1)
    }
}