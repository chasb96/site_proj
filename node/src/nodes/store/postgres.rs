use diesel::{deserialize::Queryable, Selectable, prelude::Insertable, SelectableHelper, RunQueryDsl, ExpressionMethods, QueryDsl, result::Error::NotFound};
use url::Host;
use crate::{data_store::postgres::{PostgresDatabase, nodes}, nodes::Node};
use super::{NodeStore, error::{GetNodeError, DeleteNodeError}};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::data_store::postgres::nodes)]
struct NodeModel {
    pub id: i32,
    pub name: String,
    pub host: String,
    pub port: i32
}

#[derive(Insertable)]
#[diesel(table_name = crate::data_store::postgres::nodes)]
struct NewNodeModel {
    pub name: String,
    pub host: String,
    pub port: i32,
}

impl NodeStore for PostgresDatabase {
    async fn create(&self, name: String, host: url::Host, port: u16) -> Result<crate::nodes::Node, super::error::CreateNodeError> {
        let node_entity = {
            let mut conn = self.connection_pool
                .get()?;

            diesel::insert_into(nodes::table)
                .values(
                    NewNodeModel {
                        name: name,
                        host: host.to_string(),
                        port: port as i32,
                    }
                )
                .returning(NodeModel::as_returning())
                .get_result(&mut conn)?
        };

        let node = Node {
            id: node_entity.id,
            name: node_entity.name,
            host: Host::parse(&node_entity.host)?,
            port: node_entity.port as u16,
        };

        Ok(node)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Node>, GetNodeError> {
        let query_result = {
            let mut conn = self.connection_pool
                .get()?;

            nodes::table
                .filter(nodes::id.eq(id))
                .select(NodeModel::as_select())
                .get_result(&mut conn)
        };

        let node_entity = match query_result {
            Ok(node) => node,
            Err(NotFound) => return Ok(None),
            Err(e) => return Err(e.into()),
        };

        let node = Node {
            id: node_entity.id,
            name: node_entity.name,
            host: Host::parse(&node_entity.host)?,
            port: node_entity.port as u16,
        };

        Ok(Some(node))
    }

    async fn get_by_name<'a>(&self, name: &'a str) -> Result<Option<Node>, GetNodeError> {
        let query_result = {
            let mut conn = self.connection_pool
                .get()?;

            nodes::table
                .filter(nodes::name.eq(name))
                .select(NodeModel::as_select())
                .get_result(&mut conn)
        };

        let node_entity = match query_result {
            Ok(node) => node,
            Err(NotFound) => return Ok(None),
            Err(e) => return Err(e.into()),
        };

        let node = Node {
            id: node_entity.id,
            name: node_entity.name,
            host: Host::parse(&node_entity.host)?,
            port: node_entity.port as u16,
        };

        Ok(Some(node))
    }

    async fn get_by_address<'a>(&self, host: &'a Host, port: u16) -> Result<Option<Node>, GetNodeError> {
        let query_result = {
            let mut conn = self.connection_pool
                .get()?;

            nodes::table
                .filter(nodes::host.eq(host.to_string()))
                .filter(nodes::port.eq(port as i32))
                .select(NodeModel::as_select())
                .get_result(&mut conn)
        };

        let node_entity = match query_result {
            Ok(node) => node,
            Err(NotFound) => return Ok(None),
            Err(e) => return Err(e.into()),
        };

        let node = Node {
            id: node_entity.id,
            name: node_entity.name,
            host: Host::parse(&node_entity.host)?,
            port: node_entity.port as u16,
        };

        Ok(Some(node))
    }

    async fn delete(&self, id: i32) -> Result<bool, DeleteNodeError> {
        let delete_count = {
            let mut conn = self.connection_pool
                .get()?;

            diesel::delete(nodes::table)
                .filter(nodes::id.eq(id))
                .execute(&mut conn)?
        };

        Ok(delete_count == 1)
    }
}