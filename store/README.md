# Store Module

This module handles database operations for the user management system.

## Fixed Issues

### 1. User Model (`store/src/models/user.rs`)
- **Fixed lifetime issues**: Removed lifetime parameters from `NewUser` struct
- **Added missing imports**: Added `OptionalExtension` for the `optional()` method
- **Fixed timestamp handling**: Added proper `createdat` field handling with `Utc::now().naive_utc()`
- **Added utility method**: Added `get_user_by_username()` method for better functionality

### 2. Store Module (`store/src/store.rs`)
- **Fixed missing import**: Added `ConnectionError` import from diesel

### 3. Models Module (`store/src/models/mod.rs`)
- **Fixed visibility**: Made the `user` module public

### 4. Database Migration (`store/migrations/2025-07-26-043125_create_users/up.sql`)
- **Fixed column name**: Changed `createdAt` to `createdat` to match the schema

### 5. API Dependencies (`api/Cargo.toml`)
- **Removed invalid dependency**: Removed non-existent `auth = "0.1.0"` dependency
- **Added missing dependency**: Added `uuid` with proper features

## Usage

The store module provides the following main functionality:

- `sign_up()`: Create a new user account
- `sign_in()`: Authenticate a user
- `get_user_by_username()`: Retrieve a user by username

## Database Schema

The users table has the following structure:
- `id`: UUID (primary key)
- `username`: TEXT (unique)
- `password`: TEXT (unique)
- `email`: TEXT (unique)
- `createdat`: TIMESTAMP (defaults to current timestamp) 