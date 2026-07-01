#!/bin/bash
curl -X PUT \
  -H "Content-Type: application/json" \
  -d '{"titulo":"Titulo Atualizado","descricao":"Nova descricao"}' \
  http://127.0.0.1:8000/recursos/1