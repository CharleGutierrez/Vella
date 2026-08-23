use vella::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 1. Category Model
    let category_schema = ModelSchema::new("Category")
        .category("E-Commerce")
        .icon("tag")
        .description("Product Categories and Taxonomies")
        .field(Field::string("name").required().unique().searchable())
        .field(Field::string("slug").unique().searchable())
        .field(Field::string("description"))
        .field(Field::boolean("is_active").default_value(serde_json::json!(true)))
        .with_timestamps();

    // 2. Product Model (with Money, Progress Bar, Approval on discount, and Vector Embeddings for visual / semantic search)
    let product_schema = ModelSchema::new("Product")
        .category("E-Commerce")
        .icon("shopping-bag")
        .description("Catalog Items with Vector Semantic Embeddings")
        .field(Field::string("title").required().searchable())
        .field(Field::string("sku").required().unique().searchable())
        .field(Field::money("price", "USD").required().filterable(true))
        .field(Field::float("discount_percent").requires_approval().help("Changes require Manager review"))
        .field(Field::progress_bar("stock_quantity", 500.0, "#22c55e").filterable(true))
        .field(Field::html("description").help("Rich text / Markdown formatted product overview"))
        .field(Field::r#enum("status", vec!["Draft", "Published", "Archived"]))
        .field(Field::foreign_key("category_id", "Category").help("Associated category ID"))
        .field(Field::vector("embedding", 1536).help("1536d semantic product embedding"))
        .field(Field::boolean("is_featured").default_value(serde_json::json!(false)))
        .with_timestamps();

    // 3. Customer Order Model
    let order_schema = ModelSchema::new("Order")
        .category("Sales & CRM")
        .icon("credit-card")
        .description("Customer Purchases and Payment Records")
        .field(Field::string("order_number").required().unique().searchable())
        .field(Field::email("customer_email").required().searchable())
        .field(Field::money("total_amount", "USD").required().filterable(true))
        .field(Field::r#enum("payment_status", vec!["Pending", "Paid", "Refunded", "Failed"]))
        .field(Field::r#enum("fulfillment_status", vec!["Unfulfilled", "Processing", "Shipped", "Delivered"]))
        .field(Field::string("shipping_address"))
        .with_timestamps();

    // 4. Support Ticket & Knowledge Article Model
    let ticket_schema = ModelSchema::new("Ticket")
        .category("Support")
        .icon("life-buoy")
        .description("Customer Inquiries and Helpdesk")
        .field(Field::string("subject").required().searchable())
        .field(Field::email("user_email").required().searchable())
        .field(Field::r#enum("priority", vec!["Low", "Medium", "High", "Critical"]))
        .field(Field::r#enum("status", vec!["Open", "In_Progress", "Resolved", "Closed"]))
        .field(Field::markdown("message").required())
        .field(Field::vector("embedding", 1536).help("Ticket similarity embedding"))
        .with_timestamps();

    // 5. Build and launch Vella Next-Gen Engine!
    VellaApp::new()
        .site_name("AcroStore Vella Edition")
        .bind("0.0.0.0:8080")
        .database("sqlite://vella_demo.db?mode=rwc")
        .auto_export_types_to("./frontend/types/vella.d.ts")
        .semantic_cache(true, 0.90)
        .register(category_schema)
        .register(product_schema)
        .register(order_schema)
        .register(ticket_schema)
        .run()
        .await?;

    Ok(())
}
