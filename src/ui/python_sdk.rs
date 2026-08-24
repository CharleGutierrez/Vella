use tracing::info;

pub fn generate_python_polars_sdk(base_url: &str) -> String {
    info!("Generating native Python SDK with Polars integration");
    format!(
r#"# Auto-generated Vella Python SDK
import requests
try:
    import polars as pl
    import pyarrow as pa
except ImportError:
    print("For zero-copy dataframes, install polars and pyarrow: pip install polars pyarrow")

class VellaClient:
    def __init__(self, api_key: str):
        self.base_url = "{base_url}"
        self.api_key = api_key
        self.headers = {{"Authorization": f"Bearer {{api_key}}"}}

    def query_to_polars(self, table: str, limit: int = 1000) -> 'pl.DataFrame':
        """
        Fetches data via Apache Arrow IPC and loads it instantly into a Polars DataFrame
        with strictly zero-copy memory allocation.
        """
        url = f"{{self.base_url}}/api/d/{{table}}/export?format=arrow&limit={{limit}}"
        response = requests.get(url, headers=self.headers, stream=True)
        response.raise_for_status()
        
        # Load raw bytes into PyArrow IPC reader, then lazy load into Polars
        with pa.ipc.open_stream(response.content) as reader:
            return pl.from_arrow(reader.read_all())
"#,
        base_url = base_url
    )
}
