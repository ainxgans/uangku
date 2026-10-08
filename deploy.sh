#!/usr/bin/env bash
set -e

echo "🚀 [Uangku] Starting deployment..."

# Navigate to project directory
cd "$(dirname "$0")"

echo "📥 [Uangku] Fetching and updating code from origin/main..."
git fetch origin main
git reset --hard origin/main

echo "🐳 [Uangku] Building and restarting containers..."
docker compose up -d --build

echo "🧹 [Uangku] Cleaning dangling images..."
docker image prune -f

echo "🩺 [Uangku] Checking container status..."
sleep 3
docker compose ps

echo "✅ [Uangku] Deployment completed successfully!"
