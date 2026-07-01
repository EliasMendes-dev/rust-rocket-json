#!/bin/bash

echo "========================================"
echo "1. Testando acesso SEM token"
echo "========================================"
echo "Saída esperada:"
echo '{"erro":"Sem autorização para acessar esta área"}'
echo

curl -s -X GET http://localhost:8000/recursos

echo
echo
echo "========================================"
echo "2. Realizando login..."
echo "========================================"
echo "Saída esperada:"
echo '{"id":1,"nome":"Elias","email":"elias@teste.com","token":"<JWT>"}'
echo

LOGIN_RESPONSE=$(curl -s -X POST http://localhost:8000/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "elias@teste.com",
    "senha": "123456"
  }')

echo "$LOGIN_RESPONSE"

echo
echo "========================================"
echo "3. Extraindo o token..."
echo "========================================"

TOKEN=$(echo "$LOGIN_RESPONSE" | sed -n 's/.*"token":"\([^"]*\)".*/\1/p')

echo "Token obtido:"
echo "$TOKEN"

echo
echo "========================================"
echo "4. Testando acesso COM token"
echo "========================================"
echo "Saída esperada:"
echo '[{"id":1,"titulo":"Recurso 1","descricao":"Descrição do recurso 1"}, ...]'
echo

curl -s -X GET http://localhost:8000/recursos \
  -H "Authorization: Bearer $TOKEN"

echo
echo
echo "========================================"
echo "Fim dos testes"
echo "========================================"