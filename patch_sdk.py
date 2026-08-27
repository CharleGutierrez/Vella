import os

def replace_in_file(path, old, new):
    with open(path, 'r', encoding='utf-8') as f:
        content = f.read()
    content = content.replace(old, new)
    with open(path, 'w', encoding='utf-8') as f:
        f.write(content)

# 1. Update src/ui/mod.rs
mod_old = """pub struct UiConfig {
    pub site_name: String,
    pub base_url: String,
}"""
mod_new = """pub struct UiConfig {
    pub site_name: String,
    pub base_url: String,
    pub schemas: Vec<crate::model::schema::ModelSchema>,
}"""
replace_in_file('src/ui/mod.rs', mod_old, mod_new)

mod_react_old = """pub async fn react_sdk_handler(
    axum::extract::State(config): axum::extract::State<Arc<UiConfig>>,
) -> Response {
    let sdk_ts = react_sdk::generate_react_sdk(&config.base_url);"""
mod_react_new = """pub async fn react_sdk_handler(
    axum::extract::State(config): axum::extract::State<Arc<UiConfig>>,
) -> Response {
    let sdk_ts = react_sdk::generate_react_sdk(&config.base_url, &config.schemas);"""
replace_in_file('src/ui/mod.rs', mod_react_old, mod_react_new)

mod_vue_old = """pub async fn vue_sdk_handler(
    axum::extract::State(config): axum::extract::State<Arc<UiConfig>>,
) -> Response {
    let sdk_ts = vue_sdk::generate_vue_sdk(&config.base_url);"""
mod_vue_new = """pub async fn vue_sdk_handler(
    axum::extract::State(config): axum::extract::State<Arc<UiConfig>>,
) -> Response {
    let sdk_ts = vue_sdk::generate_vue_sdk(&config.base_url, &config.schemas);"""
replace_in_file('src/ui/mod.rs', mod_vue_old, mod_vue_new)

mod_angular_old = """pub async fn angular_sdk_handler(
    axum::extract::State(config): axum::extract::State<Arc<UiConfig>>,
) -> Response {
    let sdk_ts = angular_sdk::generate_angular_sdk(&config.base_url);"""
mod_angular_new = """pub async fn angular_sdk_handler(
    axum::extract::State(config): axum::extract::State<Arc<UiConfig>>,
) -> Response {
    let sdk_ts = angular_sdk::generate_angular_sdk(&config.base_url, &config.schemas);"""
replace_in_file('src/ui/mod.rs', mod_angular_old, mod_angular_new)

# 2. Update src/app.rs
app_old = """        let ui_config = Arc::new(UiConfig {
            site_name: self.config.site_name.clone(),
            base_url: format!("http://{}", self.config.bind_address),
        });"""
app_new = """        let ui_config = Arc::new(UiConfig {
            site_name: self.config.site_name.clone(),
            base_url: format!("http://{}", self.config.bind_address),
            schemas: self.schemas.values().cloned().collect(),
        });"""
replace_in_file('src/app.rs', app_old, app_new)

# 3. Update React SDK
react_old = """pub fn generate_react_sdk(base_url: &str) -> String {
    format!("""
react_new = """use crate::types::TypeScriptGenerator;
use crate::model::schema::ModelSchema;

pub fn generate_react_sdk(base_url: &str, schemas: &[ModelSchema]) -> String {
    let mut interfaces = String::new();
    for schema in schemas {
        interfaces.push_str(&TypeScriptGenerator::generate_model_interface(schema));
        interfaces.push_str("\\n\\n");
    }

    format!("""
replace_in_file('src/ui/react_sdk.rs', react_old, react_new)
replace_in_file('src/ui/react_sdk.rs', "export interface VellaUser {", "{interfaces}\nexport interface VellaUser {")

# 4. Update Vue SDK
vue_old = """pub fn generate_vue_sdk(base_url: &str) -> String {
    format!("""
vue_new = """use crate::types::TypeScriptGenerator;
use crate::model::schema::ModelSchema;

pub fn generate_vue_sdk(base_url: &str, schemas: &[ModelSchema]) -> String {
    let mut interfaces = String::new();
    for schema in schemas {
        interfaces.push_str(&TypeScriptGenerator::generate_model_interface(schema));
        interfaces.push_str("\\n\\n");
    }

    format!("""
replace_in_file('src/ui/vue_sdk.rs', vue_old, vue_new)
replace_in_file('src/ui/vue_sdk.rs', "export interface VellaUser {", "{interfaces}\nexport interface VellaUser {")

# 5. Update Angular SDK
angular_old = """pub fn generate_angular_sdk(base_url: &str) -> String {
    format!("""
angular_new = """use crate::types::TypeScriptGenerator;
use crate::model::schema::ModelSchema;

pub fn generate_angular_sdk(base_url: &str, schemas: &[ModelSchema]) -> String {
    let mut interfaces = String::new();
    for schema in schemas {
        interfaces.push_str(&TypeScriptGenerator::generate_model_interface(schema));
        interfaces.push_str("\\n\\n");
    }

    format!("""
replace_in_file('src/ui/angular_sdk.rs', angular_old, angular_new)
replace_in_file('src/ui/angular_sdk.rs', "export interface VellaUser {", "{interfaces}\nexport interface VellaUser {")
