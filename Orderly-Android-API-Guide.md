# Orderly Android App - Arrow Server API Integration Guide

This document provides comprehensive guidance for integrating your **Orderly** Android app (built in Kotlin) with the **Arrow Server** backend deployed on Azure.

## Overview

**Arrow Server** is a high-performance REST API for restaurant order management built with:
- **Rust** (Axum web framework)
- **MySQL** database with Diesel ORM
- **JWT Authentication** with 60-minute token expiration
- **Argon2** password hashing
- **Role-based access control**

## Base API Configuration

### Azure Deployment
- **Production URL**: Replace with your actual Azure deployment URL
- **Local Development**: `http://127.0.0.1:3000`

### API Base Path
All endpoints are prefixed with: `/api/v1`

⚠️ **IMPORTANT**: Please provide your actual Azure deployment URL for production integration. All Bruno collection examples currently use `http://127.0.0.1:3000` for local development.

## TODO List

### Backend Server (Arrow Server)
- [ ] **Azure URL Configuration**: Provide actual Azure deployment URL to replace placeholder
- [ ] **JWT Environment Variables**: Configure `JWT_SECRET` and `JWT_EXPIRATION_MINUTES` in Azure
- [ ] **Database Configuration**: Set up `DATABASE_URL` environment variable in Azure
- [ ] **Azure Blob Storage**: Implement image storage service (currently TODO in codebase)
- [ ] **HTTPS Certificate**: Ensure SSL/TLS is properly configured in Azure
- [ ] **CORS Configuration**: Set up CORS for Android app domain if needed

### Android App (Orderly)
- [ ] **Project Setup**: Create new Android Studio project with Kotlin
- [ ] **Dependencies**: Add Retrofit, OkHttp, Gson, JWT decoder libraries
- [ ] **Network Layer**: Implement ApiService interface and NetworkModule
- [ ] **Authentication System**: 
  - [ ] Token storage with Android Keystore
  - [ ] Login/Register screens
  - [ ] Token refresh mechanism
  - [ ] Auto-logout on token expiration
- [ ] **UI Screens**:
  - [ ] Splash screen with token validation
  - [ ] Login/Register screen
  - [ ] Product catalog (RecyclerView)
  - [ ] Product details screen
  - [ ] Shopping cart functionality
  - [ ] Order placement screen
  - [ ] Order history screen
  - [ ] User profile screen
- [ ] **Data Layer**:
  - [ ] Room database for local caching
  - [ ] Repository pattern implementation
  - [ ] Data models matching API responses
- [ ] **Business Logic**:
  - [ ] Order total calculation
  - [ ] Quantity validation
  - [ ] Order status tracking
  - [ ] Error handling and retry logic
- [ ] **Security**:
  - [ ] Certificate pinning implementation
  - [ ] Input validation and sanitization
  - [ ] Secure token storage
  - [ ] Network security config

### Testing & Quality
- [ ] **Backend Testing**: Add unit tests for services and controllers
- [ ] **Android Testing**: 
  - [ ] Unit tests for repositories and ViewModels
  - [ ] UI tests for critical user flows
  - [ ] Integration tests for API calls
- [ ] **API Documentation**: Generate Swagger/OpenAPI documentation
- [ ] **Performance Testing**: Load testing for order processing
- [ ] **Security Audit**: Penetration testing for authentication

### Production Deployment
- [ ] **Environment Configuration**: Separate dev/staging/prod configurations
- [ ] **Monitoring & Logging**: Set up application insights and logging
- [ ] **Database Backup**: Configure automated MySQL backups
- [ ] **CI/CD Pipeline**: Set up GitHub Actions for automated deployment
- [ ] **Error Tracking**: Implement error reporting (Sentry, Bugsnag)
- [ ] **Analytics**: Add user behavior tracking
- [ ] **App Store Preparation**: Prepare for Google Play Store submission

### Future Enhancements
- [ ] **Push Notifications**: Real-time order status updates
- [ ] **Payment Integration**: Stripe, PayPal, or other payment gateways
- [ ] **Order Tracking**: Real-time GPS tracking for delivery
- [ ] **Multi-language Support**: Internationalization (i18n)
- [ ] **Dark Mode**: Theme switching capability
- [ ] **Offline Support**: Offline order queuing and sync
- [ ] **Admin Dashboard**: Web admin panel for restaurant staff
- [ ] **Analytics Dashboard**: Business intelligence and reporting
- [ ] **Social Features**: Order sharing, reviews, ratings
- [ ] **Loyalty Program**: Points and rewards system

## Authentication Flow

### 1. User Registration

**Endpoint**: `POST /api/v1/auth/register`

**Important Notes**:
- Only works when the database is empty (first user gets ADMIN role)
- Public registration is disabled after first user
- Admin users can create additional users via user management endpoints

**Request Body**:
```json
{
  "username": "adminuser",
  "password": "adminpassword"
}
```

**Response**: 
- Success: `200 OK` with success message
- Error: `403 Forbidden` if public registration is closed

### 2. User Login

**Endpoint**: `POST /api/v1/auth/login`

**Request Body**:
```json
{
  "username": "adminuser", 
  "password": "adminpassword"
}
```

**Success Response** (`200 OK`):
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "message": "Login successful"
}
```

**Error Responses**:
- `401 Unauthorized`: Invalid credentials
- `500 Internal Server Error`: Database/server error

### 3. Token Refresh

**Endpoint**: `GET /api/v1/auth/refresh`

**Headers**:
```
Authorization: Bearer <your_jwt_token>
```

**Success Response** (`200 OK`):
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "message": "Token refreshed successfully"
}
```

**Purpose**: 
- Validates if the user is still valid in the system
- Issues a new 60-minute token
- Should be called before token expires or when app resumes

## JWT Token Details

### Token Structure
The JWT contains these claims:
- `sub`: User ID (integer)
- `iat`: Issued at timestamp
- `exp`: Expiration timestamp (60 minutes from issuance)
- `roles`: Array of role IDs for authorization

### Token Expiration
- **Duration**: 60 minutes
- **Refresh Strategy**: Call refresh endpoint before expiration
- **Storage**: Store securely in Android (SharedPreferences with encryption or Keystore)

## Core API Endpoints

### Health Check
- **GET** `/api` - Basic API health check

### Products
- **GET** `/api/v1/products` - Get all products
- **GET** `/api/v1/products/{id}` - Get product by ID
- **POST** `/api/v1/products` - Create product (requires auth)
- **PUT** `/api/v1/products/{id}` - Update product (requires auth)  
- **DELETE** `/api/v1/products/{id}` - Delete product (requires auth)

**Create Product Request**:
```json
{
  "name": "Burger Deluxe",
  "description": "Juicy beef burger with all the fixings",
  "price": "12.99",
  "product_image_uri": "https://example.com/images/burger.jpg"
}
```

**Product Response Format**:
```json
{
  "product_id": 1,
  "name": "Burger Deluxe", 
  "description": "Juicy beef burger with all the fixings",
  "price": "12.99",
  "product_image_uri": "https://example.com/images/burger.jpg"
}
```

### Orders
- **GET** `/api/v1/orders` - Get all orders (requires auth)
- **GET** `/api/v1/orders/{id}` - Get order by ID (requires auth)
- **GET** `/api/v1/orders/user/{username}` - Get orders by username (requires auth)
- **POST** `/api/v1/orders` - Create order

**Create Order Request**:
```json
{
  "products": [
    {
      "product_id": 1,
      "quantity": 2
    },
    {
      "product_id": 2, 
      "quantity": 1
    }
  ]
}
```

**Order Response Format**:
```json
{
  "order_id": 1,
  "user_id": 1,
  "product_id": 1,
  "quantity": 2,
  "total_amount": "25.50",
  "status": "Pending",
  "created_at": "2023-12-08T03:09:43",
  "updated_at": "2023-12-08T03:09:43"
}
```

**Order Status Values**:
- `"Pending"` - Initial order state
- `"Accepted"` - Order confirmed by restaurant
- `"Completed"` - Order fulfilled
- `"Cancelled"` - Order cancelled

### User Management (Admin Only)
- **GET** `/api/v1/users` - Get all users
- **GET** `/api/v1/users/{id}` - Get user by ID
- **GET** `/api/v1/users/search?username={username}` - Search users by username
- **POST** `/api/v1/users/create` - Add new user
- **PUT** `/api/v1/users/{id}` - Update user
- **DELETE** `/api/v1/users/{id}` - Delete user

### Role Management (Admin Only)
- **GET** `/api/v1/roles` - Get all roles
- **POST** `/api/v1/roles/create` - Create role
- **PUT** `/api/v1/roles/update/{id}` - Update role
- **DELETE** `/api/v1/roles/{id}` - Delete role
- **POST** `/api/v1/roles/{role_id}/set_permission` - Set role permissions
- **DELETE** `/api/v1/roles/{role_id}/delete_permission` - Remove role permissions
- **POST** `/api/v1/roles/assign` - Assign role to user

**Role Permission Types**:
- `"Read"` - Can view data
- `"Write"` - Can create/update data
- `"Admin"` - Full access to all operations

## Android Integration Guidelines

### 1. HTTP Client Setup (Kotlin)

Use **Retrofit** with **OkHttp** for networking:

```kotlin
// ApiService.kt
interface ApiService {
    @POST("auth/login")
    suspend fun login(@Body request: LoginRequest): LoginResponse
    
    @GET("auth/refresh") 
    suspend fun refreshToken(): LoginResponse
    
    @GET("products")
    suspend fun getProducts(): List<ProductResponse>
    
    @GET("products/{id}")
    suspend fun getProductById(@Path("id") productId: Int): ProductResponse
    
    @POST("orders")
    suspend fun createOrder(@Body request: CreateOrderRequest): OrderResponse
    
    @GET("orders")
    suspend fun getAllOrders(): List<OrderResponse>
    
    @GET("orders/{id}")
    suspend fun getOrderById(@Path("id") orderId: Int): OrderResponse
    
    @GET("orders/user/{username}")
    suspend fun getUserOrders(@Path("username") username: String): List<OrderResponse>
}

// Data Classes
data class LoginRequest(
    val username: String,
    val password: String
)

data class LoginResponse(
    val token: String,
    val message: String
)

data class CreateOrderRequest(
    val products: List<OrderItem>
)

data class OrderItem(
    val product_id: Int,
    val quantity: Int
)

data class OrderResponse(
    val order_id: Int,
    val user_id: Int,
    val product_id: Int,
    val quantity: Int,
    val total_amount: String, // BigDecimal serialized as String
    val status: String?,
    val created_at: String?,
    val updated_at: String?
)

data class ProductResponse(
    val product_id: Int,
    val name: String,
    val description: String?,
    val price: String, // BigDecimal serialized as String
    val product_image_uri: String?
)

// NetworkModule.kt
class NetworkModule {
    // Replace with your actual Azure URL
    private val baseUrl = "https://your-azure-app.azurewebsites.net/api/v1/"
    
    val apiService: ApiService by lazy {
        val client = OkHttpClient.Builder()
            .addInterceptor(AuthInterceptor())
            .addInterceptor(HttpLoggingInterceptor().apply {
                level = HttpLoggingInterceptor.Level.BODY
            })
            .build()
            
        Retrofit.Builder()
            .baseUrl(baseUrl)
            .client(client)
            .addConverterFactory(GsonConverterFactory.create())
            .build()
            .create(ApiService::class.java)
    }
}
```

### 2. Token Management

```kotlin
class TokenManager(private val context: Context) {
    private val prefs = context.getSharedPreferences("auth", Context.MODE_PRIVATE)
    
    fun saveToken(token: String) {
        prefs.edit().putString("jwt_token", token).apply()
    }
    
    fun getToken(): String? {
        return prefs.getString("jwt_token", null)
    }
    
    fun clearToken() {
        prefs.edit().remove("jwt_token").apply()
    }
    
    fun isTokenValid(): Boolean {
        val token = getToken() ?: return false
        // Decode JWT and check expiration (use JWT library)
        // Return true if token exists and not expired
        return true
    }
}
```

### 3. Authentication Interceptor

```kotlin
class AuthInterceptor : Interceptor {
    override fun intercept(chain: Interceptor.Chain): Response {
        val request = chain.request()
        val token = TokenManager.getInstance().getToken()
        
        val newRequest = if (token != null) {
            request.newBuilder()
                .addHeader("Authorization", "Bearer $token")
                .build()
        } else {
            request
        }
        
        return chain.proceed(newRequest)
    }
}
```

### 4. Login Flow

```kotlin
class AuthRepository {
    suspend fun login(username: String, password: String): Result<LoginResponse> {
        return try {
            val response = apiService.login(LoginRequest(username, password))
            TokenManager.getInstance().saveToken(response.token)
            Result.success(response)
        } catch (e: Exception) {
            Result.failure(e)
        }
    }
    
    suspend fun refreshToken(): Result<LoginResponse> {
        return try {
            val response = apiService.refreshToken()
            TokenManager.getInstance().saveToken(response.token)
            Result.success(response)
        } catch (e: Exception) {
            // If refresh fails, redirect to login
            TokenManager.getInstance().clearToken()
            Result.failure(e)
        }
    }
}
```

### 5. Order Creation Flow

```kotlin
class OrderRepository {
    suspend fun createOrder(products: List<OrderItem>): Result<OrderResponse> {
        return try {
            val request = CreateOrderRequest(products)
            val response = apiService.createOrder(request)
            Result.success(response)
        } catch (e: Exception) {
            Result.failure(e)
        }
    }
}

// Data classes
data class OrderItem(
    val product_id: Int,
    val quantity: Int
)

data class CreateOrderRequest(
    val products: List<OrderItem>
)
```

### 6. Token Refresh Strategy

```kotlin
class TokenRefreshManager {
    private val refreshInterval = 50 * 60 * 1000L // 50 minutes
    
    fun scheduleTokenRefresh() {
        CoroutineScope(Dispatchers.IO).launch {
            while (TokenManager.getInstance().isTokenValid()) {
                delay(refreshInterval)
                try {
                    authRepository.refreshToken()
                } catch (e: Exception) {
                    // Handle refresh failure - redirect to login
                    break
                }
            }
        }
    }
}
```

## Error Handling

### Common HTTP Status Codes
- **200 OK**: Success
- **400 Bad Request**: Invalid request format
- **401 Unauthorized**: Invalid/expired token or credentials
- **403 Forbidden**: Insufficient permissions
- **404 Not Found**: Resource not found
- **500 Internal Server Error**: Server/database error

### Recommended Error Handling Strategy
1. **401 Unauthorized**: Clear token and redirect to login
2. **403 Forbidden**: Show "Access denied" message
3. **5xx Errors**: Show "Server error, please try again"
4. **Network errors**: Check connectivity and retry

## Security Best Practices

1. **Token Storage**: Use Android Keystore for sensitive token storage
2. **HTTPS Only**: Always use HTTPS in production
3. **Certificate Pinning**: Consider implementing certificate pinning
4. **Token Validation**: Check token expiration before API calls
5. **Logout**: Always clear tokens on logout
6. **Background Refresh**: Refresh tokens in background before expiration

## Testing with Bruno Collection

Use the provided Bruno collection for API testing:
- Located in `/bruno` folder
- Contains all endpoint examples
- Use for API validation during development

## Customer-Side Transaction Flow

### Typical User Journey:
1. **App Launch**: Check for stored valid token
2. **Login/Register**: If no valid token, show login screen
3. **Browse Products**: Fetch and display available products
4. **Create Order**: Submit order with selected products
5. **View Orders**: Display user's order history
6. **Token Management**: Auto-refresh tokens in background

### Recommended App Architecture:
- **MVVM Pattern** with Repository pattern
- **Coroutines** for async operations
- **Room Database** for local caching
- **Retrofit** for network calls
- **Dependency Injection** (Hilt/Dagger)

This guide provides the foundation for integrating your Orderly Android app with the Arrow Server backend. The server handles authentication, order processing, and data persistence, while your app focuses on user experience and real-time interactions.

## Implementation Priority

### Phase 1: Core Functionality (Week 1-2)
1. Set up Android project with dependencies
2. Implement authentication (login/register/refresh)
3. Create basic product listing screen
4. Implement order creation functionality

### Phase 2: Enhanced Features (Week 3-4)
1. Add order history and status tracking
2. Implement proper error handling
3. Add offline support with local caching
4. Create user profile management

### Phase 3: Production Ready (Week 5-6)
1. Security hardening (certificate pinning, token encryption)
2. UI/UX polishing and animations
3. Comprehensive testing
4. Performance optimization

### Phase 4: Advanced Features (Future)
1. Push notifications for order updates
2. Payment gateway integration
3. Advanced analytics and reporting
4. Admin dashboard features

## Quick Start Checklist

### Before You Start:
- [ ] Confirm Arrow Server is deployed and accessible on Azure
- [ ] Obtain the actual Azure deployment URL
- [ ] Verify all API endpoints are working via Bruno collection
- [ ] Set up development environment (Android Studio, Git)

### Day 1 Setup:
- [ ] Create new Android project with minimum SDK 24+
- [ ] Add required dependencies to `build.gradle`
- [ ] Implement basic network layer with Retrofit
- [ ] Test authentication endpoints with actual server

This structured approach will help you build the Orderly app systematically while ensuring all critical components are properly implemented.