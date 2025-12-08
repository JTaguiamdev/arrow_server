# Android Backend Development Plan with Kotlin (Customer App)

This document outlines the plan for developing a **customer-facing Android application** using Kotlin that communicates with the existing Rust-based API. The primary focus is on product browsing, ordering, and user-specific functionalities. Administrative features will be excluded.

## 1. Project Setup

1.  **Create a new Android Studio Project:**
    *   Select "Empty Activity" template.
    *   Language: Kotlin.
    *   Minimum SDK: API 21 (or higher).

2.  **Add Dependencies:**
    *   **Retrofit:** For making HTTP requests to the API.
    *   **Gson/Moshi:** For JSON serialization and deserialization.
    *   **OkHttp Logging Interceptor:** For logging network requests and responses (useful for debugging).
    *   **Kotlin Coroutines:** For managing asynchronous operations.
    *   **ViewModel & LiveData/Flow:** For building the UI layer with a lifecycle-aware architecture.
    *   **Hilt/Koin:** For dependency injection.
    *   **EncryptedSharedPreferences:** For securely storing the authentication token.

    ```gradle
    // app/build.gradle.kts

    dependencies {
        // Retrofit & Networking
        implementation("com.squareup.retrofit2:retrofit:2.9.0")
        implementation("com.squareup.retrofit2:converter-gson:2.9.0") // Or converter-moshi
        implementation("com.squareup.okhttp3:logging-interceptor:4.11.0")

        // Coroutines
        implementation("org.jetbrains.kotlinx:kotlinx-coroutines-android:1.7.3")

        // ViewModel & LiveData
        implementation("androidx.lifecycle:lifecycle-viewmodel-ktx:2.7.0")
        implementation("androidx.lifecycle:lifecycle-livedata-ktx:2.7.0")

        // Hilt (for dependency injection)
        implementation("com.google.dagger:hilt-android:2.51.1")
        kapt("com.google.dagger:hilt-compiler:2.51.1")

        // Secure Storage
        implementation("androidx.security:security-crypto:1.1.0-alpha06")
    }
    ```

## 2. API Service Layer (Networking)

1.  **Create Data Transfer Objects (DTOs):**
    Create Kotlin data classes that match the JSON structure of the API's requests and responses for `User` (limited to customer view), `Product`, and `Order`.

    *Example (`UserDTO.kt`):*
    ```kotlin
    data class UserDTO(
        val userId: Int?, // May or may not be included for customer's own profile
        val username: String,
        val role: RoleDTO?, // Will likely be a "customer" role
        val createdAt: String?,
        val updatedAt: String?
    )
    // ... other DTOs for Login, Product, Order etc.
    ```

2.  **Create Retrofit API Interface:**
    Define a Retrofit interface with methods for the relevant API endpoints.

    ```kotlin
    interface ApiService {
        // User Authentication & Profile
        @POST("api/auth/register")
        suspend fun register(@Body newUser: NewUserDTO): Response<LoginResponse>

        @POST("api/auth/login")
        suspend fun login(@Body loginDto: LoginDTO): Response<LoginResponse>

        @GET("api/users/{userId}") // For fetching customer's own profile
        suspend fun getUserById(@Path("userId") userId: Int): Response<UserDTO>

        // Product Routes (Customer view)
        @GET("api/products")
        suspend fun getAllProducts(): Response<List<ProductResponse>>

        @GET("api/products/{productId}")
        suspend fun getProductById(@Path("productId") productId: Int): Response<ProductResponse>

        // Order Routes (Customer view)
        @POST("api/orders")
        suspend fun createOrder(@Body createOrderRequest: CreateOrderRequest): Response<String> // Returns "Orders created"

        @GET("api/orders/user/{username}") // To fetch orders for the logged-in user
        suspend fun getUserOrders(@Path("username") username: String): Response<List<OrderResponse>>

        @GET("api/orders/{orderId}")
        suspend fun getOrderById(@Path("orderId") orderId: Int): Response<OrderResponse>
    }
    ```

3.  **Setup Retrofit Client:**
    Configure a Retrofit instance with the base URL of the Rust server and add the necessary interceptors (logging, authentication).

## 3. Authentication (JWT)

1.  **Create an `AuthInterceptor`:**
    This OkHttp interceptor will automatically add the `Authorization: Bearer <token>` header to all outgoing requests (except for initial login/register).

2.  **Secure Token Storage:**
    Use `EncryptedSharedPreferences` to securely store the JWT received after a successful login.

3.  **Login/Logout Logic:**
    *   Implement a login screen that calls the `api/auth/login` endpoint. Upon success, save the received JWT token.
    *   Implement logout functionality that deletes the stored token.

## 4. Repository Layer

Create repository classes for each data model (`UserRepository`, `ProductRepository`, `OrderRepository`). These repositories will abstract the data source (the API) from the ViewModels.

*Example (`ProductRepository.kt`):*
```kotlin
class ProductRepository @Inject constructor(private val apiService: ApiService) {
    suspend fun getAllProducts(): Result<List<ProductResponse>> {
        return try {
            val response = apiService.getAllProducts()
            if (response.isSuccessful) {
                Result.success(response.body()!!)
            } else {
                Result.failure(Exception("Failed to fetch products: ${response.code()}"))
            }
        } catch (e: Exception) {
            Result.failure(e)
        }
    }

    suspend fun getProductById(productId: Int): Result<ProductResponse> {
        return try {
            val response = apiService.getProductById(productId)
            if (response.isSuccessful) {
                Result.success(response.body()!!)
            } else {
                Result.failure(Exception("Failed to fetch product by ID: ${response.code()}"))
            }
        } catch (e: Exception) {
            Result.failure(e)
        }
    }
}
```

## 5. ViewModel Layer

Create ViewModels that use the repositories to fetch and manage data. Expose the data to the UI using `LiveData` or `StateFlow`.

*Example (`ProductViewModel.kt`):*
```kotlin
@HiltViewModel
class ProductViewModel @Inject constructor(
    private val productRepository: ProductRepository
) : ViewModel() {
    private val _products = MutableLiveData<List<ProductResponse>>()
    val products: LiveData<List<ProductResponse>> = _products

    private val _selectedProduct = MutableLiveData<ProductResponse>()
    val selectedProduct: LiveData<ProductResponse> = _selectedProduct

    fun fetchProducts() {
        viewModelScope.launch {
            val result = productRepository.getAllProducts()
            result.onSuccess { _products.postValue(it) }
                  .onFailure { /* Handle error, e.g., post error message to a LiveData */ }
        }
    }

    fun fetchProductDetails(productId: Int) {
        viewModelScope.launch {
            val result = productRepository.getProductById(productId)
            result.onSuccess { _selectedProduct.postValue(it) }
                  .onFailure { /* Handle error */ }
        }
    }
}
```

## 6. UI Layer (Views)

Create Activities and Fragments to display the data from the ViewModels and to handle user input.

*   **LoginActivity:** For user authentication (obtaining the JWT).
*   **ProductListActivity/Fragment:** Displays a list of available products.
*   **ProductDetailActivity/Fragment:** Shows details of a selected product, including options to add to cart/order.
*   **OrderConfirmationActivity/Fragment:** Summarizes the order and allows placing it.
*   **MyOrdersActivity/Fragment:** Displays a list of the customer's past orders.
*   **UserProfileActivity/Fragment:** Allows viewing the customer's own profile details.

## 7. Error Handling

Implement a centralized error handling mechanism. This can be done in the repository or ViewModel layer to catch exceptions from API calls and display appropriate messages to the user (e.g., using Toast messages or AlertDialogs).

## 8. Task Breakdown (Customer App Focus)

-   [ ] **Phase 1: Project Setup & User Authentication**
    -   [ ] Setup Gradle dependencies for networking, coroutines, DI, and secure storage.
    -   [ ] Implement DTOs for `Login`, `Register`, and `User` (customer-specific fields).
    -   [ ] Implement API service methods for `POST /api/auth/register` and `POST /api/auth/login`.
    -   [ ] Implement secure token storage using `EncryptedSharedPreferences`.
    -   [ ] Create Login/Registration UI and ViewModel to handle authentication flow.
    -   [ ] Implement `AuthInterceptor` to attach JWT to outgoing requests.
    -   [ ] Implement a mechanism to check login status on app launch and redirect accordingly.

-   [ ] **Phase 2: Product Browsing**
    -   [ ] Implement DTOs for `ProductResponse`.
    -   [ ] Add API service methods for `GET /api/products` and `GET /api/products/{productId}`.
    -   [ ] Create `ProductRepository`.
    -   [ ] Create `ProductViewModel`.
    -   [ ] Create UI (e.g., `ProductListActivity/Fragment`) to display all products.
    -   [ ] Create UI (e.g., `ProductDetailActivity/Fragment`) to display individual product details.

-   [ ] **Phase 3: Order Management**
    -   [ ] Implement DTOs for `CreateOrderRequest` and `OrderResponse`.
    -   [ ] Add API service methods for `POST /api/orders` (to create an order) and `GET /api/orders/user/{username}` (to view customer's own orders).
    -   [ ] Create `OrderRepository`.
    -   [ ] Create `OrderViewModel`.
    -   [ ] Integrate product selection into an order creation flow (e.g., a "cart" or direct "buy now").
    -   [ ] Create UI for order confirmation.
    -   [ ] Create UI (e.g., `MyOrdersActivity/Fragment`) to display the customer's past orders.

-   [ ] **Phase 4: User Profile (Viewing only)**
    -   [ ] Add API service method for `GET /api/users/{userId}` to fetch the logged-in user's profile.
    -   [ ] Update `UserRepository` and `UserViewModel` to support fetching the current user's profile.
    -   [ ] Create UI for viewing the user's profile.