# MCP Protocol Draft

Search-Mesh communicates over JSON-RPC stdio using MCP-style `tools/call` requests.

## Tool: `pulse_hyper_scan`

Scans target directories for multiple keywords and returns line-oriented matches.

### Request

```json
{
  "name": "pulse_hyper_scan",
  "arguments": {
    "targetDirs": ["src/"],
    "keywords": ["TODO", "FIXME", "deprecated"]
  }
}
```

### Response

```json
{
  "content": [
    {
      "type": "text",
      "text": "[{\"file\":\"src/main.rs\",\"line\":142,\"keyword\":\"TODO\",\"matchStr\":\"// TODO: Refactor AhoNode initialization\"}]"
    }
  ]
}
```

## Tool: `pulse_ast_probe`

Validates whether a pattern appears in a requested syntax node type.

### Request

```json
{
  "name": "pulse_ast_probe",
  "arguments": {
    "filePath": "src/main.rs",
    "queryPattern": "AhoNode",
    "nodeType": "struct"
  }
}
```

### Response

```json
{
  "content": [
    {
      "type": "text",
      "text": "{\"isValid\":true,\"nodeType\":\"struct_definition\",\"startLine\":12,\"endLine\":48}"
    }
  ]
}
```

## Tool: `pulse_squeeze`

Returns the nearest useful AST-bounded block around a match.

Status: planned.

## Tool: `pulse_patch`

Applies byte-offset edits and verifies the resulting file.

Status: planned.

## Error Shape

Tool failures should use JSON-RPC errors at the transport layer when request dispatch fails. Tool-level validation errors should return an MCP content payload with a concise message until the server has a fuller typed error model.
