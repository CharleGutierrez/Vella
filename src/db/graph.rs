use tracing::info;

pub struct GraphTraversalBuilder;

impl GraphTraversalBuilder {
    pub fn new() -> Self {
        info!("Initializing Graph Database Traversal Engine (Neo4j/Cypher syntax)");
        Self
    }

    /// Translates complex relationships into a Cypher query (e.g. Users who like Sci-Fi and watched movies by Ridley Scott)
    pub fn build_recommendation_traversal(&self, user_id: &str, max_degrees: u8) -> String {
        info!("Building Graph Traversal for User {} with {} degrees of separation", user_id, max_degrees);
        
        let cypher_query = format!(
            "MATCH (u:User {{id: '{}'}})-[:LIKES]->(g:Genre {{name: 'Sci-Fi'}})<-[:BELONGS_TO]-(m:Movie)<-[:DIRECTED]-(d:Director) \
            WHERE length(shortestPath((u)-[*]-(d))) <= {} \
            RETURN m.title, d.name LIMIT 10",
            user_id, max_degrees
        );
        
        cypher_query
    }
}
