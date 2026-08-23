use serde::{Deserialize, Serialize};

/// The data types and specialized widgets supported by Vella fields,
/// including native vector embeddings for LLMs and RAG architectures.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "config")]
pub enum FieldType {
    String,
    Integer,
    Float,
    Boolean,
    DateTime,
    Email,
    Password,
    Html,
    Markdown,
    Money { currency: String },
    ProgressBar { max: f64, color: String },
    Image { upload_dir: String },
    File { upload_dir: String },
    ForeignKey { target_model: String },
    Enum { choices: Vec<String> },
    Json,
    /// Native Vector Embedding for pgvector, sqlite-vec, and semantic RAG search
    Vector { dimensions: usize },
}

/// Metadata, validation rules, and UI rendering hints for a model field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub display_name: String,
    pub field_type: FieldType,
    pub required: bool,
    pub unique: bool,
    pub searchable: bool,
    pub filterable: bool,
    pub list_display: bool,
    pub read_only: bool,
    pub encrypted: bool,
    pub requires_approval: bool,
    pub default_value: Option<serde_json::Value>,
    pub help_text: Option<String>,
}

impl Field {
    pub fn new(name: impl Into<String>, field_type: FieldType) -> Self {
        let name_str = name.into();
        let display_name = Self::generate_display_name(&name_str);
        Self {
            name: name_str,
            display_name,
            field_type,
            required: false,
            unique: false,
            searchable: false,
            filterable: true,
            list_display: true,
            read_only: false,
            encrypted: false,
            requires_approval: false,
            default_value: None,
            help_text: None,
        }
    }

    pub fn string(name: impl Into<String>) -> Self {
        Self::new(name, FieldType::String)
    }

    pub fn integer(name: impl Into<String>) -> Self {
        Self::new(name, FieldType::Integer)
    }

    pub fn float(name: impl Into<String>) -> Self {
        Self::new(name, FieldType::Float)
    }

    pub fn boolean(name: impl Into<String>) -> Self {
        Self::new(name, FieldType::Boolean)
    }

    pub fn datetime(name: impl Into<String>) -> Self {
        Self::new(name, FieldType::DateTime)
    }

    pub fn email(name: impl Into<String>) -> Self {
        Self::new(name, FieldType::Email).searchable()
    }

    pub fn password(name: impl Into<String>) -> Self {
        let mut f = Self::new(name, FieldType::Password);
        f.list_display = false;
        f.searchable = false;
        f.required = true;
        f
    }

    pub fn html(name: impl Into<String>) -> Self {
        let mut f = Self::new(name, FieldType::Html);
        f.list_display = false;
        f
    }

    pub fn markdown(name: impl Into<String>) -> Self {
        let mut f = Self::new(name, FieldType::Markdown);
        f.list_display = false;
        f
    }

    pub fn money(name: impl Into<String>, currency: impl Into<String>) -> Self {
        Self::new(name, FieldType::Money { currency: currency.into() })
    }

    pub fn progress_bar(name: impl Into<String>, max: f64, color: impl Into<String>) -> Self {
        Self::new(name, FieldType::ProgressBar { max, color: color.into() })
    }

    pub fn image(name: impl Into<String>, upload_dir: impl Into<String>) -> Self {
        Self::new(name, FieldType::Image { upload_dir: upload_dir.into() })
    }

    pub fn file(name: impl Into<String>, upload_dir: impl Into<String>) -> Self {
        Self::new(name, FieldType::File { upload_dir: upload_dir.into() })
    }

    pub fn foreign_key(name: impl Into<String>, target_model: impl Into<String>) -> Self {
        Self::new(name, FieldType::ForeignKey { target_model: target_model.into() })
    }

    pub fn r#enum(name: impl Into<String>, choices: Vec<&str>) -> Self {
        Self::new(name, FieldType::Enum {
            choices: choices.into_iter().map(|s| s.to_string()).collect(),
        })
    }

    pub fn json(name: impl Into<String>) -> Self {
        let mut f = Self::new(name, FieldType::Json);
        f.list_display = false;
        f
    }

    /// Vector embedding field for LLM/RAG similarity search (e.g. OpenAI 1536d, Nomic 768d, etc.)
    pub fn vector(name: impl Into<String>, dimensions: usize) -> Self {
        let mut f = Self::new(name, FieldType::Vector { dimensions });
        f.list_display = false;
        f.filterable = false;
        f.searchable = true;
        f.help_text = Some(format!("Vector Embedding ({} dimensions)", dimensions));
        f
    }

    // Builder methods
    pub fn display_name(mut self, name: impl Into<String>) -> Self {
        self.display_name = name.into();
        self
    }

    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    pub fn unique(mut self) -> Self {
        self.unique = true;
        self
    }

    pub fn searchable(mut self) -> Self {
        self.searchable = true;
        self
    }

    pub fn filterable(mut self, val: bool) -> Self {
        self.filterable = val;
        self
    }

    pub fn list_display(mut self, val: bool) -> Self {
        self.list_display = val;
        self
    }

    pub fn read_only(mut self) -> Self {
        self.read_only = true;
        self
    }

    pub fn encrypted(mut self) -> Self {
        self.encrypted = true;
        self
    }

    pub fn requires_approval(mut self) -> Self {
        self.requires_approval = true;
        self
    }

    pub fn default_value(mut self, val: serde_json::Value) -> Self {
        self.default_value = Some(val);
        self
    }

    pub fn help(mut self, help: impl Into<String>) -> Self {
        self.help_text = Some(help.into());
        self
    }

    pub fn generate_display_name(raw: &str) -> String {
        let clean = raw.replace('_', " ");
        let mut chars = clean.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}
