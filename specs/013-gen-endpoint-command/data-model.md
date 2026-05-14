# Data Model: Gen Endpoint Command

## Data Entities

### 1. Endpoint Generation Arguments (`GenEndpointArgs`)

- **Fields**:
  - `operation_type`: Enum string representing HTTP verb (GET, POST, PUT, DELETE)
  - `name`: Target command or query name
  - `feature`: Target feature folder in the Application layer
  - `service`: Optional target service (if multiple exist in workspace)
- **Validation Rules**:
  - MUST match existing standard HTTP verbs (Get, Post, Put, Delete).
  - MUST find the file `Application/Features/{feature}/{name}.cs` that represents the request.

### 2. Generator Template Payload

- **Fields sent to Template Engine**:
  - `operation_type`: (String) e.g. "Get"
  - `name`: (String) e.g. "GetInventoryItem"
  - `feature`: (String) e.g. "Inventory"
  - `service_name`: (String) e.g. "Catalog"
  - `request_type`: (String) Evaluated from the name (Command vs Query) to map HTTP return types intuitively if needed.
  - `route_path`: (String) Evaluated from the feature and name (e.g. "/api/inventory/getinventoryitem").
