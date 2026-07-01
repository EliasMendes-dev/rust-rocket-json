#!/bin/bash

echo "1. Buscar ID 1 (existe):"
curl -s http://127.0.0.1:8000/recursos/1
echo -e "\n"

echo "2. Buscar ID 2 (existe):"
curl -s http://127.0.0.1:8000/recursos/2
echo -e "\n"

echo "3. Buscar ID 999 (não existe):"
curl -s http://127.0.0.1:8000/recursos/999
echo -e "\n"