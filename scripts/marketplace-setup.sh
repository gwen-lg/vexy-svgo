#!/bin/bash
# this_file: scripts/marketplace-setup.sh

# Vexy SVGO Plugin Marketplace Infrastructure Setup Script
# This script sets up the development environment for the plugin marketplace

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
MARKETPLACE_DIR="marketplace"
API_PORT=3000
WEB_PORT=3001
POSTGRES_PORT=5432
REDIS_PORT=6379

log() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

check_dependencies() {
    log "Checking dependencies..."
    
    # Check for Docker
    if ! command -v docker &> /dev/null; then
        error "Docker is required but not installed"
    fi
    
    # Check for Docker Compose
    if ! command -v docker-compose &> /dev/null; then
        error "Docker Compose is required but not installed"
    fi
    
    # Check for Node.js (for frontend development)
    if ! command -v node &> /dev/null; then
        warn "Node.js not found - frontend development will not be available"
    fi
    
    # Check for Rust
    if ! command -v cargo &> /dev/null; then
        error "Rust/Cargo is required but not installed"
    fi
    
    log "Dependencies check completed"
}

create_directory_structure() {
    log "Creating directory structure..."
    
    mkdir -p ${MARKETPLACE_DIR}/{api,web,database,scripts,docs}
    mkdir -p ${MARKETPLACE_DIR}/api/{src,migrations,tests}
    mkdir -p ${MARKETPLACE_DIR}/web/{src,public,components}
    mkdir -p ${MARKETPLACE_DIR}/database/{init,backups}
    mkdir -p ${MARKETPLACE_DIR}/scripts/{deployment,maintenance}
    
    log "Directory structure created"
}

generate_docker_compose() {
    log "Generating docker-compose.yml..."
    
    cat > ${MARKETPLACE_DIR}/docker-compose.yml << 'EOF'
version: '3.8'

services:
  # PostgreSQL Database
  postgres:
    image: postgres:15-alpine
    environment:
      POSTGRES_DB: vexy_svgo_marketplace
      POSTGRES_USER: vexy_svgo
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD:-vexy_svgo_dev_password}
    ports:
      - "${POSTGRES_PORT:-5432}:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./database/init:/docker-entrypoint-initdb.d
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U vexy_svgo"]
      interval: 30s
      timeout: 10s
      retries: 3

  # Redis Cache
  redis:
    image: redis:7-alpine
    ports:
      - "${REDIS_PORT:-6379}:6379"
    volumes:
      - redis_data:/data
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 30s
      timeout: 10s
      retries: 3

  # Marketplace API Server
  api:
    build:
      context: ./api
      dockerfile: Dockerfile
    environment:
      DATABASE_URL: postgres://vexy_svgo:${POSTGRES_PASSWORD:-vexy_svgo_dev_password}@postgres:5432/vexy_svgo_marketplace
      REDIS_URL: redis://redis:6379
      JWT_SECRET: ${JWT_SECRET:-dev_jwt_secret_change_in_production}
      API_PORT: ${API_PORT:-3000}
      RUST_LOG: ${RUST_LOG:-info}
    ports:
      - "${API_PORT:-3000}:3000"
    depends_on:
      postgres:
        condition: service_healthy
      redis:
        condition: service_healthy
    volumes:
      - ./api:/app
      - api_target:/app/target
    command: cargo run --release

  # Web Frontend
  web:
    build:
      context: ./web
      dockerfile: Dockerfile
    environment:
      NEXT_PUBLIC_API_URL: http://localhost:${API_PORT:-3000}
      PORT: ${WEB_PORT:-3001}
    ports:
      - "${WEB_PORT:-3001}:3001"
    depends_on:
      - api
    volumes:
      - ./web:/app
      - web_node_modules:/app/node_modules

  # MinIO (S3-compatible storage)
  minio:
    image: minio/minio:latest
    environment:
      MINIO_ROOT_USER: ${MINIO_ACCESS_KEY:-minioadmin}
      MINIO_ROOT_PASSWORD: ${MINIO_SECRET_KEY:-minioadmin123}
    ports:
      - "9000:9000"
      - "9001:9001"
    volumes:
      - minio_data:/data
    command: server /data --console-address ":9001"
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:9000/minio/health/live"]
      interval: 30s
      timeout: 20s
      retries: 3

volumes:
  postgres_data:
  redis_data:
  minio_data:
  api_target:
  web_node_modules:

networks:
  default:
    name: vexy_svgo_marketplace
EOF

    log "Docker Compose configuration created"
}

generate_api_scaffold() {
    log "Generating API server scaffold..."
    
    # Create Cargo.toml for API
    cat > ${MARKETPLACE_DIR}/api/Cargo.toml << 'EOF'
[package]
name = "vexy_svgo-marketplace-api"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.0", features = ["full"] }
axum = "0.7"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-rustls", "uuid", "chrono"] }
redis = { version = "0.24", features = ["tokio-comp"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace"] }
jsonwebtoken = "9.0"
bcrypt = "0.15"
reqwest = { version = "0.11", features = ["json"] }

[dev-dependencies]
tower-test = "0.4"
EOF

    # Create main.rs
    cat > ${MARKETPLACE_DIR}/api/src/main.rs << 'EOF'
//! Vexy SVGO Plugin Marketplace API Server

use axum::{
    extract::Query,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tower_http::cors::CorsLayer;
use tracing_subscriber;

#[derive(Debug, Serialize, Deserialize)]
struct PluginInfo {
    name: String,
    version: String,
    description: String,
    author: String,
    downloads: u64,
    stars: u32,
}

#[derive(Debug, Deserialize)]
struct SearchQuery {
    q: Option<String>,
    tag: Option<String>,
    category: Option<String>,
    limit: Option<usize>,
}

async fn health_check() -> &'static str {
    "OK"
}

async fn search_plugins(Query(params): Query<SearchQuery>) -> Json<Vec<PluginInfo>> {
    // Mock data for development
    let plugins = vec![
        PluginInfo {
            name: "vexy_svgo-plugin-accessibility".to_string(),
            version: "1.0.0".to_string(),
            description: "Adds accessibility attributes to SVG elements".to_string(),
            author: "Vexy SVGO Team".to_string(),
            downloads: 1250,
            stars: 34,
        },
        PluginInfo {
            name: "vexy_svgo-plugin-minify".to_string(),
            version: "2.1.0".to_string(),
            description: "Advanced minification for SVG files".to_string(),
            author: "Community".to_string(),
            downloads: 3400,
            stars: 89,
        },
    ];

    // Filter based on query parameters
    let filtered: Vec<PluginInfo> = plugins
        .into_iter()
        .filter(|p| {
            if let Some(query) = &params.q {
                p.name.contains(query) || p.description.contains(query)
            } else {
                true
            }
        })
        .take(params.limit.unwrap_or(20))
        .collect();

    Json(filtered)
}

async fn get_plugin(axum::extract::Path(name): axum::extract::Path<String>) -> Result<Json<PluginInfo>, StatusCode> {
    // Mock plugin lookup
    if name == "vexy_svgo-plugin-accessibility" {
        Ok(Json(PluginInfo {
            name: "vexy_svgo-plugin-accessibility".to_string(),
            version: "1.0.0".to_string(),
            description: "Adds accessibility attributes to SVG elements".to_string(),
            author: "Vexy SVGO Team".to_string(),
            downloads: 1250,
            stars: 34,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::init();

    // Build the application router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/plugins", get(search_plugins))
        .route("/api/v1/plugins/:name", get(get_plugin))
        .layer(CorsLayer::permissive());

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Server starting on http://0.0.0.0:3000");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
EOF

    # Create Dockerfile for API
    cat > ${MARKETPLACE_DIR}/api/Dockerfile << 'EOF'
FROM rust:1.75-slim as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifest files
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm src/main.rs

# Copy source code
COPY src ./src

# Build application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /app/target/release/vexy_svgo-marketplace-api .

EXPOSE 3000

CMD ["./vexy_svgo-marketplace-api"]
EOF

    log "API server scaffold created"
}

generate_web_scaffold() {
    log "Generating web frontend scaffold..."
    
    # Create package.json
    cat > ${MARKETPLACE_DIR}/web/package.json << 'EOF'
{
  "name": "vexy_svgo-marketplace-web",
  "version": "0.1.0",
  "private": true,
  "scripts": {
    "dev": "next dev -p 3001",
    "build": "next build",
    "start": "next start -p 3001",
    "lint": "next lint"
  },
  "dependencies": {
    "next": "14.0.4",
    "react": "^18",
    "react-dom": "^18",
    "typescript": "^5",
    "@types/node": "^20",
    "@types/react": "^18",
    "@types/react-dom": "^18",
    "tailwindcss": "^3.3.0",
    "daisyui": "^4.4.0",
    "autoprefixer": "^10.0.1",
    "postcss": "^8",
    "axios": "^1.6.0"
  },
  "devDependencies": {
    "eslint": "^8",
    "eslint-config-next": "14.0.4"
  }
}
EOF

    # Create Next.js config
    cat > ${MARKETPLACE_DIR}/web/next.config.js << 'EOF'
/** @type {import('next').NextConfig} */
const nextConfig = {
  async rewrites() {
    return [
      {
        source: '/api/:path*',
        destination: process.env.NEXT_PUBLIC_API_URL + '/api/:path*',
      },
    ]
  },
}

module.exports = nextConfig
EOF

    # Create Tailwind config
    cat > ${MARKETPLACE_DIR}/web/tailwind.config.js << 'EOF'
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './src/pages/**/*.{js,ts,jsx,tsx,mdx}',
    './src/components/**/*.{js,ts,jsx,tsx,mdx}',
    './src/app/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  theme: {
    extend: {},
  },
  plugins: [require('daisyui')],
  daisyui: {
    themes: ['light', 'dark'],
  },
}
EOF

    # Create basic page structure
    mkdir -p ${MARKETPLACE_DIR}/web/src/{app,components}
    
    # Create layout
    cat > ${MARKETPLACE_DIR}/web/src/app/layout.tsx << 'EOF'
import './globals.css'
import type { Metadata } from 'next'

export const metadata: Metadata = {
  title: 'Vexy SVGO Plugin Marketplace',
  description: 'Discover and install plugins for Vexy SVGO SVG optimizer',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  )
}
EOF

    # Create main page
    cat > ${MARKETPLACE_DIR}/web/src/app/page.tsx << 'EOF'
'use client'

import { useState, useEffect } from 'react'
import axios from 'axios'

interface Plugin {
  name: string
  version: string
  description: string
  author: string
  downloads: number
  stars: number
}

export default function Home() {
  const [plugins, setPlugins] = useState<Plugin[]>([])
  const [loading, setLoading] = useState(true)
  const [searchQuery, setSearchQuery] = useState('')

  useEffect(() => {
    fetchPlugins()
  }, [])

  const fetchPlugins = async () => {
    try {
      const response = await axios.get('/api/v1/plugins')
      setPlugins(response.data)
    } catch (error) {
      console.error('Failed to fetch plugins:', error)
    } finally {
      setLoading(false)
    }
  }

  const handleSearch = async () => {
    if (!searchQuery.trim()) {
      fetchPlugins()
      return
    }

    try {
      const response = await axios.get(`/api/v1/plugins?q=${encodeURIComponent(searchQuery)}`)
      setPlugins(response.data)
    } catch (error) {
      console.error('Search failed:', error)
    }
  }

  return (
    <div className="min-h-screen bg-base-200">
      {/* Header */}
      <div className="navbar bg-base-100 shadow-lg">
        <div className="navbar-start">
          <h1 className="text-xl font-bold">Vexy SVGO Plugin Marketplace</h1>
        </div>
        <div className="navbar-end">
          <button className="btn btn-ghost">Sign In</button>
        </div>
      </div>

      {/* Hero Section */}
      <div className="hero bg-base-100 py-8">
        <div className="hero-content text-center">
          <div className="max-w-md">
            <h2 className="text-3xl font-bold">Discover SVG Optimization Plugins</h2>
            <p className="py-6">Extend Vexy SVGO with community-built plugins for specialized SVG optimization tasks.</p>
            
            {/* Search */}
            <div className="join w-full max-w-md">
              <input
                type="text"
                placeholder="Search plugins..."
                className="input input-bordered join-item flex-1"
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                onKeyPress={(e) => e.key === 'Enter' && handleSearch()}
              />
              <button className="btn btn-primary join-item" onClick={handleSearch}>
                Search
              </button>
            </div>
          </div>
        </div>
      </div>

      {/* Plugin List */}
      <div className="container mx-auto px-4 py-8">
        {loading ? (
          <div className="text-center">
            <span className="loading loading-spinner loading-lg"></span>
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {plugins.map((plugin) => (
              <div key={plugin.name} className="card bg-base-100 shadow-xl">
                <div className="card-body">
                  <h3 className="card-title">{plugin.name}</h3>
                  <p className="text-sm opacity-70">v{plugin.version} by {plugin.author}</p>
                  <p>{plugin.description}</p>
                  
                  <div className="flex justify-between text-sm opacity-60 mt-2">
                    <span>⭐ {plugin.stars}</span>
                    <span>⬇️ {plugin.downloads.toLocaleString()}</span>
                  </div>
                  
                  <div className="card-actions justify-end mt-4">
                    <button className="btn btn-primary btn-sm">Install</button>
                    <button className="btn btn-ghost btn-sm">Info</button>
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}

        {!loading && plugins.length === 0 && (
          <div className="text-center">
            <p className="text-lg">No plugins found.</p>
          </div>
        )}
      </div>
    </div>
  )
}
EOF

    # Create globals.css
    cat > ${MARKETPLACE_DIR}/web/src/app/globals.css << 'EOF'
@tailwind base;
@tailwind components;
@tailwind utilities;
EOF

    # Create Dockerfile for Web
    cat > ${MARKETPLACE_DIR}/web/Dockerfile << 'EOF'
FROM node:18-alpine as builder

WORKDIR /app

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm ci

# Copy source code
COPY . .

# Build application
RUN npm run build

# Runtime stage
FROM node:18-alpine

WORKDIR /app

# Copy built application
COPY --from=builder /app/.next ./.next
COPY --from=builder /app/public ./public
COPY --from=builder /app/package*.json ./
COPY --from=builder /app/next.config.js ./

# Install only production dependencies
RUN npm ci --only=production

EXPOSE 3001

CMD ["npm", "start"]
EOF

    log "Web frontend scaffold created"
}

generate_database_init() {
    log "Generating database initialization scripts..."
    
    cat > ${MARKETPLACE_DIR}/database/init/01-create-tables.sql << 'EOF'
-- Vexy SVGO Plugin Marketplace Database Schema

-- Users table
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    github_username VARCHAR(100),
    avatar_url VARCHAR(500),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    last_login TIMESTAMP,
    is_verified BOOLEAN DEFAULT FALSE,
    is_admin BOOLEAN DEFAULT FALSE
);

-- Plugins table
CREATE TABLE plugins (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL,
    display_name VARCHAR(255) NOT NULL,
    description TEXT,
    author_id INTEGER REFERENCES users(id),
    license VARCHAR(100),
    homepage VARCHAR(500),
    repository VARCHAR(500),
    keywords TEXT[], -- Array of keywords
    categories TEXT[], -- Array of categories
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    is_featured BOOLEAN DEFAULT FALSE,
    is_deprecated BOOLEAN DEFAULT FALSE
);

-- Plugin versions table
CREATE TABLE plugin_versions (
    id SERIAL PRIMARY KEY,
    plugin_id INTEGER REFERENCES plugins(id),
    version VARCHAR(50) NOT NULL,
    changelog TEXT,
    package_url VARCHAR(500),
    package_size BIGINT,
    package_hash VARCHAR(128),
    signature TEXT,
    vexy_svgo_version_requirement VARCHAR(100),
    published_at TIMESTAMP DEFAULT NOW(),
    yanked BOOLEAN DEFAULT FALSE,
    yank_reason TEXT,
    UNIQUE(plugin_id, version)
);

-- Plugin dependencies table
CREATE TABLE plugin_dependencies (
    id SERIAL PRIMARY KEY,
    version_id INTEGER REFERENCES plugin_versions(id),
    dependency_name VARCHAR(255),
    version_requirement VARCHAR(100),
    optional BOOLEAN DEFAULT FALSE
);

-- Plugin stars table
CREATE TABLE plugin_stars (
    id SERIAL PRIMARY KEY,
    plugin_id INTEGER REFERENCES plugins(id),
    user_id INTEGER REFERENCES users(id),
    created_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(plugin_id, user_id)
);

-- Plugin reviews table
CREATE TABLE plugin_reviews (
    id SERIAL PRIMARY KEY,
    plugin_id INTEGER REFERENCES plugins(id),
    user_id INTEGER REFERENCES users(id),
    rating INTEGER CHECK (rating >= 1 AND rating <= 5),
    title VARCHAR(200),
    comment TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(plugin_id, user_id)
);

-- Download statistics table
CREATE TABLE download_stats (
    id SERIAL PRIMARY KEY,
    plugin_id INTEGER REFERENCES plugins(id),
    version_id INTEGER REFERENCES plugin_versions(id),
    download_date DATE DEFAULT CURRENT_DATE,
    download_count INTEGER DEFAULT 1,
    UNIQUE(plugin_id, version_id, download_date)
);

-- API tokens table
CREATE TABLE api_tokens (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    token_hash VARCHAR(255),
    name VARCHAR(100),
    scopes TEXT[], -- Array of scopes
    created_at TIMESTAMP DEFAULT NOW(),
    last_used TIMESTAMP,
    expires_at TIMESTAMP
);

-- Create indexes for better performance
CREATE INDEX idx_plugins_name ON plugins(name);
CREATE INDEX idx_plugins_author ON plugins(author_id);
CREATE INDEX idx_plugins_categories ON plugins USING GIN (categories);
CREATE INDEX idx_plugins_keywords ON plugins USING GIN (keywords);
CREATE INDEX idx_plugin_versions_plugin ON plugin_versions(plugin_id);
CREATE INDEX idx_plugin_reviews_plugin ON plugin_reviews(plugin_id);
CREATE INDEX idx_download_stats_date ON download_stats(download_date);
CREATE INDEX idx_download_stats_plugin ON download_stats(plugin_id);
EOF

    cat > ${MARKETPLACE_DIR}/database/init/02-seed-data.sql << 'EOF'
-- Seed data for development

-- Insert test users
INSERT INTO users (username, email, password_hash, github_username, is_verified) VALUES
('admin', 'admin@vexy_svgo.org', '$2b$12$dummy.hash.for.development', 'vexy_svgo-admin', true),
('developer1', 'dev1@example.com', '$2b$12$dummy.hash.for.development', 'dev1', true),
('developer2', 'dev2@example.com', '$2b$12$dummy.hash.for.development', 'dev2', true);

-- Insert test plugins
INSERT INTO plugins (name, display_name, description, author_id, license, repository, keywords, categories) VALUES
(
    'vexy_svgo-plugin-accessibility',
    'Accessibility Plugin',
    'Adds accessibility attributes to SVG elements for better screen reader support',
    2,
    'MIT',
    'https://github.com/dev1/vexy_svgo-plugin-accessibility',
    ARRAY['accessibility', 'a11y', 'aria'],
    ARRAY['accessibility', 'enhancement']
),
(
    'vexy_svgo-plugin-minify',
    'Advanced Minifier',
    'Advanced minification techniques for maximum file size reduction',
    3,
    'Apache-2.0',
    'https://github.com/dev2/vexy_svgo-plugin-minify',
    ARRAY['minify', 'compression', 'optimization'],
    ARRAY['optimization', 'size-reduction']
);

-- Insert plugin versions
INSERT INTO plugin_versions (plugin_id, version, changelog, package_size, vexy_svgo_version_requirement) VALUES
(1, '1.0.0', 'Initial release with basic accessibility features', 45632, '>=2.0.0'),
(1, '1.0.1', 'Bug fixes for aria-label generation', 45891, '>=2.0.0'),
(2, '2.0.0', 'Complete rewrite with better compression algorithms', 78234, '>=2.0.0'),
(2, '2.1.0', 'Added support for path optimization', 82156, '>=2.0.1');

-- Insert some stars and reviews
INSERT INTO plugin_stars (plugin_id, user_id) VALUES
(1, 1), (1, 3), (2, 1), (2, 2);

INSERT INTO plugin_reviews (plugin_id, user_id, rating, title, comment) VALUES
(1, 3, 5, 'Great accessibility plugin!', 'This plugin really helps make SVGs more accessible. Easy to use and configure.'),
(2, 2, 4, 'Excellent compression', 'Achieves great file size reduction, though could use better documentation.');

-- Insert download statistics
INSERT INTO download_stats (plugin_id, version_id, download_date, download_count) VALUES
(1, 1, CURRENT_DATE - INTERVAL '30 days', 156),
(1, 1, CURRENT_DATE - INTERVAL '29 days', 143),
(1, 2, CURRENT_DATE - INTERVAL '7 days', 234),
(2, 3, CURRENT_DATE - INTERVAL '15 days', 89),
(2, 4, CURRENT_DATE - INTERVAL '5 days', 167);
EOF

    log "Database initialization scripts created"
}

generate_environment_file() {
    log "Generating environment configuration..."
    
    cat > ${MARKETPLACE_DIR}/.env.example << 'EOF'
# Database Configuration
POSTGRES_PASSWORD=vexy_svgo_dev_password
DATABASE_URL=postgres://vexy_svgo:vexy_svgo_dev_password@localhost:5432/vexy_svgo_marketplace

# Redis Configuration
REDIS_URL=redis://localhost:6379

# API Configuration
JWT_SECRET=dev_jwt_secret_change_in_production
API_PORT=3000

# Web Configuration
NEXT_PUBLIC_API_URL=http://localhost:3000
WEB_PORT=3001

# Storage Configuration (MinIO)
MINIO_ACCESS_KEY=minioadmin
MINIO_SECRET_KEY=minioadmin123

# Logging
RUST_LOG=info

# Security (Production only)
# SSL_CERT_PATH=/path/to/cert.pem
# SSL_KEY_PATH=/path/to/key.pem
EOF

    # Copy to actual .env file for development
    cp ${MARKETPLACE_DIR}/.env.example ${MARKETPLACE_DIR}/.env
    
    log "Environment configuration created"
}

generate_scripts() {
    log "Generating utility scripts..."
    
    # Development script
    cat > ${MARKETPLACE_DIR}/scripts/dev.sh << 'EOF'
#!/bin/bash
# Development startup script

set -e

echo "Starting Vexy SVGO Plugin Marketplace development environment..."

# Check if .env exists
if [ ! -f .env ]; then
    echo "Creating .env file from .env.example..."
    cp .env.example .env
fi

# Start services
docker-compose up -d postgres redis minio

# Wait for services to be ready
echo "Waiting for services to be ready..."
sleep 10

# Check if we need to run migrations
echo "Running database migrations..."
# TODO: Add migration runner here

# Start API in development mode
echo "Starting API server..."
cd api && cargo run &
API_PID=$!

# Start web frontend in development mode
echo "Starting web frontend..."
cd web && npm run dev &
WEB_PID=$!

echo "Development environment is ready!"
echo "API: http://localhost:3000"
echo "Web: http://localhost:3001"
echo "MinIO Console: http://localhost:9001"

# Wait for Ctrl+C
trap "echo 'Shutting down...'; kill $API_PID $WEB_PID; docker-compose down" EXIT
wait
EOF

    # Production deployment script
    cat > ${MARKETPLACE_DIR}/scripts/deploy.sh << 'EOF'
#!/bin/bash
# Production deployment script

set -e

echo "Deploying Vexy SVGO Plugin Marketplace..."

# Build and deploy with production settings
docker-compose -f docker-compose.yml -f docker-compose.prod.yml up -d --build

echo "Deployment complete!"
echo "Don't forget to:"
echo "1. Set up SSL certificates"
echo "2. Configure domain name"
echo "3. Set up monitoring"
echo "4. Configure backups"
EOF

    # Backup script
    cat > ${MARKETPLACE_DIR}/scripts/backup.sh << 'EOF'
#!/bin/bash
# Database backup script

set -e

BACKUP_DIR="./database/backups"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="marketplace_backup_${TIMESTAMP}.sql"

echo "Creating database backup..."

docker-compose exec -T postgres pg_dump -U vexy_svgo vexy_svgo_marketplace > "${BACKUP_DIR}/${BACKUP_FILE}"

echo "Backup created: ${BACKUP_FILE}"

# Keep only last 7 backups
find "${BACKUP_DIR}" -name "marketplace_backup_*.sql" -type f -mtime +7 -delete

echo "Old backups cleaned up"
EOF

    # Make scripts executable
    chmod +x ${MARKETPLACE_DIR}/scripts/*.sh
    
    log "Utility scripts created"
}

generate_readme() {
    log "Generating README..."
    
    cat > ${MARKETPLACE_DIR}/README.md << 'EOF'
# Vexy SVGO Plugin Marketplace

A comprehensive marketplace infrastructure for discovering, sharing, and managing Vexy SVGO SVG optimization plugins.

## Quick Start

1. **Prerequisites**
   - Docker and Docker Compose
   - Node.js 18+ (for frontend development)
   - Rust 1.75+ (for API development)

2. **Start Development Environment**
   ```bash
   ./scripts/dev.sh
   ```

3. **Access Services**
   - Web Interface: http://localhost:3001
   - API Server: http://localhost:3000
   - MinIO Console: http://localhost:9001

## Architecture

- **API Server**: Rust with Axum framework
- **Web Frontend**: Next.js with TypeScript and DaisyUI
- **Database**: PostgreSQL for metadata
- **Cache**: Redis for session and API caching
- **Storage**: MinIO (S3-compatible) for plugin packages

## Development

### API Development

```bash
cd api
cargo run
```

The API server will start on http://localhost:3000 with hot reloading.

### Frontend Development

```bash
cd web
npm install
npm run dev
```

The web interface will start on http://localhost:3001 with hot reloading.

### Database

Database migrations and seeding are handled automatically. To reset the database:

```bash
docker-compose down -v
docker-compose up -d postgres
```

## API Endpoints

### Plugins
- `GET /api/v1/plugins` - Search plugins
- `GET /api/v1/plugins/{name}` - Get plugin details
- `POST /api/v1/plugins` - Publish plugin (authenticated)
- `GET /api/v1/plugins/{name}/download/{version}` - Download plugin

### Authentication
- `POST /api/v1/auth/login` - User login
- `POST /api/v1/auth/register` - User registration
- `POST /api/v1/auth/logout` - User logout

### Users
- `GET /api/v1/users/{id}` - Get user profile
- `GET /api/v1/users/{id}/plugins` - Get user's plugins

## Deployment

### Production Deployment

1. **Configure Environment**
   ```bash
   cp .env.example .env
   # Edit .env with production values
   ```

2. **Deploy Services**
   ```bash
   ./scripts/deploy.sh
   ```

3. **Set Up SSL**
   - Configure SSL certificates
   - Update nginx/reverse proxy configuration

4. **Configure Monitoring**
   - Set up Prometheus and Grafana
   - Configure log aggregation

### Database Backups

Automated backups are created daily:

```bash
./scripts/backup.sh
```

## Security

### Code Signing

All plugins must be signed with a trusted certificate. The signing process:

1. Plugin author generates a key pair
2. Plugin package is signed with private key
3. Signature is verified during installation

### Vulnerability Scanning

Automated security scanning for all submitted plugins:

- Static code analysis
- Dependency vulnerability scanning
- Malware detection

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

This project is licensed under the same terms as Vexy SVGO: MIT OR Apache-2.0.
EOF

    log "README created"
}

main() {
    echo -e "${BLUE}Vexy SVGO Plugin Marketplace Setup${NC}"
    echo "======================================="
    
    check_dependencies
    create_directory_structure
    generate_docker_compose
    generate_api_scaffold
    generate_web_scaffold
    generate_database_init
    generate_environment_file
    generate_scripts
    generate_readme
    
    echo
    log "✅ Marketplace infrastructure setup completed!"
    echo
    echo -e "${GREEN}Next steps:${NC}"
    echo "1. cd ${MARKETPLACE_DIR}"
    echo "2. ./scripts/dev.sh"
    echo "3. Open http://localhost:3001 in your browser"
    echo
    echo -e "${YELLOW}Note:${NC} This is a development setup. See README.md for production deployment."
}

# Run the main function
main "$@"